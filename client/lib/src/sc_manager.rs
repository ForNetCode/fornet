use std::sync::Arc;
use std::time::Duration;
use mqrstt::{AsyncEventHandler, ConnectOptions, MqttClient, new_tokio};
use mqrstt::packets::Packet;

use paho_mqtt as mqtt;
use prost::Message;
use tokio::sync::mpsc::Sender;
use tokio_rustls::rustls;
use tokio_rustls::rustls::ClientConfig;
use tokio_stream::StreamExt;
use crate::config::{NodeInfo, Config as AppConfig};

use crate::protobuf::config::{ClientMessage, NetworkMessage, NetworkStatus, NodeStatus, WrConfig};
use crate::protobuf::config::client_message::Info::{Config, Status};
use crate::protobuf::config::network_message::Info::{Peer, Status as NStatus};
use crate::server_manager::ServerMessage;

//Sync Config Manager

struct Duplication {
    wr_config: Option<WrConfig>,
    status: Option<NodeStatus>,

}

struct MqttWrapper {
    pub client:MqttClient,
    pub client_topic: String,
    pub network_topic: String,
    pub deduplication: Duplication,
    pub sender: Sender<ServerMessage>
}

impl AsyncEventHandler for MqttWrapper {
    async fn handle(&mut self, event: Packet) {
        match event {
            Packet::Publish(p) => {
                match &p.topic {
                    topic if topic == &self.client_topic => {
                        if let Ok(client_message) = ClientMessage::decode(p.payload) {
                            if let Some(info) = client_message.info {
                                match info {
                                    Config(wr_config) => {
                                        if self.deduplication.wr_config == Some(wr_config.clone()) {
                                            return;
                                        }

                                        let _ = self.sender.send(ServerMessage::SyncConfig(wr_config.clone())).await;
                                        self.deduplication.wr_config = Some(wr_config);
                                    }
                                    Status(status) => {
                                        if let Some(node_status) = NodeStatus::from_i32(status) {
                                            if self.deduplication.status == Some(node_status) {
                                                return;
                                            }
                                            match node_status {
                                                NodeStatus::NodeForbid => {
                                                    let _ = self.sender.send(
                                                        ServerMessage::StopWR("node has been forbid or delete".to_owned())
                                                    ).await;
                                                    self.deduplication.wr_config = None;
                                                }
                                                _ => {
                                                    // this would conflict with Info::Config message, so ignore this.
                                                }
                                            }
                                            self.deduplication.status = Some(node_status)
                                        }
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("client message can not decode, may should update software");
                        }
                    }
                    topic if topic == &self.network_topic => {
                        if let Ok(network_message) = NetworkMessage::decode(p.payload) {
                            if let Some(info) = network_message.info {
                                match info {
                                    Peer(peer_change) => {
                                        let _ = self.sender.send(ServerMessage::SyncPeers(peer_change)).await;
                                    }
                                    NStatus(status) => {
                                        if let Some(NetworkStatus::NetworkDelete) = NetworkStatus::from_i32(status) {
                                            let _ = self.sender.send(
                                                ServerMessage::StopWR("network has been delete".to_owned())
                                            ).await;
                                        }
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("network message can not decode, may should update software");
                        }
                    }
                    _ => {
                        tracing::warn!("topic:{} message can not decode, may should update software", msg.topic());
                    }
                }
            }
            Packet::ConnAck(_) => {
                tracing::info!("mqtt connected");
            }
            _ => {}
        }

    }
}



pub struct SCManager {
    sender: Sender<ServerMessage>,
}
impl SCManager {
    pub fn new(sender: Sender<ServerMessage>) -> Self {
        SCManager {
            sender,
        }
    }

    async fn mqtt2_reconnect(sender:Sender<ServerMessage>,node_info: &NodeInfo, config:Arc<AppConfig>, deduplication:&mut Duplication) -> anyhow::Result<()> {
        let client_id = config.identity.pk_base64.clone();
        let options = ConnectOptions::new(client_id);
        let (mut network, client) = new_tokio(options);
        if node_info.mqtt_url.starts_with("mqtts") {
            let root_certs = rustls::RootCertStore::empty();
            let config = ClientConfig::builder().with_safe_defaults().with_root_certificates(root_certs).with_no_client_auth();
            let connector = tokio_rustls::TlsConnector::from(Arc::new(config));

            let stream = tokio::net::TcpStream::connect(node_info.mqtt_url.clone()).await?;
            let connection = connector.connect(domain, stream).await?;
            
            //network.connect(connection,)
        }
        Ok(())

    }

    async fn mqtt_reconnect(sender:Sender<ServerMessage>, node_info: &NodeInfo, config:Arc<AppConfig>, deduplication:&mut Duplication) -> anyhow::Result<()> {
        let mut client = mqtt::CreateOptionsBuilder::new()
            .server_uri(&node_info.mqtt_url)
            .client_id(
                &config.identity.pk_base64,
            ).create_client()?;
        let mut stream = client.get_stream(25);

        let encrypt = config.identity.sign2(Vec::new())?;
        let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);

        let conn_ops = mqtt::ConnectOptionsBuilder::new_v5()
            .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
            .user_name(&node_info.node_id)
            .password(password)
            .finalize();
        //client

        //tokio spawn

        client.connect(conn_ops).await?;
        let client_topic = format!("client/{}",&node_info.node_id);
        let network_topic = format!("network/{}", &node_info.network_id);
        let topics = vec!(&client_topic, &network_topic);
        let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

        let qos = vec![1i32; topics.len()];

        client.subscribe_many_with_options(&topics, &qos, &sub_opts, None)
            .await?;

        while let Some(msg_opt) = stream.next().await {
            if let Some(msg) = msg_opt {
                tracing::debug!("receive message, topic: {}",msg.topic());
                match msg.topic() {
                    topic if topic == &client_topic => {
                        if let Ok(client_message) = ClientMessage::decode(msg.payload()) {
                            if let Some(info) = client_message.info {
                                match info {
                                    Config(wr_config) => {
                                        if deduplication.wr_config == Some(wr_config.clone()) {
                                            continue;
                                        }

                                        let _ = sender.send(ServerMessage::SyncConfig(wr_config.clone())).await;
                                        deduplication.wr_config = Some(wr_config);
                                    }
                                    Status(status) => {
                                        if let Some(node_status) = NodeStatus::from_i32(status) {
                                            if deduplication.status == Some(node_status) {
                                                continue;
                                            }
                                            match node_status {
                                                NodeStatus::NodeForbid => {
                                                    let _ = sender.send(
                                                        ServerMessage::StopWR("node has been forbid or delete".to_owned())
                                                    ).await;
                                                    deduplication.wr_config = None;
                                                }
                                                _ => {
                                                    // this would conflict with Info::Config message, so ignore this.
                                                }
                                            }
                                            deduplication.status = Some(node_status)
                                        }
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("client message can not decode, may should update software");
                        }
                    }
                    topic if topic == &network_topic => {
                        if let Ok(network_message) = NetworkMessage::decode(msg.payload()) {
                            if let Some(info) = network_message.info {
                                match info {
                                    Peer(peer_change) => {
                                        let _ = sender.send(ServerMessage::SyncPeers(peer_change)).await;
                                    }
                                    NStatus(status) => {
                                        if let Some(NetworkStatus::NetworkDelete) = NetworkStatus::from_i32(status) {
                                            let _ = sender.send(
                                                ServerMessage::StopWR("network has been delete".to_owned())
                                            ).await;
                                        }
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("network message can not decode, may should update software");
                        }
                    }
                    _ => {
                        tracing::warn!("topic:{} message can not decode, may should update software", msg.topic());
                    }
                }
            } else {
                // A "None" means we were disconnected. Try to reconnect...
                while let Err(err) = client.reconnect().await {
                    tracing::debug!("mqtt reconnect error: {}", err);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
        Ok(())
    }

    pub async fn mqtt_connect(&mut self, config: Arc<crate::config::Config>) -> anyhow::Result<()> {
        for node_info in &config.server_config.info {
            let mut deduplication = Duplication {
                wr_config: None,
                status: None,
            };
            let node_info = node_info.clone();
            let _config = config.clone();
            let _sender = self.sender.clone();
            tokio::spawn(async move{
                loop {
                    let _config = _config.clone();
                    let _sender = _sender.clone();
                    let _ = SCManager::mqtt_reconnect(_sender, &node_info, _config, &mut deduplication).await;
                    tracing::debug!("mqtt connect error");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            });

        }
        Ok(())
    }
}