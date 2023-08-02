use std::path::PathBuf;
use anyhow::Context;
use cfg_if::cfg_if;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::{UnixListener, UnixStream};
use tokio::task::JoinHandle;

pub type APISocket = UnixStream;
pub type StreamResponse = Lines<BufReader<UnixStream>>;

// pub const SERVER_API_SOCKET: &str =  "/var/run/fornet.sock";

cfg_if!{
    if #[cfg(target_os = "android")] {
        pub fn get_server_api_socket_path(dir:&str) -> PathBuf {
            PathBuf::from(dir).join("fornet_socket.sock")
        }

    } else {
        pub fn get_server_api_socket_path()-> PathBuf {
            PathBuf::from("/var/run/fornet.sock")
        }

    }
}

pub fn init_api_server(sender: tokio::sync::mpsc::Sender<APISocket>, api_socket_path:PathBuf) -> anyhow::Result<JoinHandle<()>> {

    if api_socket_path.exists() {
        std::fs::remove_file(&api_socket_path).with_context(||format!("remove api socket fail: {}", api_socket_path.display()))?;
    }
    let unix_listener = UnixListener::bind(&api_socket_path).with_context(|| format!("create api socket fail: {}", api_socket_path.display()))?;
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

#[derive(Debug)]
pub(super) struct _ApiClient {
    api_socket_path: PathBuf
}

impl _ApiClient {
    pub fn new(path:PathBuf) -> Self {
        Self {
            api_socket_path: path
        }
    }

    pub async fn send_command(&self, command: &str) -> anyhow::Result<String> {
        let mut stream = UnixStream::connect(&self.api_socket_path).await?;
        stream.write(command.as_bytes()).await?;
        let mut lines = BufReader::new(stream).lines();
        while let Some(line) = lines.next_line().await? {
            return Ok(line);
        }
        anyhow::bail!("could not get response");
    }

    pub async fn send_command_stream(&self, command:&str) -> anyhow::Result<StreamResponse> {
        let mut stream = UnixStream::connect(&self.api_socket_path).await?;
        stream.write(command.as_bytes()).await?;
        let lines = BufReader::new(stream).lines();
        Ok(lines)
    }
}

