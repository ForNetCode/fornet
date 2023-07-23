/*
1. load config, start server
2. login/join network, keycloak, simple..
3. logout, stop client
4. list network
 */

use std::str::FromStr;
use cfg_if::cfg_if;
use once_cell::sync::OnceCell;

use tokio::runtime::Runtime;
use tracing::Level;
use crate::{default_config_path, server_manager};
use crate::server_manager::StartMethod;

static mut FLUTTER_HAS_INIT: bool = false;

static RT:OnceCell<Runtime> = OnceCell::new();

fn get_rt<'a>() -> &'a Runtime{
    RT.get().unwrap()
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


pub fn init_runtime(config_path:String, work_thread:usize, log_level: String) -> anyhow::Result<()> {
    // This is a workaround for the fact that Flutter always call in dev mode
    unsafe {
        if FLUTTER_HAS_INIT {
            return Ok(());
        }
        FLUTTER_HAS_INIT = true;
    }
    //TODO: file log with path
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(Level::from_str(&log_level).unwrap_or(Level::INFO))
        .init();


    RT.set(tokio::runtime::Builder::new_multi_thread().worker_threads(work_thread).enable_all().build()?).unwrap();
    tracing::info!("init tokio runtime and log success");
    //let is_root = nix::unistd::Uid::effective().is_root();
    //tracing::info!("is root, {is_root}, {}",nix::unistd::Uid::effective());


    get_rt().block_on(server_manager::ServerManager::start_server(config_path, StartMethod::FlutterLib))

}

pub fn join_network(invite_code:String) -> anyhow::Result<String> {
    todo!()
    //get_rt().block_on(crate::api::command_api::join_network(&invite_code))

}

pub fn list_network() -> anyhow::Result<String> {
    get_rt().block_on(crate::api::command_api::list_network())
}
