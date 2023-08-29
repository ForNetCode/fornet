use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use clap::{Arg, Command};
use tokio::sync::{Mutex, RwLock};

use fornet_lib::{APP_NAME, default_config_path};
use fornet_lib::server_manager::{ServerManager, StartMethod};
use tracing_subscriber::EnvFilter;
use fornet_lib::api::file_socket_api_server::FileSocketApiServer;
use fornet_lib::client_manager::{ForNetClient, pc_handle_server_message};
use fornet_lib::config::AppConfig;
use fornet_lib::device::check_permission;
use fornet_lib::sc_manager::ConfigSyncManager;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if !check_permission() {
        panic!("Please run as Root/Administrator User");
    }
    let default_config_path = default_config_path();

    let matches = Command::new(APP_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .author("ForNetCode <zsy.evan@gmail.com>")
        .args(&[
            Arg::new("config")
                .long("config")
                .short('c')
                .env("FORNET_CONFIG")
                .help("config directory path")
                .default_value(&default_config_path),
        ])
        .get_matches();


    let config_dir = matches.value_of("config").unwrap().to_owned();


    //console_subscriber::init();

    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from_str(log_level).unwrap());
    if cfg!(debug_assertions) {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(false)
            .with_ansi(false)
            .init();
    }

    let config_dir = PathBuf::from(config_dir);
    let app_config = AppConfig::load_config(&config_dir)?;


    let client = Arc::new(RwLock::new(ForNetClient::new(app_config)));
    //ConfigSyncManager
    let (config_sync_manager,mut receiver ) = ConfigSyncManager::new(client.clone());
    let config_sync_manager = Arc::new(Mutex::new(config_sync_manager));
    let api_server = FileSocketApiServer::start(client.clone(), config_sync_manager)?;




    //ServerManager::start_server(config_dir, StartMethod::CommandLine).await?;
    tokio::spawn(async move {
        while let Some(message) = receiver.recv() {
            pc_handle_server_message(client.clone(), message).await;
        }
    });
    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}



