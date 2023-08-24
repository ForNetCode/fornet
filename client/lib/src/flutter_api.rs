use std::str::FromStr;
use cfg_if::cfg_if;
use std::sync::OnceLock;
use flutter_rust_bridge::frb;

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


// MacOS/Linux/Windows
pub fn get_config_path() -> String {
    option_env!("FORNET_CONFIG").map(|x|x.to_owned()).unwrap_or_else(||default_config_path())
}

cfg_if! {
    if #[cfg(target_os="android")] {
        fn init_log(log_level:String) {
            let log_level: tracing_subscriber::filter::LevelFilter = log_level.parse().unwrap();
            //TODO: need change com.example.for_net_ui
            let android_layer = paranoid_android::layer("com.fornet.ui")
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




use crate::protobuf::config::*;



#[frb(mirror(Interface))]
pub struct _Interface {
    pub name: Option<String>,
    pub address: Vec<String>,
    pub listen_port: i32,
    pub dns: Vec<String>,
    pub mtu: Option<u32>,
    pub pre_up: Option<String>,
    pub post_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_down: Option<String>,
    pub protocol: i32,
}

#[frb(mirror(Peer))]
struct _Peer {
    pub endpoint: Option<String>,
    pub allowed_ip: Vec<String>,
    pub public_key: String,
    pub persistence_keep_alive: u32,
    pub address:Vec<String>,
}
#[frb(mirror(PeerChange))]
struct _PeerChange {

    pub add_peer: Option<Peer>,

    pub remove_public_key: Option<String>,
    pub change_peer: Option<Peer>,
}

#[frb(mirror(WrConfig))]
struct _WrConfig {
    pub interface: Option<Interface>,
    pub peers: Vec<Peer>,
    pub typ: i32,
}
use client_message::Info as ClientInfo;
#[frb(mirror(ClientMessage))]
struct _ClientMessage {
    pub network_id: String,
    pub info: Option<ClientInfo>,
}
#[frb(mirror(ClientInfo))]
enum _Info {
    Config(WrConfig),
    Status(i32),
}

use network_message::Info as NetworkInfo;
#[frb(mirror(NetworkMessage))]
struct _NetworkMessage {
    pub network_id: String,
    pub info: Option<NetworkInfo>,
}

#[frb(mirror(NodeType))]
enum _NodeType {
    NodeClient = 0,
    NodeRelay = 1,
}
#[frb(mirror(NodeStatus))]
enum _NodeStatus {
    NodeWaiting = 0,
    NodeNormal = 1,
    NodeForbid = 2,
}

pub fn test_param(client_message: ClientMessage) -> Option<ClientMessage>{
    None
}

