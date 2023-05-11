use std::convert::identity;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use paho_mqtt as mqtt;
use paho_mqtt::SslVersion::Default;
use prost::Message;
use tokio::sync::mpsc::Sender;
use tokio_stream::StreamExt;
use tonic::metadata::{Ascii, MetadataValue};
use tonic::Request;
use tonic::transport::Channel;

use crate::protobuf::config::{ClientMessage, NetworkMessage, NodeStatus, PeerChange, WrConfig};
use crate::protobuf::config::client_message::Info::{Config, Status};
use crate::protobuf::config::network_message::Info::Peer;
use crate::server_manager::ServerMessage;

//Sync Config Manager

struct Duplication {
    wr_config: Option<WrConfig>,
    status: Option<NodeStatus>,
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

    pub async fn mqtt_connect(&mut self, config: Arc<crate::config::Config>) -> anyhow::Result<()> {
        let mut deduplication = Duplication {
            wr_config: None,
            status: None,
        };

        for (_, mqtt_url) in &config.server_config.mqtt {
            let mut client = mqtt::CreateOptionsBuilder::new()
                .server_uri(mqtt_url)
                .client_id(
                    &config.identity.pk_base64,
                ).create_client()?;
            let mut stream = client.get_stream(25);

            let encrypt = config.identity.sign2(Vec::new())?;
            let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);

            let conn_ops = mqtt::ConnectOptionsBuilder::new_v5()
                .properties(mqtt::properties![mqtt::PropertyCode::SessionExpiryInterval => 3600])
                .password(password)
                .finalize();
            //client

            //tokio spawn

            client.connect(conn_ops).await?;
            let mut topics = config.server_config.mqtt.iter().map(|(key,_)| format!("network/{key}")).collect::<Vec<String>>();
            topics.push("client".to_owned());
            let sub_opts = vec![mqtt::SubscribeOptions::with_retain_as_published(); topics.len()];

            let qos = vec![1i32; topics.len()];

            client.subscribe_many_with_options(&topics, &qos, &sub_opts, None)
                .await?;

            while let Some(msg_opt) = stream.next().await {
                if let Some(msg) = msg_opt {
                    tracing::debug!("receive message, topic: {}",msg.topic());
                    match msg.topic() {
                        "client" => {
                            if let Ok(client_message) = ClientMessage::decode(msg.payload()) {
                                if let Some(info) = client_message.info {
                                    match info {
                                        Config(wr_config) => {
                                            if deduplication.wr_config == Some(wr_config.clone()) {
                                                continue;
                                            }

                                            let _ = self.sender.send(ServerMessage::SyncConfig(wr_config.clone())).await;
                                            deduplication.wr_config = Some(wr_config);
                                        }
                                        Status(status) => {
                                            if let Some(node_status) = NodeStatus::from_i32(status) {
                                                if deduplication.status == Some(node_status) {
                                                    continue;
                                                }
                                                match node_status {
                                                    NodeStatus::NodeForbid => {
                                                        let _ = self.sender.send(
                                                            ServerMessage::StopWR("node has been forbid or delete".to_owned())
                                                        ).await;
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
                        "network" => {
                            if let Ok(network_message) = NetworkMessage::decode(msg.payload()) {
                                if let Some(info) = network_message.info {
                                    match info {
                                        Peer(peer_change) => {
                                            let _ = self.sender.send(ServerMessage::SyncPeers(peer_change)).await;
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
            break;
        }
        Ok(())
    }
}