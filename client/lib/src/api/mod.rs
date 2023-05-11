use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use anyhow::{anyhow, bail};
use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::Sender;
use crate::config::{Config, Identity, ServerConfig};
use crate::sc_manager::SCManager;
use crate::protobuf::auth::{auth_client::AuthClient, InviteConfirmRequest, OAuthDeviceCodeRequest, SsoLoginInfoRequest};
use crate::server_api::APISocket;
use crate::server_manager::{ServerManager, ServerMessage};
use std::time::Duration;
use auto_launch_extra::AutoLaunchBuilder;
use cfg_if::cfg_if;
use tonic::{
    transport::Channel,
    Request,
};
use crate::{APP_NAME, MAC_OS_PACKAGE_NAME};

pub mod command_api;


async fn join_network(server_manager: &mut ServerManager, invite_code: &str, stream: &mut APISocket, tx: Sender<ServerMessage>) -> anyhow::Result<()> {
    let config_dir = PathBuf::from(&server_manager.config_path);
    if ServerConfig::exits(&PathBuf::from(&server_manager.config_path)) {
        bail!("config file already exists, it now do not support join multiple network");
    }
    let data = String::from_utf8(base64::decode(invite_code)?)?;
    let data: Vec<&str> = data.split('|').collect();
    let version = data[0].parse::<u32>()?;

    let identity = if !Identity::exists(&config_dir) {
        let identity = Identity::new();
        if let Ok(_) = identity.save(&config_dir) {
            identity
        } else {
            bail!("write identity file error")
        }
    } else {
        if let Ok(identity) = Identity::read_from_file(&config_dir) {
            identity
        } else {
            bail!("read identity file error")

        }
    };
    let change_config_and_init_sync_manger=  || {
        let config = Config::load_config(&config_dir).unwrap().unwrap();
        let config = Arc::new(config);
        server_manager.config = Some(config.clone());
        let mut sc_manager = SCManager::new(tx);
        let _ = tokio::spawn(async move {
            match sc_manager.mqtt_connect(config).await {
                Ok(()) => tracing::warn!("sync config manager close, now can not receive any update from server"),
                Err(e) => tracing::error!("sync config manager connect server result:{:?}", e),
            };
        });
    };
    if version == 1u32 {
        let invite_token = InviteToken::new(data);
        match server_invite_confirm(
            identity,
            &invite_token.endpoint,
            &invite_token.network_id,
            invite_token.node_id,
        )
            .await
        {
            Ok(mqtt_url) => {
                let mut map = HashMap::new();
                map.insert(invite_token.network_id, mqtt_url);

                //This must be success
                let server_config = ServerConfig {
                    server: invite_token.endpoint,
                    mqtt: map,
                };
                server_config.save_config(&config_dir)?;
                change_config_and_init_sync_manger();
            }
            Err(e) => {
                tracing::warn!("connect server error!, {e}");
                bail!("connect server error!, {e}")
            }
        };
    } else if version == 2u32 { // keycloak login
        let (mut client,sso_login) = SSOLogin::get_login_info(data).await?;
        match handle_oauth(identity, &mut client, &sso_login, stream).await {
            Ok(mqtt_url) => {
                let mut map = HashMap::new();
                map.insert(sso_login.network_id, mqtt_url);
                let server_config = ServerConfig {
                    server: sso_login.endpoint,
                    mqtt: map,
                };
                server_config.save_config(&config_dir)?;
                change_config_and_init_sync_manger();
            }
            Err(e) => {
                tracing::warn!("handle oauth error:{e}");
                bail!("Oauth error");
            }
        }
    } else {
        bail!("can not parse invite token, please upgrade");
    }
    Ok(())

}
pub async fn api_handler(server_manager: &mut ServerManager, command: String, stream: &mut APISocket, tx: Sender<ServerMessage>) {
    let command:Vec<&str> = command.split(' ').collect::<Vec<&str>>();
    match command[0] {
        "join" => {
            match join_network(server_manager, command[1], stream, tx).await {
                Ok(()) => {
                    let _ = stream.write(api_success("join success".to_owned()).to_json().as_bytes()).await;
                }
                Err(e) => {
                    let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
                }
            }
        }
        "list" => {
            if server_manager.wr_manager.is_alive() {
                let data = server_manager.wr_manager.device_info();
                let _ = stream.write(ApiResponse::boxed(data).to_json().as_bytes()).await;
            }
        }
        "autoLaunch" => {
            cfg_if! {
                if #[cfg(target_os="macos")] {
                    let _ = auto_launch_config(command, stream).await;
                } else {
                    let _ = stream.write(api_error("do not support".to_owned()).to_json().as_bytes()).await;
                }
            }
        }
        _ => {
            tracing::error!("unknown command");
            let _ = stream.write(b"unknown command").await;
        }
    }
}

