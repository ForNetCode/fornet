use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use anyhow::bail;
use async_trait::async_trait;
use mqrstt::{AsyncEventHandler, ConnectOptions, MqttClient, new_tokio};
use mqrstt::packets::{Packet, QoS, SubscriptionOptions};
use prost::Message;
use tokio_rustls::rustls::{ClientConfig, ServerName};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use crate::client_manager::{ForNetClient, ServerMessage};
use crate::protobuf::config::{ClientMessage, NetworkMessage, NetworkStatus, NodeStatus, WrConfig};
use crate::protobuf::config::client_message::Info::{Config, Status};
use crate::protobuf::config::network_message::Info::{Peer, Status as NStatus};

//Sync Config Manager
#[derive(Clone)]
struct Duplication {
    wr_config: Option<WrConfig>,
    status: Option<NodeStatus>,

}

impl Default for Duplication {
    fn default() -> Self {
        Duplication {
            wr_config: None,
            status: None,
        }
    }
}

struct MqttWrapper {
    pub mqtt_client: MqttClient,
    pub deduplication: Duplication,
    pub sender: Sender<ServerMessage>,
    pub network_topics: Vec<String>,
    pub client_topic: String,
}

#[async_trait]
impl AsyncEventHandler for MqttWrapper {
    async fn handle(&mut self, event: Packet) {
        match event {
            Packet::Publish(p) => {
                tracing::debug!("come message, topic: {}", &p.topic);
                match &p.topic {
                    topic if topic == &self.client_topic => {
                        if let Ok(client_message) = ClientMessage::decode(p.payload) {
                            if let Some(info) = client_message.info {
                                match info {
                                    Config(wr_config) => {
                                        if self.deduplication.wr_config == Some(wr_config.clone()) {
                                            return;
                                        }
                                        let network_topic = format!("network/{}", &client_message.network_id);
                                        if !self.network_topics.contains(&network_topic) {
                                            self.network_topics.push(network_topic.clone());
                                            let _ = self.mqtt_client.subscribe((network_topic, SubscriptionOptions {
                                                qos: QoS::AtLeastOnce,
                                                ..Default::default()
                                            })).await;
                                        }
                                        self.deduplication.wr_config = Some(wr_config.clone());

                                        let _ = self.sender.send(ServerMessage::SyncConfig(client_message.network_id.clone(), wr_config)).await;
                                    }
                                    Status(status) => {
                                        if let Some(node_status) = NodeStatus::from_i32(status) {
                                            if self.deduplication.status == Some(node_status) {
                                                return;
                                            }
                                            match node_status {
                                                NodeStatus::NodeForbid => {
                                                    let _ = self.sender.send(
                                                        ServerMessage::StopWR {
                                                            network_id: client_message.network_id.clone(),
                                                            reason: "node has been forbid or delete".to_owned(),
                                                            delete_network: true,
                                                        }
                                                    ).await;
                                                    self.network_topics = self.network_topics.clone().into_iter().filter(|network_topic| &network_topic != &topic).collect();
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
                    topic if self.network_topics.contains(topic) => {
                        if let Ok(network_message) = NetworkMessage::decode(p.payload) {
                            if let Some(info) = network_message.info {
                                match info {
                                    Peer(peer_change) => {
                                        let _ = self.sender.send(ServerMessage::SyncPeers(network_message.network_id.clone(), peer_change)).await;
                                    }
                                    NStatus(status) => {
                                        if let Some(NetworkStatus::NetworkDelete) = NetworkStatus::from_i32(status) {
                                            let _ = self.sender.send(
                                                ServerMessage::StopWR {
                                                    network_id: network_message.network_id.clone(),
                                                    reason: "network has been delete".to_owned(),
                                                    delete_network: true,
                                                }
                                            ).await;
                                            self.network_topics = self.network_topics.clone().into_iter().filter(|network_topic| &network_topic != &topic).collect();
                                            let d = self.mqtt_client.unsubscribe(topic.clone()).await;


                                            tracing::debug!("unsubscribe topic: {}, result:{:?}", topic, d);
                                        }
                                    }
                                }
                            }
                        } else {
                            tracing::warn!("network message can not decode, may should update software");
                        }
                    }
                    _ => {
                        tracing::warn!("topic:{} message does not handle, may should update software", p.topic);
                    }
                }
            }
            Packet::ConnAck(v) => {
                tracing::info!("mqtt connected{:?}",v);
            }
            _ => {}
        }
    }
}


#[derive(Debug)]
pub struct ConfigSyncManager {
    client_manager: Arc<RwLock<ForNetClient>>,
    mqtt_connections: Arc<Mutex<HashMap<String, MqttClient>>>,
    sender: Sender<ServerMessage>,

}

impl ConfigSyncManager {
    pub fn new(client_manager: Arc<RwLock<ForNetClient>>) -> (Self, Receiver<ServerMessage>) {
        let (sender, rx) = tokio::sync::mpsc::channel::<ServerMessage>(32);
        (ConfigSyncManager {
            client_manager,
            mqtt_connections: Arc::new(Mutex::new(HashMap::default())),
            sender,
        }, rx)
    }
    pub async fn connect(&mut self, mqtt_url: String) -> anyhow::Result<()> {
        let deduplication = Duplication {
            wr_config: None,
            status: None,
        };
        let mqtt_connections = self.mqtt_connections.clone();
        {
            if mqtt_connections.lock().await.contains_key(&mqtt_url) {
                bail!("{} mqtt connection has already started", &mqtt_url)
            }
        }
        let sender = self.sender.clone();
        let client_manager = self.client_manager.clone();

        tokio::spawn(async move {
            let mut deduplication = deduplication;
            loop {
                tracing::debug!("begin to connect mqtt {}", &mqtt_url);
                match ConfigSyncManager::connect_mqtt(mqtt_url.clone(), deduplication.clone(), sender.clone(), client_manager.clone(), mqtt_connections.clone()).await {
                    Ok(duplication) => {
                        let has_connection = { mqtt_connections.lock().await.contains_key(&mqtt_url) };
                        tokio::time::sleep(Duration::from_secs(10)).await;
                        {
                            mqtt_connections.lock().await.remove(&mqtt_url);
                        }
                        if has_connection {
                            deduplication = duplication;
                        } else {
                            break;
                        }
                    }
                    Err(e) => {
                        let has_connection = { mqtt_connections.lock().await.contains_key(&mqtt_url) };
                        tracing::warn!("mqtt connection: {} has failed: {e}",&mqtt_url);
                        {
                            mqtt_connections.lock().await.remove(&mqtt_url);
                        }
                        if !has_connection {
                            break;
                        }
                        tokio::time::sleep(Duration::from_secs(30)).await;
                    }
                }
            }
        });
        Ok(())
    }

    async fn connect_mqtt(
        mqtt_url: String,
        deduplication: Duplication,
        sender: Sender<ServerMessage>,
        client_manager: Arc<RwLock<ForNetClient>>,
        mqtt_connections: Arc<Mutex<HashMap<String, MqttClient>>>,
    ) -> anyhow::Result<Duplication> {
        let (options, device_id, network_ids) = {
            let app_config = &client_manager.read().await.config;
            let info = app_config.local_config.server_info.iter().find_map(|info| {
                if &info.mqtt_url == &mqtt_url {
                    Some((info.device_id.clone(), info.network_id.clone()))
                } else {
                    None
                }
            });
            if let Some((device_id, network_ids)) = info {
                let username = app_config.identity.pk_base64.clone();
                let encrypt = app_config.identity.sign(vec![device_id.clone()])?;
                let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);
                let mut options = ConnectOptions::new(device_id.clone());
                options.password = Some(password);
                options.username = Some(username);
                (options, device_id, network_ids)
            } else {
                anyhow::bail!("There's no mqtt connection: {}", &mqtt_url)
            }
        };

        let url = reqwest::Url::parse(&mqtt_url)?;
        let host = url.host_str().unwrap_or("");
        let port = url.port_or_known_default().unwrap_or(1883);// secret: 8883
        let client_topic = format!("client/{}", device_id);
        let network_topics: Vec<String> = network_ids.into_iter().map(|network_id| format!("network/{}", &network_id)).collect();

        let subscribe_topics: Vec<(String, SubscriptionOptions)> = [vec![client_topic.clone()], network_topics.clone()].concat().into_iter().map(|topic| (topic, SubscriptionOptions {
            qos: QoS::AtLeastOnce,
            ..Default::default()
        })).collect();


        let mqtt_wrapper = if url.scheme() == "mqtts" {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                mqtt_client: client.clone(),
                client_topic,
                network_topics,
                deduplication,
                sender,
            };

            let root_certs = tokio_rustls::rustls::RootCertStore::empty();
            let config = ClientConfig::builder().with_safe_defaults().with_root_certificates(root_certs).with_no_client_auth();
            let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
            let domain = ServerName::try_from(host)?;
            let stream = tokio::net::TcpStream::connect((host, port)).await?;
            let connection = connector.connect(domain, stream).await?;

            network.connect(connection, &mut mqtt_wrapper).await?;
            client.subscribe(subscribe_topics).await?;
            {
                mqtt_connections.lock().await.insert(mqtt_url, client.clone());
            }
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue;
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);

                        break;
                    }
                }
            }
            mqtt_wrapper
        } else {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                mqtt_client: client.clone(),
                network_topics,
                client_topic,
                deduplication,
                sender,
            };
            let stream = tokio::net::TcpStream::connect((host, port)).await?;
            network.connect(stream, &mut mqtt_wrapper).await?;

            mqtt_wrapper.mqtt_client.subscribe(subscribe_topics).await?;
            {
                mqtt_connections.lock().await.insert(mqtt_url, client.clone());
            }
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue;
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);
                        break;
                    }
                }
            }
            mqtt_wrapper
        };
        Ok(mqtt_wrapper.deduplication)
    }

    pub async fn disconnect_mqtt(&mut self, mqtt_url: String) {
        let client = {
            self.mqtt_connections.lock().await.remove(&mqtt_url)
        };
        if let Some(client) = client {
            tracing::debug!("begin to disconnect mqtt client: {}", &mqtt_url);
            let _ = client.disconnect().await;
        } else {
            tracing::debug!("There's no mqtt client: {}", &mqtt_url);
        }
    }
}
