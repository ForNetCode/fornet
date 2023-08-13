use std::str::FromStr;
use cfg_if::cfg_if;
use std::sync::OnceLock;

use tokio::runtime::Runtime;
use tracing::Level;
use tracing_subscriber::prelude::*;
use crate::{default_config_path, server_manager};
use crate::server_api::{ApiClient, get_server_api_socket_path};
use crate::server_manager::StartMethod;

#[derive(Debug)]
struct DLLRuntime {
    rt:Runtime,
    client: ApiClient,
}

static RT:OnceLock<DLLRuntime> = OnceLock::new();

fn get_rt<'a>() -> &'a Runtime{
    &RT.get().unwrap().rt
}

fn get_client<'a>() -> &'a ApiClient {
    &RT.get().unwrap().client
}

pub fn test_one(a: i32, b: i32) -> anyhow::Result<i32> {
    Ok(a + b)
}
pub fn test_two(a:i32) -> anyhow::Result<i32> {
    println!("test two: {}", a);
    Ok(1)
}
// MacOS/Linux/Windows
pub fn get_config_path() -> String {
    option_env!("FORNET_CONFIG").map(|x|x.to_owned()).unwrap_or_else(||default_config_path())
}

cfg_if! {
    if #[cfg(target_os="android")] {
        fn init_log(log_level:String) {
            let log_level: tracing_subscriber::filter::LevelFilter = log_level.parse().unwrap();
            //TODO: need change com.example.for_net_ui
            let android_layer = paranoid_android::layer("com.example.for_net_ui")
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .with_thread_names(true)
                .with_filter(log_level);

            tracing_subscriber::registry()
                .with(android_layer)
                .init();
        }
    } else {
        fn init_log(log_level:String) {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(Level::from_str(&log_level).unwrap_or(Level::INFO))
                .init();
        }
    }
}



pub fn init_runtime(config_path:String, work_thread:usize, log_level: String) -> anyhow::Result<()> {
    // This is a workaround for the fact that Flutter always call in dev mode
    //tracing_subscriber::registry().with()
    if RT.get().is_some() {
        return Ok(());
    }

    let tokio_runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(work_thread).enable_all().build()?;

    //RT.set(tokio::runtime::Builder::new_multi_thread().worker_threads(work_thread).enable_all().build()?).unwrap();
    init_log(log_level);
    cfg_if::cfg_if!{
        if #[cfg(target_os="android")] {
            let client = ApiClient::new(get_server_api_socket_path(&config_path));
        }else {
             let client = ApiClient::new(get_server_api_socket_path());
        }
    }

    let ddl_runtime = DLLRuntime {
        rt: tokio_runtime,
        client,
    };
    RT.set(ddl_runtime).unwrap();
    tracing::info!("init tokio runtime and log success, begin to start server");
    //let is_root = nix::unistd::Uid::effective().is_root();
    //tracing::info!("is root, {is_root}, {}",nix::unistd::Uid::effective());
    get_rt().block_on(server_manager::ServerManager::start_server(config_path, StartMethod::FlutterLib))

}

pub fn join_network(invite_code:String) -> anyhow::Result<String> {
    todo!()
    //get_rt().block_on(crate::api::command_api::join_network(&invite_code))

}

pub fn list_network() -> anyhow::Result<String> {
    //get_rt().block_on(crate::api::command_api::list_network())
    todo!()
}

pub fn version() -> anyhow::Result<String> {
    Ok(get_client().version())
}
