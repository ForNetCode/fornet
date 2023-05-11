use arrayref::array_ref;
use ed25519_compact::Seed;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use cfg_if::cfg_if;
//should not reserve
use  std::collections::HashMap;
use crate::protobuf::auth::EncryptRequest;

#[cfg(target_os="windows")]
#[derive(Deserialize, Serialize, Debug)]
pub struct WindowsClientConfig {
    //key: public_key, value: windows tun device guid
    pub tun_guid: HashMap<String, String>
}
#[cfg(target_os="windows")]
impl WindowsClientConfig {
    pub fn load(config_dir:&PathBuf, identity:&Identity)->anyhow::Result<Self> {
        let pub_key = identity.pk_base64.clone();
        if Self::exists(config_dir) {
            let mut config = Self::read_from_file(config_dir)?;
            if config.tun_guid.contains_key(&pub_key) {
                Ok(config)
            } else {
                config.tun_guid.insert(pub_key, format!("{:?}", windows::core::GUID::new()?));
                config.save_config(config_dir)?;
                Ok(config)
            }
        } else {
            let config = WindowsClientConfig {
                tun_guid: HashMap::from([
                    (pub_key,format!("{:?}", windows::core::GUID::new()?))
                ])
            };
            config.save_config(config_dir)?;
            Ok(config)
        }
    }
    pub fn exists(config_dir:&PathBuf)->bool {
        config_dir.join("windows_client.json").exists()
    }
    pub fn read_from_file(config_dir:&PathBuf) -> anyhow::Result<Self>{
        let config_str = fs::read_to_string(config_dir.join("windows_client.json"))?;
        Ok(serde_json::from_str::<WindowsClientConfig>(&config_str)?)
    }

    pub fn save_config(&self, config_dir: &PathBuf) -> anyhow::Result<()> {
        let path = config_dir.join("windows_client.json");
        Ok(fs::write(path, serde_json::to_string_pretty(self)?)?)
    }
}

pub struct Config {
    pub config_path: PathBuf,
    pub server_config: ServerConfig,
    pub identity: Identity,
    #[cfg(target_os = "windows")]
    pub client_config: WindowsClientConfig
}

impl Config {
    pub fn load_config(config_path: &PathBuf) -> anyhow::Result<Option<Config>> {
        if !ServerConfig::exits(config_path) {
            return Ok(None)
        }
        let server_config = ServerConfig::read_from_file(&config_path)?;
        let identity = Identity::read_from_file(&config_path)?;

        #[cfg(target_os = "windows")]
        let client_config = WindowsClientConfig::load(&config_path, &identity)?;
        Ok(Some(Config {
            config_path: config_path.clone(),
            server_config,
            identity,
            #[cfg(target_os = "windows")]
            client_config,
        }))
    }
    cfg_if! {
        if #[cfg(target_os = "windows")] {
            pub fn get_tun_name(&self) -> String {
                //  This must be have
                self.client_config.tun_guid.get(&self.identity.pk_base64).unwrap().clone()
            }
        } else {
            pub fn get_tun_name(&self) -> String {
                "for0".to_owned()
            }
        }
    }
}

pub struct Identity {
    pub x25519_pk: x25519_dalek::PublicKey,
    pub x25519_sk: x25519_dalek::StaticSecret,
    pub ed25519_pk: ed25519_compact::PublicKey,
    pub ed25519_sk: ed25519_compact::SecretKey,
    pub pk_base64: String,
}

const IDENTITY_PUB_FILE_NAME: &str = "id_curve25519.pub";
const IDENTITY_PRIVATE_FILE_NAME: &str = "id_curve25519.key";

