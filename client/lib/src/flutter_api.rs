use std::path::PathBuf;
use cfg_if::cfg_if;
use std::sync::{Arc, OnceLock};
use anyhow::bail;
use flutter_rust_bridge::StreamSink;

use tokio::runtime::Runtime;
use tokio::sync::{Mutex, RwLock};
use crate::{default_config_path};
use crate::api::flutter_ffi::flutter_handler_server_message;
use crate::api::JoinNetworkResult;
use crate::client_manager::ForNetClient;
use crate::config::AppConfig;
use crate::sc_manager::ConfigSyncManager;

#[derive(Debug)]
struct DLLRuntime {
    rt:Runtime,
    client:Arc<RwLock<ForNetClient>>,
    sync_manager:Arc<Mutex<ConfigSyncManager>>,
}

static RT:OnceLock<DLLRuntime> = OnceLock::new();

fn get_rt<'a>() -> &'a Runtime{
    &RT.get().unwrap().rt
}

fn get_client() -> Arc<RwLock<ForNetClient>> {
    RT.get().unwrap().client.clone()
}

fn get_sync_manager() -> Arc<Mutex<ConfigSyncManager>> {
    RT.get().unwrap().sync_manager.clone()
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
        use std::str::FromStr;
        fn init_log(log_level:String) {
            tracing_subscriber::fmt()
                .pretty()
                .with_max_level(tracing::Level::from_str(&log_level).unwrap_or(tracing::Level::INFO))
                .init();
        }
    }
}


pub enum ForNetFlutterMessage {
    Stop,
    ConfigChange,
    Start,
}

pub fn init_runtime(config_path:String, work_thread:usize, log_level: String, stream:StreamSink<ForNetFlutterMessage>) -> anyhow::Result<()> {
    // This is a workaround for the fact that Flutter always call in dev mode
    //tracing_subscriber::registry().with()
    if RT.get().is_some() {
        return Ok(());
    }

    let tokio_runtime = tokio::runtime::Builder::new_multi_thread().worker_threads(work_thread).enable_all().build()?;

    //RT.set(tokio::runtime::Builder::new_multi_thread().worker_threads(work_thread).enable_all().build()?).unwrap();
    init_log(log_level);


    tracing::info!("init tokio runtime and log success, begin to start server");

    let config_dir = PathBuf::from(config_path);
    let app_config = AppConfig::load_config(&config_dir)?;
    let client = Arc::new(RwLock::new(ForNetClient::new(app_config)));


    let (config_sync_manager,mut receiver ) = ConfigSyncManager::new(client.clone());
    let config_sync_manager = Arc::new(Mutex::new(config_sync_manager));

    let ddl_runtime = DLLRuntime {
        rt: tokio_runtime,
        client: client.clone(),
        sync_manager: config_sync_manager,
    };


    let stream = Arc::new(stream);
    ddl_runtime.rt.spawn(async move {
       while let Some(message) = receiver.recv().await {
           flutter_handler_server_message(client.clone(),message, stream.clone()).await;
       }
    });
    RT.set(ddl_runtime).unwrap();
    //let is_root = nix::unistd::Uid::effective().is_root();
    //tracing::info!("is root, {is_root}, {}",nix::unistd::Uid::effective());

    Ok(())

}

pub fn join_network(invite_code:String) -> anyhow::Result<String> {
    let client = get_client();
    let sync_manager = get_sync_manager();
    let result = get_rt().block_on(async move{
        client.write().await.join_network(&invite_code).await
    });
    match result {
         Ok(JoinNetworkResult::JoinSuccess(server_info,_)) => {
             let _ = get_rt().spawn(async {
                 sync_manager.lock().await.connect(server_info.mqtt_url).await
             });
             Ok("Join Success".to_owned())
        }

        Ok(JoinNetworkResult::WaitingSSOAuth {..}) => {
            Ok("Not Implement".to_owned())
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub fn list_network() -> anyhow::Result<Vec<String>> {
    let client = get_client();
    let result = get_rt().block_on(async move {
       client.read().await.list_network().await
    });
    Ok(result.into_iter().map(|info| info.name).collect())
}

//This is for Android
pub fn start(network_id:String, raw_fd: Option<i32>) -> anyhow::Result<()> {
    if cfg!(android)  && raw_fd.is_none() {
        bail!("raw_fd show not been null")
    }
    let client = get_client();
    // WRConfig
    get_rt().block_on(async move {

    });
    get_rt().block_on(async move {
        let client = client.write().await;
        cfg_if! {
            if #[cfg(target_os = "android")] {

            }else {

            }
        }

        //client.start()
    });
    Ok(())
}
pub fn version() -> String {
   env!("CARGO_PKG_VERSION").to_owned()
}




// pub use crate::protobuf::config::{Interface, Peer,PeerChange,WrConfig};
//
//
//
// #[frb(mirror(Interface))]
// pub struct _Interface {
//     pub name: Option<String>,
//     pub address: Vec<String>,
//     pub listen_port: i32,
//     pub dns: Vec<String>,
//     pub mtu: Option<u32>,
//     pub pre_up: Option<String>,
//     pub post_up: Option<String>,
//     pub pre_down: Option<String>,
//     pub post_down: Option<String>,
//     pub protocol: i32,
// }
//
// #[frb(mirror(Peer))]
// pub struct _Peer {
//     pub endpoint: Option<String>,
//     pub allowed_ip: Vec<String>,
//     pub public_key: String,
//     pub persistence_keep_alive: u32,
//     pub address:Vec<String>,
// }
// #[frb(mirror(PeerChange))]
// pub struct _PeerChange {
//     pub add_peer: Option<Peer>,
//     pub remove_public_key: Option<String>,
//     pub change_peer: Option<Peer>,
// }
//
// #[frb(mirror(WrConfig))]
// pub struct _WrConfig {
//     pub interface: Option<Interface>,
//     pub peers: Vec<Peer>,
//     pub r#type: i32,
// }
//
//
// //#[frb(mirror(crate::protobuf::config::client_message::Info))]
// //enum _ClientInfo {
// //    Config(WrConfig),
// //    Status(i32),
// //}
// pub use crate::server_manager::ServerMessage;
//
// #[frb(mirror(ServerMessage))]
// pub enum _ServerMessage{
//     StopWR{network_id:String,reason:String, delete_network:bool},
//     SyncPeers(String, PeerChange),
//     SyncConfig(String, WrConfig),
// }
//
// #[frb(mirror(NodeType))]
// enum _NodeType {
//     NodeClient = 0,
//     NodeRelay = 1,
// }
// #[frb(mirror(NodeStatus))]
// enum _NodeStatus {
//     NodeWaiting = 0,
//     NodeNormal = 1,
//     NodeForbid = 2,
// }

// pub fn test_param(client_message: ServerMessage) -> Option<ServerMessage>{
//    None
// }

