use std::path::PathBuf;
use anyhow::{anyhow, bail};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::windows::named_pipe::{NamedPipeServer, ClientOptions, ServerOptions, NamedPipeClient};
use tokio::task::JoinHandle;

pub type APISocket = NamedPipeServer;

pub type StreamResponse = Lines<BufReader<NamedPipeClient>>;


pub const SERVER_API_SOCKET: &str = r"\\.\pipe\fornet_sock";

pub fn init_api_server(sender: tokio::sync::mpsc::Sender<APISocket>) -> anyhow::Result<JoinHandle<()>> {
    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(SERVER_API_SOCKET)?;
    tracing::info!("api server open");
    let task: JoinHandle<()> = tokio::spawn(async move {
        //TODO: test it, fix it
        loop {
            let _ = server.connect().await;
            let _ = sender.send(server).await;
            server = ServerOptions::new().create(SERVER_API_SOCKET).unwrap();
        }

        tracing::info!("api server closed");
    });
    Ok(task)
}

pub async fn send_command(command: &str) -> anyhow::Result<String> {
    let mut client = ClientOptions::new()
        .open(SERVER_API_SOCKET)?;

    client.write(command.as_bytes()).await?;
    client.readable().await?;

    let mut buf = vec![0; 4096];
    loop {
        match client.try_read(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    let result = String::from_utf8(buf[0..size].to_vec())?;
                    return Ok(result);
                } else {
                    return Ok("".to_string());
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                bail!("fail to send message:{}", e)
            }
        }
    }
}


pub async fn send_command_stream(command:&str) -> anyhow::Result<StreamResponse> {
    let mut client = ClientOptions::new()
        .open(SERVER_API_SOCKET)?;
    client.write(command.as_bytes()).await?;
    client.readable().await?;
    Ok(BufReader::new(client).lines())
}