impl Identity {
    pub fn new() -> Identity {
        let x25519_secret_key = x25519_dalek::StaticSecret::new(&mut OsRng {});
        let x25519_public_key = x25519_dalek::PublicKey::from(&x25519_secret_key);
        let ed25519_keypair = ed25519_compact::KeyPair::from_seed(Seed::default());

        Identity {
            pk_base64: base64::encode(
                [
                    x25519_public_key.to_bytes(),
                    ed25519_keypair.pk.deref().clone(),
                ]
                .concat(),
            ),
            x25519_pk: x25519_public_key,
            x25519_sk: x25519_secret_key,
            ed25519_sk: ed25519_keypair.sk,
            ed25519_pk: ed25519_keypair.pk,
        }
    }

    pub fn exists(config_dir: &PathBuf) -> bool {
        config_dir.join(IDENTITY_PRIVATE_FILE_NAME).exists()
    }

    pub fn read_from_file(config_dir: &PathBuf) -> anyhow::Result<Identity> {
        let public_key_path = config_dir.join(IDENTITY_PUB_FILE_NAME);
        let private_key_path = config_dir.join(IDENTITY_PRIVATE_FILE_NAME);

        let public_key = fs::read(public_key_path)?;
        let private_key = fs::read(private_key_path)?;

        let x25519_pk = x25519_dalek::PublicKey::from(array_ref![public_key, 0, 32].clone());
        let x25519_sk = x25519_dalek::StaticSecret::from(array_ref![private_key, 0, 32].clone());

        let ed25519_pk = ed25519_compact::PublicKey::new(array_ref![public_key, 32, 32].clone());

        let mut ed25519_sk = [0u8; 64];
        let (seed, public) = ed25519_sk.split_at_mut(32);
        seed.copy_from_slice(array_ref![private_key, 32, 32]);
        public.copy_from_slice(array_ref![public_key, 32, 32]);
        let ed25519_sk = ed25519_compact::SecretKey::new(ed25519_sk);

        Ok(Identity {
            x25519_pk,
            x25519_sk,
            ed25519_sk,
            pk_base64: base64::encode(public_key),
            ed25519_pk,
        })
    }

    pub fn save(&self, config_dir: &PathBuf) -> anyhow::Result<()> {
        let public_key_path = config_dir.join(IDENTITY_PUB_FILE_NAME);
        let private_key_path = config_dir.join(IDENTITY_PRIVATE_FILE_NAME);

        let public_key = [self.x25519_pk.to_bytes(), self.ed25519_pk.deref().clone()].concat();
        let private_key = [
            self.x25519_sk.to_bytes(),
            array_ref![self.ed25519_sk.deref(), 0, 32].clone(),
        ]
        .concat();

        fs::write(public_key_path, public_key)?;
        fs::write(private_key_path, private_key)?;
        Ok(())
    }

    pub fn get_pub_identity_from_base64(
        base64_str: &str,
    ) -> anyhow::Result<(x25519_dalek::PublicKey, ed25519_compact::PublicKey)> {
        let public_key = base64::decode(base64_str)?;
        let x25519_pk = x25519_dalek::PublicKey::from(array_ref![public_key, 0, 32].clone());
        let ed25519_pk = ed25519_compact::PublicKey::new(array_ref![public_key, 32, 32].clone());
        Ok((x25519_pk, ed25519_pk))
    }
    pub fn sign(&self, network_id: &str) -> anyhow::Result<GRPCAuth> {
        let mut raw = vec![0; 16];
        OsRng.fill_bytes(&mut raw);
        let nonce = base64::encode_config(raw, base64::STANDARD);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let plain_text = format!("{}-{}-{}", timestamp, network_id, nonce);
        let signature = self.ed25519_sk.sign(plain_text, None);
        let signature = base64::encode(*signature);
        Ok(GRPCAuth {
            timestamp,
            network_id: network_id.to_owned(),
            nonce,
            sign: signature,
            public_key: self.pk_base64.clone(),
        })
    }
    pub fn sign2(&self, params:Vec<String>) -> anyhow::Result<EncryptRequest> {
        let mut raw = vec![0; 16];
        OsRng.fill_bytes(&mut raw);
        let nonce = base64::encode_config(raw, base64::STANDARD);
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let plain_text = if !params.is_empty() {
            let mut plain_text = params.join("|");
            plain_text.push_str( &format!("|{}|{}",nonce, timestamp));
            plain_text
        } else {
            format!("{}|{}",nonce, timestamp)
        };
        //tracing::debug!("plain_text: {plain_text}");
        let signature = self.ed25519_sk.sign(plain_text, None);
        let signature = base64::encode(*signature);
        Ok(EncryptRequest {
            public_key: self.pk_base64.clone(),
            timestamp,
            nonce,
            signature,
        })
    }
}

