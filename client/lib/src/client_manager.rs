use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use std::str::FromStr;
use std::sync::Arc;
use anyhow::{anyhow, bail};
use base64::Engine;
use cfg_if::cfg_if;
use tokio::sync::RwLock;
use tonic::transport::{Channel};
use crate::api::{handle_oauth2, InviteToken, JoinNetworkResult, OAuthDevice, OAuthDeviceJWToken, server_invite_confirm, SSOLogin};
use crate::config::{AppConfig, Identity, ServerInfo};
use crate::device::{Device};
use crate::device::peer::AllowedIP;
use crate::device::script_run::Scripts;
use crate::protobuf::auth::auth_client::AuthClient;
use crate::protobuf::auth::OAuthDeviceCodeRequest;
use crate::protobuf::config::{NodeType, Peer, WrConfig, Protocol, PeerChange};
use crate::wr_manager::{DeviceInfoResp};




pub struct ForNetClient {
    pub config: AppConfig,
    pub device: Option<Device>,
    pub wr_configs: HashMap<String, WrConfig>
}

impl Debug for ForNetClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ForNetClient").finish()
    }
}

impl ForNetClient {
    pub fn new(config:AppConfig) -> Self {
        ForNetClient {
            config,
            device: None,
            wr_configs: HashMap::new(),
        }
    }

    pub async fn join_network(&mut self, invite_code:&str) ->anyhow::Result<JoinNetworkResult> {
        if !self.config.local_config.server_info.is_empty() {
            bail!("ForNet now don't support join multiple network")
        }
        let data = String::from_utf8(base64::engine::general_purpose::STANDARD.decode(invite_code)?)?;
        let data: Vec<&str> = data.split('|').collect();
        let version = data[0].parse::<u32>()?;
        if version == 1u32 {

            let invite_token = InviteToken::new(data);
            if self.config.local_config.server_info.iter().find(|x|&x.server_url == &invite_token.endpoint && x.network_id.iter().find(|x| *x == &invite_token.network_token_id).is_some()).is_some() {
                bail!("this node has joined the network {}", &invite_token.network_token_id)
            }
            let info = self.config.local_config.server_info.iter().find(|x|&x.server_url == &invite_token.endpoint);
            let device_id_opt = info.map(|info| info.device_id.clone());
            let (mut server_info, network_token_id) = match server_invite_confirm(
                &self.config.identity,
                &invite_token.endpoint,
                &invite_token.network_token_id,
                invite_token.node_id,
                device_id_opt,
            )
                .await {
                Ok(resp) => {
                    match info {
                        Some(info) => {
                            let server_url  = info.server_url.clone();
                            let _resp = info.clone();
                            let server_info = self.config.local_config.server_info.clone();
                            self.config.local_config.server_info = server_info.into_iter().map(|mut info|{
                                if &info.server_url == &server_url {
                                    info.network_id.push(invite_token.network_token_id.clone())
                                }
                                info
                            }).collect();
                            (_resp, invite_token.network_token_id)
                        },
                        None => {
                            let server_info = ServerInfo {
                                server_url: invite_token.endpoint,
                                mqtt_url: resp.mqtt_url,
                                device_id: resp.device_id,
                                network_id: vec![invite_token.network_token_id.clone()]
                            };
                            self.config.local_config.server_info.push(server_info.clone());
                            (server_info, invite_token.network_token_id)
                        }
                    }
                }

                Err(e) => {
                    tracing::warn!("connect server error!, {e}");
                    bail!("connect server error!, {e}")
                }
            };
            self.config.local_config.save_config(&self.config.config_path)?;
            server_info.network_id = vec![];
            return Ok(JoinNetworkResult::JoinSuccess(server_info, network_token_id))
        }else if version == 2u32 {
            let server_info = &self.config.local_config.server_info;

            let (client, sso_login) = SSOLogin::get_login_info(data).await?;
            if server_info.iter().find(|x|&x.server_url == &sso_login.endpoint && x.network_id.iter().find(|x| *x == &sso_login.network_token_id).is_some()).is_some() {
                bail!("this node has joined the network {}", &sso_login.network_token_id)
            }
            let device_id_opt = server_info.iter().find_map(|x|if &x.server_url == &sso_login.endpoint {Some(x.device_id.clone())} else {None});
            return Ok(JoinNetworkResult::WaitingSSOAuth{
                resp:handle_oauth2(&sso_login).await?,
                sso:sso_login,
                client,
                device_id:device_id_opt,
            });
        }

        bail!("please upgrade ForNet, it does not support the new join network format")
    }

