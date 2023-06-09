use std::str::FromStr;
use clap::{Arg, Command};

use fornet_lib::{APP_NAME, default_config_path};
use fornet_lib::server_manager::{ServerManager, StartMethod};
use tracing_subscriber::EnvFilter;
use fornet_lib::device::check_permission;


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

    ServerManager::start_server(config_dir, StartMethod::CommandLine).await?;
    tokio::signal::ctrl_c().await.unwrap();
    Ok(())
}