#[cfg(target_os="macos")]
async fn auto_launch_config(command:Vec<&str>,  stream: &mut APISocket) {
    match env::current_dir()  {
        Ok(x) => {
            let app_path = x.join(APP_NAME);
            let auto = crate::device::auto_launch::AutoLaunch::new( MAC_OS_PACKAGE_NAME.to_owned(), app_path.to_str().unwrap().to_owned());

            tracing::debug!("app name:{APP_NAME}, app path: {:?}", app_path);
            let is_enabled = auto.is_enabled();
            let result = match command.get(1) {
                Some(&"enable") => {
                    (if is_enabled.is_err() {
                        Err(is_enabled.err().unwrap())
                    } else if is_enabled.unwrap_or(false) {
                        Ok(())
                    } else {
                        auto.enable()
                    }).map(|_| {
                        api_success("enable auto launch success".to_owned())
                    })
                }
                Some(&"disable") => {
                    (if is_enabled.is_err() {
                        Err(is_enabled.err().unwrap())
                    } else if !is_enabled.unwrap_or(false) {
                        Ok(())
                    } else {
                        auto.disable()
                    }).map(|_| {
                        api_success("disable auto launch success".to_owned())
                    })
                }
                _ => {
                    (if is_enabled.is_err() {
                        Err(is_enabled.err().unwrap())
                    } else {
                        Ok(is_enabled.unwrap_or(false))
                    }).map(|x| {
                        api_success(format!("{APP_NAME} auto launch: {}", if x { "enabled" } else { "disabled" }))
                    })
                }
            };
            match result {
                Ok(response) => {
                    let _ = stream.write(response.to_json().as_bytes()).await;
                }
                Err(e) => {
                    let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
                }}
        }
        Err(e) => {
            let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
        }
    }
}

#[derive(Serialize, Deserialize)]
struct OAuthDevice {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Serialize, Deserialize)]
struct OAuthDeviceJWToken {
    access_token: String,
    expires_in: u32,
    //...
}

// https://github.com/keycloak/keycloak-community/blob/main/design/oauth2-device-authorization-grant.md
async fn handle_oauth(identity: Identity, client:&mut AuthClient<Channel>, sso_login: &SSOLogin, stream: &mut APISocket) -> anyhow::Result<String> {

    let network_id = sso_login.network_id.clone();

    let response = reqwest::Client::new().post(format!("{}/realms/{}/protocol/openid-connect/auth/device", &sso_login.sso_url, &sso_login.realm))
        .form(&[("client_id", &sso_login.client_id)])
        .send().await?.json::<OAuthDevice>().await?;

    let message = api_success(format!(
        "please open browser with {} to login, and type in device code: {}",
        &response.verification_uri_complete, &response.device_code
    )).to_json();
    stream.write(message.as_bytes()).await?;

    let mut max_retry = response.expires_in / (response.interval+1) -1;

    while max_retry > 0 {
        max_retry -= 1;
        tokio::time::sleep(Duration::from_secs((response.interval+1) as u64)).await;

        let loop_response = reqwest::Client::new().post(format!("{}/realms/{}/protocol/openid-connect/token", &sso_login.sso_url, &sso_login.realm))
            .form(&[("grant_type","urn:ietf:params:oauth:grant-type:device_code"), ("client_id", &sso_login.client_id), ("device_code", &response.device_code)])
            .send().await?;
        if loop_response.status().is_success() {
            let loop_response = loop_response.json::<OAuthDeviceJWToken>().await?;
            //Seq(request.accessToken, request.deviceCode, request.networkId)
            let encrypt = identity.sign2(vec![loop_response.access_token.clone(), response.device_code.clone(), sso_login.network_id.clone()])?;
            let request = Request::new(OAuthDeviceCodeRequest {
                device_code: (&response.device_code).clone(),
                access_token: loop_response.access_token,
                network_id,
                encrypt:Some(encrypt),
            });
            let response= client.oauth_device_code_confirm(request).await?.into_inner();
            return if response.is_ok {
                Ok(response.mqtt_url.unwrap().clone())
            } else {
                Err(anyhow!(response.message.unwrap()))
            }
        } else {
            tracing::debug!("check login status: not login, will try to check after {} seconds...", response.interval + 1);
        }
    }
    return Err(anyhow!("this login cost more time than expected, please try again"));
}