    pub async fn sso_auth_check(&mut self, response:OAuthDevice, sso_login:&SSOLogin, client:&mut AuthClient<Channel>, device_id:Option<String>) -> anyhow::Result<(ServerInfo,String)>{
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
                        let server_config = ServerInfo {
                            server_url: sso_login.endpoint.clone(),
                            mqtt_url: resp.mqtt_url,
                            device_id: resp.device_id,
                            network_id: vec![sso_login.network_token_id.clone()],
                        };
                        self.config.local_config.server_info.push(server_config.clone());
                        self.config.local_config.save_config(&self.config.config_path)?;
                        Ok((server_config, sso_login.network_token_id.clone()))
                    },
                    _ => bail!("analyse auth response error"),
                }
            } else {
                tracing::debug!("check login status: not login, will try to check after {} seconds...", response.interval + 1);
            }
        }
        return bail!("this login cost more time than expected, please try again");
    }

    pub async fn list_network(&self) -> Vec<DeviceInfoResp>  {
        //TODO: add api to get network name
        self.config.local_config.server_info.iter().flat_map(|info| info.network_id.clone().into_iter().map(|network_id|{
            DeviceInfoResp {
                name: network_id
            }
        })).collect()
    }

    async fn add_peers(&mut self, peers:&[Peer]) -> anyhow::Result<()> {
        for peer in peers {
            let (x_pub_key,_) = Identity::get_pub_identity_from_base64(&peer.public_key)?;
            let endpoint = peer.endpoint.as_ref().map(|v| SocketAddr::from_str(v).unwrap());
            let allowed_ip:Vec<AllowedIP> = peer.allowed_ip.iter().map(|ip| AllowedIP::from_str(ip).unwrap()).collect();
            let ip:IpAddr = peer.address.first().unwrap().parse().unwrap();
            let keepalive = peer.persistence_keep_alive as u16;
            self.add_peer(x_pub_key,
                          endpoint,
                           allowed_ip.as_slice(),
                           ip,
                           Some(keepalive)).await;
        }
        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(device) =  self.device.as_mut() {
            device.close().await;
            self.device = None;
        }
    }

    #[cfg(not(target_os = "android"))]
    pub async fn start(&mut self, _network_token_id: String, wr_config: WrConfig) ->anyhow::Result<()> {

        let interface = wr_config.interface.unwrap();

        let mut address: Vec<AllowedIP> =Vec::new();
        for addr in &interface.address {
            address.push(AllowedIP::from_str(addr).map_err(|e| anyhow!(e))?);
        }
        if self.device.is_some() {
            bail!("It's already run, please stop it firstly")
        }
        cfg_if! {
            if #[cfg(target_os = "windows")] {
                let tun_name = &self.config.local_config.tun_name.clone().unwrap();
            } else {
                let tun_name = self.config.local_config.tun_name.clone();
            }
        }

        let protocol = Protocol::from_i32(interface.protocol).unwrap_or(Protocol::Udp);
        let node_type = NodeType::from_i32(wr_config.r#type).unwrap();

        let scripts = Scripts::load_from_interface(&interface);
        let key_pair = (self.config.identity.x25519_sk.clone(), self.config.identity.x25519_pk.clone());

        tracing::debug!("begin to start device");
        #[cfg(not(target_os = "windows"))]
        let wr_interface = Device::new(
            tun_name,
            &address,
            key_pair,
            Some(interface.listen_port as u16),
            interface.mtu.unwrap_or(1420) as u32,
            scripts,
            protocol,
            node_type,
        )?;
        #[cfg(target_os = "windows")]
        let wr_interface = Device::new(
            tun_name,
            &address,
            key_pair,
            Some(interface.listen_port as u16),
            interface.mtu.unwrap_or(1420) as u32,
            scripts,
            protocol,
            node_type,
            self.config.driver_path.clone(),
        )?;
        if Some(&wr_interface.name) != self.config.local_config.tun_name.as_ref() {
            self.config.local_config.tun_name = Some(wr_interface.name.clone());
            self.config.local_config.save_config(&self.config.config_path)?;
        }
        self.device = Some(wr_interface);
        self.add_peers(&wr_config.peers).await?;
        Ok(())
    }
    pub async fn peer_change_sync(&mut self, peer_change_message:PeerChange) {
        if let Some(public_key) = peer_change_message.remove_public_key {

            if self.config.identity.pk_base64 != public_key {
                match Identity::get_pub_identity_from_base64(&public_key) {
                    Ok((x_pub_key, _)) => {
                        self.remove_peer(&x_pub_key).await;
                    }
                    Err(_) => {
                        tracing::warn!("peer identity parse error")
                    }
                }
            }
        }
        if let Some(peer) = peer_change_message.add_peer {
            let ip:IpAddr = peer.address.first().unwrap().parse().unwrap();
            let allowed_ip:Vec<AllowedIP> = peer.allowed_ip.into_iter().map(|ip| AllowedIP::from_str(&ip).unwrap()).collect();
            let endpoint = peer.endpoint.map(|endpoint| endpoint.parse::<SocketAddr>().unwrap());
            if let Ok((x_pub_key,_)) = Identity::get_pub_identity_from_base64(&peer.public_key) {
                self.add_peer(
                    x_pub_key,
                    endpoint,
                    &allowed_ip,
                    ip,
                    Some(peer.persistence_keep_alive as u16),
                ).await;
            }

        }
        if let Some(peer) = peer_change_message.change_peer {

            if &self.config.identity.pk_base64 != &peer.public_key {
                let ip:IpAddr = peer.address.first().unwrap().parse().unwrap();
                let allowed_ip:Vec<AllowedIP> = peer.allowed_ip.into_iter().map(|ip| AllowedIP::from_str(&ip).unwrap()).collect();
                let endpoint = peer.endpoint.map(|endpoint| endpoint.parse::<SocketAddr>().unwrap());
                let (x_pub_key,_) = Identity::get_pub_identity_from_base64(&peer.public_key).unwrap();
                self.add_peer(
                    x_pub_key,
                    endpoint,
                    &allowed_ip,
                    ip,
                    Some(peer.persistence_keep_alive as u16),).await;
            }
        }
    }

    #[cfg(target_os = "android")]
    pub async fn start(&mut self, raw_fd: i32, protocol:i32, port:Option<u16>, peers:Vec<Peer>) -> anyhow::Result<()>{
        if self.device.is_some() {
            bail!("It's already run, please stop it firstly")
        }
        let protocol = Protocol::from_i32(protocol).unwrap();
        let key_pair = (self.config.identity.x25519_sk.clone(), self.config.identity.x25519_pk.clone());
        let raw_fd = std::os::fd::RawFd::from(raw_fd);
        let device = Device::new(key_pair, port, protocol, raw_fd)?;
        self.device = Some(device);
        self.add_peers(peers).await?;
        Ok(())
    }

    pub async fn add_peer(&mut self,
                          //network_token_id:&str,
                          pub_key: x25519_dalek::PublicKey,
                          endpoint: Option<SocketAddr>,
                          allowed_ips: &[AllowedIP],
                          ip:IpAddr,
                          keepalive: Option<u16>) {
        if let Some(device) = self.device.as_mut() {
            device.update_peer(
                pub_key,
                false,
                endpoint,
                allowed_ips,
                keepalive,
                ip,
                None,
            ).await;
        } else {
            tracing::warn!("there's no active device when add/update peer")
        }
    }

    pub async fn is_alive(&self) -> bool {self.device.is_some()}

    pub async fn remove_peer(&mut self,public_key: &x25519_dalek::PublicKey) {
        if let Some(device) = self.device.as_mut() {
            device.remove_peer(public_key).await;
        } else {
            tracing::warn!("there's no device when remove peer")
        }
    }

    pub async fn close(&mut self) {
        if let Some(device)= self.device.as_mut() {
            device.close().await;
            self.device = None;
        }else {
            tracing::warn!("there's no device to close")
        }
    }
}
pub async fn command_handle_server_message(client:Arc<RwLock<ForNetClient>>, message:ServerMessage) {
    tracing::debug!("GOT = {:?}", message);
    match message {
        ServerMessage::StopWR{ network_id,reason, delete_network} => {
            tracing::info!("stop proxy, reason: {}", reason);
            if delete_network {
                // this must be true...
                let mut client = client.write().await;
                client.config.local_config.server_info = client.config.local_config.server_info.clone().into_iter().filter_map(|mut x|{
                    x.network_id = x.network_id.into_iter().filter(|x| x != &network_id).collect();
                    if !x.network_id.is_empty() {
                        Some(x)
                    } else {
                        None
                    }}).collect();
                let _ = client.config.local_config.save_config(&client.config.config_path);

            }
            client.write().await.close().await;
        }

        ServerMessage::SyncConfig(network_token_id,wr_config) => {
            let mut client = client.write().await;
            client.stop().await;

            if let Err(e) = client.start(network_token_id, wr_config).await {
                tracing::warn!("start device error:{e}");
            }
        }

        ServerMessage::SyncPeers(_network_token_id, peer_change_message) => {
            let mut client = client.write().await;
            client.peer_change_sync(peer_change_message).await;
        }
    };
}


