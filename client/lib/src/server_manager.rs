use crate::config::{Config, Identity};
use crate::protobuf::config::WrConfig;
use crate::sc_manager::SCManager;
use crate::wr_manager::WRManager;
use tokio::io::AsyncWriteExt;
// this is for pip
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::io::AsyncReadExt;
use crate::server_api::APISocket;

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
            std::fs::create_dir(&path)?;
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
            config_path,
        };
        let (sender, mut receiver) = mpsc::channel::<APISocket>(10);
        crate::server_api::init_api_server(sender)?;
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
                            ServerMessage::StopWR(message) => {
                                tracing::info!("stop proxy, reason: {}", message);
                                server_manager.wr_manager.close().await;
                            }
                            ServerMessage::SyncConfig(wr_config) => {
                                if let Some(config) = &server_manager.config {
                                    server_manager.wr_manager
                                    .start(&config, wr_config)
                                    .await
                                    .unwrap_or_else(|e| panic!("wr_manager start tun error,{:?}", e));
                                }
                            }
                            ServerMessage::SyncPeers(peer_change_message) => {
                                if let Some(public_key) = peer_change_message.remove_public_key {
                                    match Identity::get_pub_identity_from_base64(&public_key) {
                                        Ok((x_pub_key, _)) => {
                                            server_manager.wr_manager.remove_peer(&x_pub_key).await;
                                        }
                                        Err(_) => {
                                            tracing::warn!("peer identity parse error")
                                        }
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
    StopWR(String),
    SyncPeers(crate::protobuf::config::PeerChange),
    SyncConfig(WrConfig),
}
