use std::sync::Arc;
use flutter_rust_bridge::StreamSink;
use tokio::sync::RwLock;
use crate::client_manager::{ForNetClient, ServerMessage};
use crate::flutter_api::ForNetFlutterMessage;

pub async fn flutter_handler_server_message(client:Arc<RwLock<ForNetClient>>, message:ServerMessage, stream:Arc<StreamSink<ForNetFlutterMessage>>) {
    tracing::debug!("GOT = {:?}", message);
    match message {
        ServerMessage::StopWR{..} => {
            stream.add(ForNetFlutterMessage::Stop);
        }
        ServerMessage::SyncConfig(network_token_id,wr_config) => {
            {
                let mut client = client.write().await;
                client.wr_configs.insert(network_token_id, wr_config);
            }
            stream.add(ForNetFlutterMessage::ConfigChange);
        }

        ServerMessage::SyncPeers(_network_token_id, peer_change_message) => {
            let mut client = client.write().await;
            client.peer_change_sync(peer_change_message).await;
        }

    }
}