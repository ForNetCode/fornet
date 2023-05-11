use std::path::PathBuf;
use anyhow::bail;
use crate::server_api::{send_command, send_command_stream, StreamResponse};
use crate::config::ServerConfig;

pub fn config_path_init(path:String) -> anyhow::Result<PathBuf> {
    let config_dir = PathBuf::from(path);
    if !config_dir.exists() {
        //panic!("{:?} already exists, it now do not support join multiple network", &config_dir);
        std::fs::create_dir(&config_dir)?;
    } else if !config_dir.is_dir() {
        bail!("{:?} is not directory", config_dir);
    }
    if ServerConfig::exits(&config_dir) {
        bail!(
            "{:?} already exists, it now do not support join multiple network",
            ServerConfig::config_file_path(&config_dir)
        );
    }
    Ok(config_dir)

}

pub async fn join_network(invite_code:&str)->anyhow::Result<StreamResponse> {
    send_command_stream(&format!("join {}", invite_code)).await
}

pub async fn list_network() -> anyhow::Result<String> {
    send_command( "list").await
}

pub async fn auto_launch(sub_command:&str) -> anyhow::Result<String> {
    send_command(&format!("autoLaunch {sub_command}")).await
}