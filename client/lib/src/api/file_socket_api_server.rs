use std::sync::Arc;
use cfg_if::cfg_if;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use crate::api::{api_error, api_success, ApiJsonResponse, ApiResponse, JoinNetworkResult};
use crate::client_manager::ForNetClient;
use crate::server_api::{APISocket, get_server_api_socket_path};

struct FileSocketApiServer {
    client_manager:Arc<ForNetClient>,
    handler: JoinHandle<()>
}

impl FileSocketApiServer {
    pub fn start(client_manager:Arc<ForNetClient>) ->anyhow::Result<()> {
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<APISocket>(10);
        let handler = crate::server_api::init_api_server(
            sender, get_server_api_socket_path()
        )?;
        let server = Arc::new(FileSocketApiServer{
            client_manager,
            handler,
        });
        FileSocketApiServer::api_handler(server.clone(), receiver);
        Ok(())

    }
    fn api_handler(server:Arc<FileSocketApiServer>, mut receiver:Receiver<APISocket>) {
        tokio::spawn(async {
            while let Some(mut stream) = receiver.recv().await  {
                let mut buffer = vec![0u8;1024];
                if let Ok(size) = stream.read(&mut buffer).await {
                    let command = String::from_utf8_lossy(&buffer[..size]);
                    let command:Vec<&str> = command.split(' ').collect::<Vec<&str>>();
                    match command[0] {
                        "join" => {
                            match server.client_manager.join_network(command[1]).await {
                                Ok(JoinNetworkResult::JoinSuccess(server_config))=> {
                                    let _ = stream.write(api_success("join success".to_owned()).to_json().as_bytes()).await;
                                }
                                Ok(JoinNetworkResult::WaitingSSOAuth {resp,sso,mut client,device_id}) => {
                                    match server.client_manager.sso_auth_check(resp,&sso,&mut client, device_id).await {
                                        Ok(server_config) => {
                                            let _ = stream.write(api_success("join success".to_owned()).to_json().as_bytes()).await;
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
                            let data = server.client_manager.list_network();
                            let _ = stream.write(ApiResponse::boxed(data).to_json().as_bytes()).await;
                        }
                        "autoLaunch" => {
                            cfg_if! {
                                if #[cfg(target_os="macos")] {
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