#[cfg(any(target_os = "macos", target_os = "windows"))]
pub async fn auto_launch(param:&str)->anyhow::Result<String> {
    match std::env::current_dir() {
        Ok(x) => {
            let app_path = x.join(crate::APP_NAME);
            //std::env::current_exe()
            #[cfg(target_os = "macos")]
            let auto = crate::device::auto_launch::AutoLaunch::new(crate::MAC_OS_PACKAGE_NAME.to_owned(), app_path.to_str().unwrap().to_owned())?;
            #[cfg(target_os = "windows")]
            let auto =crate::device::auto_launch::AutoLaunch::new(crate::APP_NAME.to_owned(),app_path.to_str().unwrap().to_owned())?;

            tracing::debug!("app name:{}, app path: {:?}",crate::APP_NAME, app_path);
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
                "disable" => {
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
                        format!("{} auto launch: {}", crate::APP_NAME, if x { "enabled" } else { "disabled" })
                    })
                }
            }
        }
        Err(e) => {
            Err(anyhow::anyhow!(e))
        }
    }
}


#[derive(Debug, Clone)]
pub enum ServerMessage {
    // NodeStatus::Normal => start WireGuard, other => stop WireGuard
    StopWR{network_id:String,reason:String, delete_network:bool, },
    SyncPeers(String, crate::protobuf::config::PeerChange),
    SyncConfig(String, WrConfig),
}

#[cfg(test)]
mod test {
    #[test]
    fn test_name() {
        println!("{:?}", std::env::current_exe())

    }
}