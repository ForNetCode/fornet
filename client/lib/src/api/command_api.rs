use crate::server_api::{send_command, send_command_stream, StreamResponse};




pub async fn join_network(invite_code:&str)->anyhow::Result<StreamResponse> {
    send_command_stream(&format!("join {}", invite_code)).await
}

pub async fn list_network() -> anyhow::Result<String> {
    send_command( "list").await
}

pub async fn auto_launch(sub_command:&str) -> anyhow::Result<String> {
    send_command(&format!("autoLaunch {sub_command}")).await
}