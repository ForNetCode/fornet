#![allow(dead_code)]

mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod config;
pub mod server_manager;
pub mod sc_manager;
pub mod wr_manager;
pub mod device;
pub mod server_api;
pub mod api;
pub mod flutter_api;
pub mod client_manager;


pub mod protobuf {
    pub mod auth {
        tonic::include_proto!("auth");
    }
    pub mod config {
        tonic::include_proto!("config");
    }
}
pub const APP_NAME:&str = "fornet";

pub const MAC_OS_PACKAGE_NAME:&str = "com.timzaak.fornet";

cfg_if::cfg_if! {
    if #[cfg(any(target_os="windows",target_os="macos"))] {
        pub fn default_config_path() -> String {
            dirs::home_dir().unwrap().join(format!(".{}",APP_NAME)).into_os_string().into_string().unwrap()
        }
    } else {
        //Linux, Android, iOS does not support.
        pub fn default_config_path() -> String {
            format!("/etc/{}",APP_NAME)
        }
    }
}
