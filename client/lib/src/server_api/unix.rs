use std::path::PathBuf;
use anyhow::Context;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::{UnixListener, UnixStream};
use tokio::task::JoinHandle;

pub type APISocket = UnixStream;
pub type StreamResponse = Lines<BufReader<UnixStream>>;

pub const SERVER_API_SOCKET: &str =  "/var/run/fornet.sock";


pub fn init_api_server(sender: tokio::sync::mpsc::Sender<APISocket>) -> anyhow::Result<JoinHandle<()>> {
    let api_sock_path = PathBuf::from(SERVER_API_SOCKET);
    if api_sock_path.exists() {
        std::fs::remove_file(&api_sock_path).with_context(||format!("remove api socket fail: {}", api_sock_path.display()))?;
    }
    let unix_listener = UnixListener::bind(api_sock_path).with_context(|| format!("create api socket fail: {}", api_sock_path.display()))?;
    tracing::info!("api server open");
    let task:JoinHandle<()> = tokio::spawn(async move {
        while let Ok((stream, _)) = unix_listener.accept().await {
            let _ = sender.send(stream).await;
        }
        sender.closed().await;
        tracing::info!("api server close");
    });
    Ok(task)
}

pub async fn send_command(command: &str) -> anyhow::Result<String> {
    let mut stream = UnixStream::connect(SERVER_API_SOCKET).await?;
    stream.write(command.as_bytes()).await?;
    let mut lines = BufReader::new(stream).lines();
    while let Some(line) = lines.next_line().await? {
        return Ok(line);
    }
    anyhow::bail!("could not get response");
}

pub async fn send_command_stream(command:&str) -> anyhow::Result<StreamResponse> {
    let mut stream = UnixStream::connect(SERVER_API_SOCKET).await?;
    stream.write(command.as_bytes()).await?;
    let lines = BufReader::new(stream).lines();
    Ok(lines)
}
