use std::sync::Arc;
use flutter_rust_bridge::StreamSink;
use tokio::sync::RwLock;
use crate::client_manager::ForNetClient;
use crate::flutter_api::ForNetFlutterMessage;
use crate::server_manager::ServerMessage;

pub async fn flutter_handler_server_message(client:Arc<RwLock<ForNetClient>>, message:ServerMessage, stream:Arc<StreamSink<ForNetFlutterMessage>>) {
    tracing::debug!("GOT = {:?}", message);
    //stream.add(ForNetFlutterMessage::Stop);

}