impl Debug for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Identity(x_sk:{}, x_pk:{}, ed_sk:{}, ed_pk:{})",
            base64::encode(self.x25519_sk.to_bytes()),
            base64::encode(self.x25519_pk.to_bytes()),
            base64::encode(self.ed25519_sk.deref()),
            base64::encode(self.ed25519_pk.deref()),
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerConfig {
    pub server: String,
    pub mqtt: HashMap<String,String>
}

/* impl default for serverconfig {
    fn default() -> self {
        serverconfig {
            server: "http://127.0.0.1:9000".to_owned(),
            network_id: "".to_owned(),
        }
    }
}
 */
const SERVER_SAVE_NAME: &str = "config.json";

impl ServerConfig {
    pub fn exits(config_dir: &PathBuf) -> bool {
        Self::config_file_path(config_dir).exists()
    }

    pub fn config_file_path(config_dir:&PathBuf) -> PathBuf{
        config_dir.join(SERVER_SAVE_NAME)
    }

    pub fn save_config(&self, config_dir: &PathBuf) -> anyhow::Result<()> {
        let path = config_dir.join(SERVER_SAVE_NAME);
        Ok(fs::write(path, serde_json::to_string_pretty(self)?)?) //.unwrap_or_else(|_| panic!("write config file error:{:?}", &config));
    }

    pub fn read_from_file(config_dir: &PathBuf) -> anyhow::Result<ServerConfig> {
        let config_str = fs::read_to_string(config_dir.join(SERVER_SAVE_NAME))?;
        Ok(serde_json::from_str::<ServerConfig>(&config_str)?)
    }

}

// add lifetime to reduce copy?
// or add ARC to public_key
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GRPCAuth {
    pub timestamp: u64,
    pub network_id: String,
    pub nonce: String,
    pub sign: String,
    pub public_key: String,
}
#[cfg(test)]
mod tests {
    use crate::config::Identity;
    use arrayref::array_ref;
    use std::ops::Deref;

    #[test]
    fn identity_combine() {
        let mut identity = Identity::new();

        let x25519_pk: [u8; 32] = base64::decode("lpgpJqleWa1zqrk/O/jRThnqK1dGDzogKKicoefQrFs=")
            .unwrap()
            .try_into()
            .unwrap();
        let x25519_sk: [u8; 32] = base64::decode("6LI166lIZJmAxMWzTf/r/KyIjKJXXFsry3Z0XDIRbHo=")
            .unwrap()
            .try_into()
            .unwrap();
        let x25519_pk = x25519_dalek::PublicKey::from(x25519_pk);
        let x25519_sk = x25519_dalek::StaticSecret::from(x25519_sk);
        identity.x25519_sk = x25519_sk;
        identity.x25519_pk = x25519_pk;

        let public_key = base64::encode(
            [
                identity.x25519_pk.to_bytes(),
                identity.ed25519_pk.deref().clone(),
            ]
            .concat(),
        );
        let private_key = base64::encode(
            [
                identity.x25519_sk.to_bytes(),
                array_ref![identity.ed25519_sk.deref(), 0, 32].clone(),
            ]
            .concat(),
        );
        println!("{}", public_key);
        println!("{}", private_key);
    }
}
