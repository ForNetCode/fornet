use std::sync::Arc;
use anyhow::Context;
use cfg_if::cfg_if;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{RwLock, Mutex};
use tokio::task::JoinHandle;
use crate::api::{api_error, api_success, ApiJsonResponse, ApiResponse, JoinNetworkResult, APISocket, get_server_api_socket_path};
use crate::client_manager::ForNetClient;
use crate::sc_manager::ConfigSyncManager;

pub struct FileSocketApiServer {
    handler: JoinHandle<()>
}

impl FileSocketApiServer {
    pub fn start(client_manager:Arc<RwLock<ForNetClient>>,  config_sync_manager: Arc<Mutex<ConfigSyncManager>>) ->anyhow::Result<FileSocketApiServer> {
        let (sender, receiver) = tokio::sync::mpsc::channel::<APISocket>(10);
        let handler = crate::api::init_api_server(
            sender, get_server_api_socket_path()
        ).with_context(|| "init file socket api server fail")?;
        let server = FileSocketApiServer {
            handler,
        };
        server.api_handler(client_manager, config_sync_manager, receiver);
        Ok(server)
    }

    fn api_handler(&self, client_manager:Arc<RwLock<ForNetClient>>,  config_sync_manager: Arc<Mutex<ConfigSyncManager>>, mut receiver:Receiver<APISocket>) {
        tokio::spawn(async move {
            while let Some(mut stream) = receiver.recv().await  {
                let mut buffer = vec![0u8;1024];
                if let Ok(size) = stream.read(&mut buffer).await {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let command:Vec<&str> = command.split(' ').collect::<Vec<&str>>();
                    match command[0] {
                        "join" => {
                            let join_network_result = {
                                client_manager.write().await.join_network(command[1]).await
                            };
                            match join_network_result {
                                Ok(JoinNetworkResult::JoinSuccess(server_info, network_token_id))=> {
                                    let _ = stream.write(api_success("join success".to_owned()).to_json().as_bytes()).await;
                                    let _  = config_sync_manager.lock().await.connect(server_info.mqtt_url).await;
                                }
                                Ok(JoinNetworkResult::WaitingSSOAuth {resp,sso,mut client,device_id}) => {
                                    let sso_auth_check_result = {
                                        client_manager.write().await.sso_auth_check(resp,&sso,&mut client, device_id).await
                                    };
                                    match sso_auth_check_result {
                                        Ok((server_info, network_token_id)) => {
                                            let _ = stream.write(api_success("join success".to_owned()).to_json().as_bytes()).await;
                                            let _  = config_sync_manager.lock().await.connect(server_info.mqtt_url).await;
                                        }
                                        Err(e)=> {
                                            let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
                                }
                            }
                        }
                        "list" => {
                            let data = client_manager.read().await.list_network().await;
                            let _ = stream.write(ApiResponse::boxed(data).to_json().as_bytes()).await;
                        }
                        "autoLaunch" => {
                            cfg_if! {
                                if #[cfg(any(target_os="macos", target_os = "windows"))] {
                                    match crate::client_manager::auto_launch(command[1]).await {
                                        Ok(resp) => {
                                            let _ = stream.write(api_success(resp).to_json().as_bytes()).await;
                                        }
                                        Err(e) => {
                                            let _ = stream.write(api_error(e.to_string()).to_json().as_bytes()).await;
                                        }
                                    }
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
                } else {
                    tracing::error!("read command error");
                }
            }
            tracing::debug!("FileSocketApiServer does not accept new request");
        });
    }


}