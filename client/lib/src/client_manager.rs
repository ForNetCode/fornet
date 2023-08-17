
use std::sync::Arc;
use std::time::Duration;
use anyhow::bail;
use tonic::transport::{Channel};
use crate::api::{handle_oauth2, InviteToken, JoinNetworkResult, OAuthDevice, OAuthDeviceJWToken, server_invite_confirm, SSOLogin};
use crate::config::{Config, NetworkInfo, ServerConfig};
use crate::protobuf::auth::auth_client::AuthClient;
use crate::protobuf::auth::OAuthDeviceCodeRequest;
use crate::wr_manager::{DeviceInfoResp, WRManager};

pub struct ForNetClient {
    pub wr_manager: WRManager,
    pub config: Arc<Config>,
    //pub sc_service: SCManager,
}

/*
//
 */

impl ForNetClient {
    pub fn new(config:Arc<Config>) -> Self {
        ForNetClient {
            wr_manager: WRManager::new(),

            config,
        }
    }
    pub async fn join_network(&self, invite_code:&str) ->anyhow::Result<JoinNetworkResult> {
        let mut server_config_opt = if ServerConfig::exits(&self.config.config_path) {
            Some(ServerConfig::read_from_file(&self.config.config_path)?)
        }else {
            None
        };
        if server_config_opt.as_ref().is_some_and(|server_config| !server_config.info.is_empty()) {
            return bail!("ForNet now don't support join multiple network")
        }
        let data = String::from_utf8(base64::decode(invite_code)?)?;
        let data: Vec<&str> = data.split('|').collect();
        let version = data[0].parse::<u32>()?;

        let device_id_opt = server_config_opt.as_ref().map(|v|v.device_id.clone());

        if version == 1u32 {
            let invite_token = InviteToken::new(data);
            let server_config = match server_invite_confirm(
                &self.config.identity,
                &invite_token.endpoint,
                &invite_token.network_token_id,
                invite_token.node_id,
                device_id_opt,
            )
                .await {
                Ok(resp) => {
                    let network_info = NetworkInfo {network_id: invite_token.network_token_id, tun_name: None};
                    match &mut server_config_opt {
                        Some(server_config) => {
                            server_config.info.push(network_info);
                            server_config.save_config(&self.config.config_path)?;
                            server_config.clone()
                        },
                        None => {
                            let server_config = ServerConfig {
                                server_url: invite_token.endpoint,
                                mqtt_url: resp.mqtt_url,
                                device_id: resp.device_id,
                                info: vec![network_info],
                            };
                            server_config.save_config(&self.config.config_path)?;
                            server_config
                        }
                    }

                }
                Err(e) => {
                    tracing::warn!("connect server error!, {e}");
                    bail!("connect server error!, {e}")
                }
            };
            return Ok(JoinNetworkResult::JoinSuccess(server_config))
        }else if version == 2u32 {
            let (client, sso_login) = SSOLogin::get_login_info(data).await?;
            if server_config_opt.as_ref().is_some_and(|server_config| server_config.info.iter().find(|x| x.network_id == sso_login.network_token_id).is_some()) {
                bail!("network has been joined");
            }
            return Ok(JoinNetworkResult::WaitingSSOAuth{
                resp:handle_oauth2(&sso_login).await?,
                sso:sso_login,
                client,
                device_id:device_id_opt,
            });
        }
        return bail!("please upgrade fornet, it does not support the new join network format")
    }

    pub async fn sso_auth_check(&self, response:OAuthDevice, sso_login:&SSOLogin, client:&mut AuthClient<Channel>, device_id:Option<String>) -> anyhow::Result<ServerConfig>{
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
                let encrypt = self.config.identity.sign(params)?;
                let request = tonic::Request::new(OAuthDeviceCodeRequest {
                    device_code: (&response.device_code).clone(),
                    access_token: loop_response.access_token,
                    network_token_id: sso_login.network_token_id.clone(),
                    encrypt:Some(encrypt),
                    device_id,
                });
                let response = client.oauth_device_code_confirm(request).await?.into_inner().response;
                return match response {
                    Some(crate::protobuf::auth::action_response::Response::Error(message)) => bail!(message),
                    Some(crate::protobuf::auth::action_response::Response::Success(resp))=> {
                        let network_info = NetworkInfo {network_id: sso_login.network_token_id.clone(), tun_name: None};
                        Ok(ServerConfig {
                            server_url: sso_login.endpoint.clone(),
                            mqtt_url: resp.mqtt_url,
                            device_id: resp.device_id,
                            info: vec![network_info],
                        })
                    },
                    _ => bail!("analyse auth response error"),
                }
            } else {
                tracing::debug!("check login status: not login, will try to check after {} seconds...", response.interval + 1);
            }
        }
        return bail!("this login cost more time than expected, please try again");
    }

    pub async fn list_network(&self) -> Vec<DeviceInfoResp>{
        self.wr_manager.device_info()
    }



}

#[cfg(target_os = "macos")]
pub async fn auto_launch(param:&str)->anyhow::Result<String> {
    match std::env::current_dir() {
        Ok(x) => {
            let app_path = x.join(crate::APP_NAME);
            let auto = crate::device::auto_launch::AutoLaunch::new( crate::MAC_OS_PACKAGE_NAME.to_owned(), app_path.to_str().unwrap().to_owned());

            tracing::debug!("app name:{APP_NAME}, app path: {:?}", app_path);
            let is_enabled = auto.is_enabled();
            match param {
                "enable" => {
                    (if is_enabled.is_err() {
                        Err(is_enabled.err().unwrap())
                    } else if is_enabled.unwrap_or(false) {
                        Ok(())
                    } else {
                        auto.enable()
                    }).map(|_| {
                        "enable auto launch success".to_owned()
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
                        "disable auto launch success".to_owned()
                    })
                }
                _ => {
                    (if is_enabled.is_err() {
                        Err(is_enabled.err().unwrap())
                    } else {
                        Ok(is_enabled.unwrap_or(false))
                    }).map(|x| {
                        format!("{APP_NAME} auto launch: {}", if x { "enabled" } else { "disabled" })
                    })
                }
            }
        }
        Err(e) => {
            anyhow::anyhow!((e))
        }
    }
}