async fn server_invite_confirm(
    identity: Identity,
    endpoint: &String,
    network_id: &String,
    node_id: Option<String>,
) -> anyhow::Result<String> {
    tracing::debug!("endpoint: {endpoint}");
    let channel = Channel::from_shared(endpoint.clone())?.connect().await?;
    let mut client = AuthClient::new(channel);


    let mut params = vec!(network_id.to_owned());
    if let Some(ref node_id) = node_id {
        params.push(node_id.clone());
    }
    let encrypt = identity.sign2(params)?;
    tracing::debug!("encrypt: {encrypt:?}");

    let request = Request::new(InviteConfirmRequest {
        node_id,
        network_id: network_id.clone(),
        encrypt: Some(encrypt),
    });
    let response = client.invite_confirm(request).await?;

    let response = response.into_inner();
    if response.is_ok {
        Ok(response.mqtt_url.unwrap().clone())
    } else {
        Err(anyhow!(response.message.unwrap()))
    }
}

struct InviteToken {
    endpoint: String,
    network_id: String,
    node_id: Option<String>,
}

impl InviteToken {
    fn new(data: Vec<&str>) -> Self {
        let endpoint = data[1].to_owned();
        let network_id = data[2].to_owned();
        let node_id = if data.len() > 3 {
            Some(data[3].to_owned())
        } else {
            None
        };
        InviteToken {
            endpoint,
            network_id,
            node_id,
        }
    }
}

struct SSOLogin {
    endpoint: String,
    network_id: String,
    sso_url: String,
    realm: String,
    client_id: String,
}

impl SSOLogin {
    async fn get_login_info(data: Vec<&str>) -> anyhow::Result<(AuthClient<Channel>, SSOLogin)> {
        let grpc_endpoint = data[1].to_owned();
        let network_id = data[2].to_owned();

        let channel = Channel::from_shared(grpc_endpoint.clone())?.connect().await?;
        let mut client = AuthClient::new(channel);
        let request = Request::new(SsoLoginInfoRequest {
            network_id: network_id.clone(),
        });
        let response = client.get_sso_login_info(request).await?;
        let response = response.into_inner();
        //let response = reqwest::get(format!("{}/api/auth/oauth/device_code?n_id={}", &grpc_endpoint, &network_id)).await?;
        Ok((client, SSOLogin {
            endpoint: grpc_endpoint,
            network_id,
            sso_url: response.sso_url,
            realm: response.realm,
            client_id: response.client_id,
        }))

    }
}

pub fn invite_token_parse(data: &str) -> anyhow::Result<(u32, String, String, Option<String>)> {
    let data = String::from_utf8(base64::decode(data)?)?;
    let data: Vec<&str> = data.split('|').collect();
    let version = data[0].parse::<u32>()?;
    if version == 1u32 || version == 2u32 {
        let endpoint = data[1].to_owned();
        let network_id = data[2].to_owned();
        let node_id = if data.len() > 3 {
            Some(data[3].to_owned())
        } else {
            None
        };
        Ok((version, endpoint, network_id, node_id))
    } else {
        //tracing::warn!("can not parse invite token, please upgrade");
        panic!("can not parse invite token, please upgrade");
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<B:Sized> {
    pub ok: bool,
    pub data: B,
}
impl <B>ApiResponse<B> {
    pub fn boxed(data:B) -> Box<ApiResponse<B>> {
        Box::new(ApiResponse {
            ok: true,
            data,
        })
    }
}

pub trait ApiJsonResponse {
    fn to_json(&self) -> String;
}

impl <'a, B:serde::Serialize + serde::Deserialize<'a>> ApiJsonResponse for ApiResponse<B> {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn api_error(data:String) -> ApiResponse<String> {
    ApiResponse {
        ok: false,
        data,
    }
}
pub fn api_success(data:String) -> ApiResponse<String> {
    ApiResponse {
        ok: true,
        data,
    }
}
pub fn api_box_error(data:String) -> Box<dyn ApiJsonResponse> {
    Box::new(api_error(data))
}


#[cfg(test)]
mod test {
    use std::result;
    use serde_json::json;
    use crate::api::{api_error, ApiJsonResponse, ApiResponse};
    use crate::wr_manager::DeviceInfoResp;

    #[test]
    fn test_serialize_deserialize() {
        let info = api_error("test".to_string());
        assert_eq!("{\"ok\":false,\"data\":\"test\"}",serde_json::to_string(&info).unwrap());
    }

    fn serialize_deserialize(success:bool) -> Box<dyn ApiJsonResponse> {
        if success {
            let info = ApiResponse {
                ok: true,
                data: DeviceInfoResp{
                    name: "123".to_owned(),
                }
            };
            return Box::new(info);
        } else {
            let info = api_error("test".to_string());
            return Box::new(info);
        }
    }

    #[test]
    fn test_serialize_deserialize_2() {
        let result = serialize_deserialize(false);
        println!("{}",result.to_json());
    }

}