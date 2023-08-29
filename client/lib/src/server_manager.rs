use crate::config::{Config, Identity};
use crate::protobuf::config::WrConfig;
use crate::sc_manager::SCManager;
use crate::wr_manager::WRManager;
use tokio::io::AsyncWriteExt;
// this is for pip
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use std::net::{IpAddr, SocketAddr};
use anyhow::Context;
use cfg_if::cfg_if;
use tokio::sync::mpsc;
use tokio::io::AsyncReadExt;
use crate::server_api::{APISocket, get_server_api_socket_path};
use crate::device::peer::AllowedIP;

pub struct ServerManager {
    pub wr_manager: WRManager,
    pub config: Option<Arc<Config>>,
    pub config_path: String,
}
#[derive(PartialEq)]
pub enum StartMethod {
    CommandLine,
    FlutterLib,
}
impl ServerManager {
    pub async fn start_server(config_path: String, start_method:StartMethod) -> anyhow::Result<()> {
        let (tx, mut rx) = mpsc::channel::<ServerMessage>(32);
        let path = PathBuf::from(&config_path);
        if !path.exists() {
            std::fs::create_dir(&path).with_context(|| format!("fail to create config directory: {}", path.display()))?;
        }
        let config = Config::load_config(&path)?.map(Arc::new);
        if let Some(ref config) = config {
            let mut sc_manager = SCManager::new(tx.clone());
            let config = config.clone();
            let _ = tokio::spawn(async move {
                tracing::debug!("mqtt connect");
                let _ = sc_manager.mqtt_connect(config).await;
            });
        } else {
            if start_method == StartMethod::CommandLine {
                tracing::info!("please use `fornet-cli join $TOKEN` to join the network");
            }
        }

        let mut server_manager = ServerManager {
            wr_manager: WRManager::new(),
            config: config.clone(),
            config_path: config_path.clone(),
        };
        let (sender, mut receiver) = mpsc::channel::<APISocket>(10);
        cfg_if! {
            if #[cfg(target_os="android")] {
                crate::server_api::init_api_server(sender, get_server_api_socket_path(&config_path))?;
            }else {
                crate::server_api::init_api_server(sender, get_server_api_socket_path())?;
            }
        }
        tracing::debug!("init api server success");
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(mut stream) = receiver.recv() => {
                        const BUFFER_SIZE: usize = 1024;
                        let mut buffer = [0u8; BUFFER_SIZE];
                        if let Ok(size) = stream.read(&mut buffer).await {
                            let command = String::from_utf8_lossy(&buffer[..size]);
                            crate::api::api_handler(&mut server_manager, command.to_string(), &mut stream, tx.clone()).await;
                        } else {
                            tracing::error!("read command error");
                        }
                    }
                    Some(message) = rx.recv() => {
                        tracing::debug!("GOT = {:?}", message);
                        match message {
                            ServerMessage::StopWR{ network_id,reason, delete_network} => {
                                tracing::info!("stop proxy, reason: {}", reason);
                                if delete_network {
                                    // this must be true...
                                    if let Some(config) = &mut server_manager.config {
                                        let mut server_config = config.server_config.write().await;
                                        server_config.info = server_config.info.iter().filter_map(|x|{
                                            if &x.network_id != &network_id {
                                                Some(x.clone())
                                            } else {
                                                None
                                            }}).collect();
                                        let _ = server_config.save_config(&config.config_path);
                                    }
                                }
                                server_manager.wr_manager.close(&network_id).await;
                            }
                            ServerMessage::SyncConfig(network_token_id,wr_config) => {
                                if let Some(config) = &server_manager.config {
                                    server_manager.wr_manager
                                    .start(network_token_id, &config, wr_config)
                                    .await
                                    .unwrap_or_else(|e| panic!("wr_manager start tun error,{:?}", e));
                                }
                            }
                            ServerMessage::SyncPeers(network_token_id, peer_change_message) => {

                                if let Some(public_key) = peer_change_message.remove_public_key {
                                    if server_manager.config.as_ref().map(|x|x.identity.pk_base64 != public_key).unwrap_or(true) {
                                        match Identity::get_pub_identity_from_base64(&public_key) {
                                            Ok((x_pub_key, _)) => {
                                                server_manager.wr_manager.remove_peer(&network_token_id, &x_pub_key).await;
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
                                    let (x_pub_key,_) = Identity::get_pub_identity_from_base64(&peer.public_key).unwrap();
                                    server_manager.wr_manager.add_peer(
                                        &network_token_id,
                                        x_pub_key,
                                        endpoint,
                                        &allowed_ip,
                                        ip,
                                        Some(peer.persistence_keep_alive as u16),
                                    ).await;
                                }
                                if let Some(peer) = peer_change_message.change_peer {
                                    if server_manager.config.as_ref().map(|x|x.identity.pk_base64 != peer.public_key).unwrap_or(true) {
                                        let ip:IpAddr = peer.address.first().unwrap().parse().unwrap();
                                        let allowed_ip:Vec<AllowedIP> = peer.allowed_ip.into_iter().map(|ip| AllowedIP::from_str(&ip).unwrap()).collect();
                                        let endpoint = peer.endpoint.map(|endpoint| endpoint.parse::<SocketAddr>().unwrap());
                                        let (x_pub_key,_) = Identity::get_pub_identity_from_base64(&peer.public_key).unwrap();
                                        server_manager.wr_manager.add_peer(
                                            &network_token_id,
                                            x_pub_key,
                                            endpoint,
                                            &allowed_ip,
                                            ip,
                                            Some(peer.persistence_keep_alive as u16),
                                        ).await;
                                    }
                                }
                            }
                        };
                    }
                }
            }
            tracing::info!("close server");
            exit(0);
        });

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ServerMessage {
    // NodeStatus::Normal => start WireGuard, other => stop WireGuard
    StopWR{network_id:String,reason:String, delete_network:bool, },
    SyncPeers(String, crate::protobuf::config::PeerChange),
    SyncConfig(String, WrConfig),
}
