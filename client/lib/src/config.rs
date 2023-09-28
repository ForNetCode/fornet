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
use anyhow::Context;
use cfg_if::cfg_if;
use base64::Engine;
use crate::protobuf::auth::EncryptRequest;

const SERVER_SAVE_NAME: &str = "config.json";

pub struct AppConfig {
    pub config_path: PathBuf,
    pub identity: Identity,
    pub local_config: LocalConfig,
    #[cfg(target_os="windows")]
    pub driver_path: String
}
impl AppConfig {

    fn _load_config(config_path:&PathBuf) -> anyhow::Result<(Identity, LocalConfig)>{
        let identity = if Identity::exists(config_path) {
            Identity::read_from_file(config_path)?
        } else {
            fs::create_dir(config_path).with_context(|| format!("create config path: {:?} failure", config_path))?;
            let identity = Identity::new();
            identity.save(config_path)?;
            identity
        };
        let server_config = if LocalConfig::exits(config_path) {
            LocalConfig::read_from_file(config_path)?
        } else {
            let server_config = LocalConfig::new();
            server_config.save_config(config_path)?;
            server_config
        };
        Ok((identity, server_config))
    }

    #[cfg(target_os = "windows")]
    pub fn load_config(config_path: &PathBuf, driver_path:String) -> anyhow::Result<Self> {
        let (identity,local_config) = Self::_load_config(config_path)?;
        Ok(Self {
            config_path: config_path.clone(),
            identity,
            local_config,
            driver_path,
        })
    }
    #[cfg(not(target_os = "windows"))]
    pub fn load_config(config_path: &PathBuf) -> anyhow::Result<Self> {
        let (identity,local_config) = Self::_load_config(config_path)?;
        Ok(Self {
            config_path: config_path.clone(),
            identity,
            local_config,
        })
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
            pk_base64: base64::engine::general_purpose::STANDARD.encode(
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
            pk_base64: base64::engine::general_purpose::STANDARD.encode(public_key),
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
        let public_key = base64::engine::general_purpose::STANDARD.decode(base64_str)?;
        let x25519_pk = x25519_dalek::PublicKey::from(array_ref![public_key, 0, 32].clone());
        let ed25519_pk = ed25519_compact::PublicKey::new(array_ref![public_key, 32, 32].clone());
        Ok((x25519_pk, ed25519_pk))
    }

    pub fn sign(&self, params:Vec<String>) -> anyhow::Result<EncryptRequest> {
        let mut raw = vec![0; 16];
        OsRng.fill_bytes(&mut raw);

        let nonce = base64::engine::general_purpose::STANDARD.encode(&raw);
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
        let signature =base64::engine::general_purpose::STANDARD.encode(*signature);
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
            base64::engine::general_purpose::STANDARD.encode(self.x25519_sk.to_bytes()),
            base64::engine::general_purpose::STANDARD.encode(self.x25519_pk.to_bytes()),
            base64::engine::general_purpose::STANDARD.encode(self.ed25519_sk.deref()),
            base64::engine::general_purpose::STANDARD.encode(self.ed25519_pk.deref()),
        )
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkInfo {
    pub network_id: String,
    pub tun_name: Option<String>,

}

impl NetworkInfo {
    pub fn new(network_id:String) -> Self {
        Self {
            network_id,
            tun_name: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerInfo {
    pub server_url: String,
    pub device_id: String,
    pub mqtt_url: String,
    pub network_id: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServerConfig {
    pub server_url: String,
    pub device_id: String,
    pub mqtt_url: String,
    //networkId, mqttUrl, clientId
    pub info: Vec<NetworkInfo>
}


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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LocalConfig {
    pub server_info: Vec<ServerInfo>,
    pub tun_name: Option<String>,
}

impl LocalConfig {
    pub fn new () -> Self {
        cfg_if! {
            if #[cfg(target_os = "linux")] {
                let tun_name = Some("for0".to_owned());
            } else if #[cfg(target_os = "windows")] {
                let tun_name = Some(format!("{:?}", windows::core::GUID::new().unwrap()));
            }else {
                // mac,android
                let tun_name = None;
            }
        }
        Self{
            server_info:vec![],
            tun_name,
        }

    }
    pub fn exits(config_dir: &PathBuf) -> bool {
        Self::config_file_path(config_dir).exists()
    }

    pub fn config_file_path(config_dir:&PathBuf) -> PathBuf{
        config_dir.join(SERVER_SAVE_NAME)
    }

    pub fn save_config(&self, config_dir: &PathBuf) -> anyhow::Result<()> {
        let path = config_dir.join(SERVER_SAVE_NAME);
        Ok(fs::write(path, serde_json::to_string_pretty(&self)?)?) //.unwrap_or_else(|_| panic!("write config file error:{:?}", &config));
    }

    pub fn read_from_file(config_dir: &PathBuf) -> anyhow::Result<LocalConfig> {
        let config_str = fs::read_to_string(config_dir.join(SERVER_SAVE_NAME))?;
        Ok(serde_json::from_str::<LocalConfig>(&config_str)?)
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
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct A(Vec<String>);
    #[derive(Deserialize, Serialize)]
    struct B{
        pub a:A,
    }

    #[test]
    fn JsonParse() {
        let b = B{a: A(vec!["abc".to_owned()])};
        let z = serde_json::to_string_pretty(&b).unwrap();
        println!("{}", z);
    }

}
