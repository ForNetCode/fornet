use cfg_if::cfg_if;
#[cfg(not(target_os = "android"))]
pub mod file_socket_api_server;
pub(crate) mod flutter_ffi;

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        pub use self::unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use self::windows::*;
    }
}

use std::path::PathBuf;
use anyhow::anyhow;
use tokio::io::AsyncWriteExt;
use crate::config::{Identity, ServerInfo};
use crate::protobuf::auth::{auth_client::AuthClient, InviteConfirmRequest, OAuthDeviceCodeRequest, SsoLoginInfoRequest, SuccessResponse};
use std::time::Duration;
use base64::Engine;
use serde_derive::{Deserialize, Serialize};

use tonic::{
    transport::Channel,
    Request,
};
use crate::protobuf::auth::action_response::Response;




#[derive(Debug)]
pub struct ApiClient {
    client: _ApiClient
}
impl ApiClient {
    pub fn new(path:PathBuf) -> Self{
        Self {
            client: _ApiClient::new(path)
        }
    }
    pub async fn join_network(&self, invite_code:&str)->anyhow::Result<StreamResponse> {
        self.client.send_command_stream(&format!("join {}", invite_code)).await
    }

    pub async fn list_network(&self) -> anyhow::Result<String> {
        self.client.send_command( "list").await
    }

    pub async fn auto_launch(&self, sub_command:&str) -> anyhow::Result<String> {
        self.client.send_command(&format!("autoLaunch {sub_command}")).await
    }

    // pub fn version(&self) -> String {
    //     env!("CARGO_PKG_VERSION").to_owned()
    // }
}





#[derive(Serialize, Deserialize)]
pub struct OAuthDevice {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u32,
    pub interval: u32,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthDeviceJWToken {
    pub access_token: String,
    pub expires_in: u32,
    //...
}

pub async fn handle_oauth2(sso_login: &SSOLogin) -> anyhow::Result<OAuthDevice> {
    let response = reqwest::Client::new().post(format!("{}/realms/{}/protocol/openid-connect/auth/device", &sso_login.sso_url, &sso_login.realm))
        .form(&[("client_id", &sso_login.client_id)])
        .send().await?.json::<OAuthDevice>().await?;

    return Ok(response)
}

// https://github.com/keycloak/keycloak-community/blob/main/design/oauth2-device-authorization-grant.md
pub async fn handle_oauth(identity: Identity, client:&mut AuthClient<Channel>, sso_login: &SSOLogin, stream: &mut APISocket, device_id: Option<String>) -> anyhow::Result<SuccessResponse> {

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
            //Seq(request.accessToken, request.deviceCode, deviceId, request.networkId)
            let params = vec![Some(loop_response.access_token.clone()), Some(response.device_code.clone()), device_id.clone(), Some(sso_login.network_token_id.clone())].into_iter().filter_map(|v|v).collect::<Vec<String>>();
            let encrypt = identity.sign(params)?;
            let request = Request::new(OAuthDeviceCodeRequest {
                device_code: (&response.device_code).clone(),
                access_token: loop_response.access_token,
                network_token_id: sso_login.network_token_id.clone(),
                encrypt:Some(encrypt),
                device_id,
            });
            let response= client.oauth_device_code_confirm(request).await?.into_inner().response;
            return match response {
                Some(Response::Error(message)) => Err(anyhow!(message)),
                Some(Response::Success(resp))=> Ok(resp),
                _ => Err(anyhow!("analyse auth response error")),
            }
        } else {
            tracing::debug!("check login status: not login, will try to check after {} seconds...", response.interval + 1);
        }
    }
    return Err(anyhow!("this login cost more time than expected, please try again"));
}


pub async fn server_invite_confirm(
    identity: &Identity,
    endpoint: &String,
    network_id: &String,
    node_id: Option<String>,
    device_id: Option<String>,
) -> anyhow::Result<SuccessResponse> {
    tracing::debug!("endpoint: {endpoint}");
    let channel = Channel::from_shared(endpoint.clone())?.connect().await?;
    let mut client = AuthClient::new(channel);

    let params = vec!(device_id.clone(),Some(network_id.to_owned()),node_id.clone()).into_iter().filter_map(|v|{
        v
    }).collect::<Vec<String>>();

    let encrypt = identity.sign(params)?;
    tracing::debug!("encrypt: {encrypt:?}");

    let request = Request::new(InviteConfirmRequest {
        node_id,
        network_token_id: network_id.clone(),
        encrypt: Some(encrypt),
        device_id,
    });
    let response = client.invite_confirm(request).await?;

    match response.into_inner().response {
        Some(Response::Error(message)) => Err(anyhow!(message)),
        Some(Response::Success(resp))=> Ok(resp),
        _ => Err(anyhow!("analyse auth response error")),
    }
}

pub(crate) struct InviteToken {
    pub endpoint: String,
    pub network_token_id: String,
    pub node_id: Option<String>,
}

impl InviteToken {
    pub fn new(data: Vec<&str>) -> Self {
        let endpoint = data[1].to_owned();
        let network_token_id = data[2].to_owned();
        let node_id = if data.len() > 3 {
            Some(data[3].to_owned())
        } else {
            None
        };
        InviteToken {
            endpoint,
            network_token_id,
            node_id,
        }
    }
}

pub struct SSOLogin {
    pub endpoint: String,
    pub network_token_id: String,
    pub sso_url: String,
    pub realm: String,
    pub client_id: String,
}

impl SSOLogin {
    pub async fn get_login_info(data: Vec<&str>) -> anyhow::Result<(AuthClient<Channel>, SSOLogin)> {
        let grpc_endpoint = data[1].to_owned();
        let network_token_id = data[2].to_owned();

        let channel = Channel::from_shared(grpc_endpoint.clone())?.connect().await?;
        let mut client = AuthClient::new(channel);
        let request = Request::new(SsoLoginInfoRequest {
            network_id: network_token_id.clone(),
        });
        let response = client.get_sso_login_info(request).await?;
        let response = response.into_inner();
        //let response = reqwest::get(format!("{}/api/auth/oauth/device_code?n_id={}", &grpc_endpoint, &network_id)).await?;
        Ok((client, SSOLogin {
            endpoint: grpc_endpoint,
            network_token_id,
            sso_url: response.sso_url,
            realm: response.realm,
            client_id: response.client_id,
        }))

    }
}

pub fn invite_token_parse(data: &str) -> anyhow::Result<(u32, String, String, Option<String>)> {
    let data = String::from_utf8(base64::engine::general_purpose::STANDARD.decode(data)?)?;
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

pub enum JoinNetworkResult {
    JoinSuccess(ServerInfo, String),
    WaitingSSOAuth {
        resp:OAuthDevice,
        sso:SSOLogin,
        client:AuthClient<Channel>,
        device_id:Option<String>
    }
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
