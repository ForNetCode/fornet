use std::sync::Arc;
use std::time::{Duration, SystemTime};
use async_trait::async_trait;
use mqrstt::{AsyncEventHandler, ConnectOptions, MqttClient, new_tokio};
use mqrstt::packets::{Packet, QoS};

use prost::Message;
use tokio::sync::mpsc::Sender;
//use tokio_rustls::rustls;
//use tokio_rustls::rustls::{ClientConfig, ServerName};

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

impl Default for Duplication {
    fn default() -> Self {
        Duplication {
            wr_config: None,
            status: None,
        }
    }
}

struct MqttWrapper<'a> {
    pub client:MqttClient,
    pub client_topic: String,
    pub network_topic: String,
    pub deduplication: &'a mut Duplication,
    pub sender: Sender<ServerMessage>
}
#[async_trait]
impl <'a> AsyncEventHandler for MqttWrapper<'a> {
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
                        tracing::warn!("topic:{} message can not decode, may should update software", p.topic);
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

    async fn mqtt_reconnect(sender:Sender<ServerMessage>, node_info: &NodeInfo, config:Arc<AppConfig>, deduplication: &mut Duplication) -> anyhow::Result<()> {
        let url = reqwest::Url::parse(&node_info.mqtt_url)?;

        let host = url.host_str().unwrap_or("");
        let port = url.port_or_known_default().unwrap_or(1883);// secret: 8883
        let client_id = config.identity.pk_base64.clone();
        let mut options = ConnectOptions::new(client_id);
        let encrypt = config.identity.sign(Vec::new())?;
        let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);
        options.password = Some(password);
        options.username = Some(node_info.node_id.clone());


        let client_topic = format!("client/{}",&node_info.node_id);
        let network_topic = format!("network/{}", &node_info.network_id);
        let subscribe_topics:&[(&str, QoS)] = &[(&client_topic.clone(),QoS::AtLeastOnce ), (&network_topic.clone(), QoS::AtLeastOnce)];
        //let deduplication =  Duplication::default();


        if url.scheme() == "mqtts" {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                client:client.clone(),
                client_topic,
                network_topic,
                deduplication,
                sender,
            };

            //let root_certs = rustls::RootCertStore::empty();
            //let config = ClientConfig::builder().with_safe_defaults().with_root_certificates(root_certs).with_no_client_auth();
            //let connector = tokio_rustls::TlsConnector::from(Arc::new(config));
            //let domain = ServerName::try_from(host)?;


            //let connection = connector.connect(domain, stream).await?;

            let cx = tokio_native_tls::native_tls::TlsConnector::builder().build()?;
            let cx = tokio_native_tls::TlsConnector::from(cx);

            let stream = tokio::net::TcpStream::connect((host, port)).await?;

            let connection = cx.connect(host, stream).await?;

            network.connect(connection,&mut mqtt_wrapper).await?;
            client.subscribe(subscribe_topics).await?;
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);
                    }
                }
            }

        } else {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                client:client.clone(),
                client_topic,
                network_topic,
                deduplication,
                sender,
            };
            let stream = tokio::net::TcpStream::connect((host, port)).await?;
            network.connect(stream, &mut mqtt_wrapper).await?;
            client.subscribe(subscribe_topics).await?;
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);
                        break;

                    }
                }
            }

        };

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
                    let v = SCManager::mqtt_reconnect(_sender, &node_info, _config, &mut deduplication).await;
                    tracing::debug!("mqtt connect error, {:?}", v);
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            });

        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_url_parse() {
        let obc = reqwest::Url::parse("mqtts://abc.com:392");

        println!("laa{:?}", obc);
        //assert!(obc.is_ok());
        //assert!(!obc.is_ok());
    }
}