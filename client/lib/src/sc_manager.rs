use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use anyhow::{anyhow, bail};
use async_trait::async_trait;
use mqrstt::{AsyncEventHandler, ConnectOptions, MqttClient, new_tokio};
use mqrstt::packets::{Packet, QoS, SubscriptionOptions};

use prost::Message;
use tokio_rustls::rustls::{ClientConfig, ServerName};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};


use tokio_stream::StreamExt;
use tracing::instrument::WithSubscriber;
use windows::Win32::Networking::WinSock::send;
use crate::client_manager::ForNetClient;
use crate::config::{Config as AppConfig, ServerConfig};

use crate::protobuf::config::{ClientMessage, NetworkMessage, NetworkStatus, NodeStatus, WrConfig};
use crate::protobuf::config::client_message::Info::{Config, Status};
use crate::protobuf::config::network_message::Info::{Peer, Status as NStatus};
use crate::server_manager::ServerMessage;

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
struct MqttWrapper2 {
    pub mqtt_client: MqttClient,
    pub deduplication: Duplication,
    pub sender: Sender<ServerMessage>,
    pub network_topics: Vec<String>,
    pub client_topic:String,
}
impl MqttWrapper2 {

}

#[async_trait]
impl AsyncEventHandler for MqttWrapper2 {
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
                                        let network_topic = format!("network/{}", &client_message.network_id);
                                        if !self.network_topics.contains(&network_topic) {
                                            self.network_topics.push( network_topic.clone());
                                            let _ = self.mqtt_client.subscribe((network_topic, SubscriptionOptions{
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
                                                ServerMessage::StopWR{network_id: network_message.network_id.clone(),
                                                     reason:"network has been delete".to_owned(), delete_network: true }
                                            ).await;
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

struct MqttWrapper<'a> {
    pub client:MqttClient,
    pub client_topic: String,
    pub network_topics: Vec<String>,
    //TODO: Duplication diff by network_token_id, and use timestamp to diff would be more effect and good.
    pub deduplication: &'a mut Duplication,
    pub sender: Sender<ServerMessage>,
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
                                        let network_topic = format!("network/{}", &client_message.network_id);
                                        if !self.network_topics.contains(&network_topic) {
                                            self.network_topics.push( network_topic.clone());
                                            let _ = self.client.subscribe((network_topic, SubscriptionOptions{
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
                                                ServerMessage::StopWR{network_id: network_message.network_id.clone(),
                                                    reason:"network has been delete".to_owned(), delete_network: true }
                                            ).await;
                                            let d = self.client.unsubscribe(topic.clone()).await;

                                            self.network_topics = self.network_topics.iter().filter_map(|x| {
                                                if x != topic {
                                                    Some(x.clone())
                                                }else {
                                                    None
                                                }
                                            }).collect();
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

    async fn mqtt_reconnect(sender:Sender<ServerMessage>, config:Arc<AppConfig>, deduplication: &mut Duplication) -> anyhow::Result<()> {
        let server_config = config.server_config.clone();
        let server_config = server_config.read().await;
        let url = reqwest::Url::parse(&server_config.mqtt_url.clone())?;

        let host = url.host_str().unwrap_or("");
        let port = url.port_or_known_default().unwrap_or(1883);// secret: 8883
        let username = config.identity.pk_base64.clone();
        let mut options = ConnectOptions::new(server_config.device_id.clone());
        let encrypt = config.identity.sign(vec![server_config.device_id.clone()])?;
        let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);
        options.password = Some(password);
        options.username = Some(username);

        let client_topic = format!("client/{}",&server_config.device_id);
        let network_topics:Vec<String> = server_config.info.iter().map(|info| format!("network/{}", &info.network_id)).collect();

        let subscribe_topics:Vec<(String, SubscriptionOptions)> = [vec![client_topic.clone()], network_topics.clone()].concat().into_iter().map(|topic| (topic, SubscriptionOptions{
            qos: QoS::AtLeastOnce,
            ..Default::default()
        })).collect();

        //let subscribe_topics:&[(&str, QoS)] = &[(&client_topic.clone(),QoS::AtLeastOnce ), (&network_topic.clone(), QoS::AtLeastOnce)];

        //let deduplication =  Duplication::default();


        if url.scheme() == "mqtts" {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                client:client.clone(),
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

            //let connection = connector.connect(domain, stream).await?;

            //let cx = tokio_rustls::TlsConnector::builder().build()?;
            //let cx = tokio_native_tls::TlsConnector::from(cx);
            //let stream = tokio::net::TcpStream::connect((host, port)).await?;
            //let connection = cx.connect(host, stream).await?;

            network.connect(connection,&mut mqtt_wrapper).await?;
            client.subscribe(subscribe_topics).await?;
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);
                        break
                    }
                }
            }

        } else {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                client,
                client_topic,
                network_topics,
                deduplication,
                sender,
            };
            let stream = tokio::net::TcpStream::connect((host, port)).await?;
            network.connect(stream, &mut mqtt_wrapper).await?;

            mqtt_wrapper.client.subscribe(subscribe_topics).await?;
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
        let mut deduplication = Duplication {
            wr_config: None,
            status: None,
        };
        let _config = config.clone();
        let _sender = self.sender.clone();
        tokio::spawn(async move{
            loop {
                let _config = _config.clone();
                let _sender = _sender.clone();
                let v = SCManager::mqtt_reconnect(_sender, _config, &mut deduplication).await;
                tracing::debug!("mqtt connect error, {:?}", v);
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        });
        Ok(())
    }
}

pub struct ConfigSyncManager {
    client_manager: Arc<RwLock<ForNetClient>>,
    mqtt_connections: Arc<Mutex<HashMap<String, MqttClient>>>,
    sender: Sender<ServerMessage>,

}

impl ConfigSyncManager {
    pub fn new(client_manager: Arc<RwLock<ForNetClient>>) -> (Self,Receiver<ServerMessage>) {
        let (sender, mut rx) = tokio::sync::mpsc::channel::<ServerMessage>(32);
        (ConfigSyncManager {
            client_manager,
            mqtt_connections:Arc::new(Mutex::new(HashMap::default())),
            sender,
        },rx)
    }
    pub async fn connect(&mut self, mqtt_url: String) -> anyhow::Result<()>  {
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

        tokio::spawn(async move  {
           self.connect_mqtt(mqtt_url,deduplication, sender, client_manager).await;
        });
        Ok(())
    }

    async fn connect_mqtt(&mut self, mqtt_url: String, deduplication:Duplication, sender: Sender<ServerMessage>, client_manager:Arc<RwLock<ForNetClient>>) -> anyhow::Result<Duplication> {
        let (options,device_id, network_ids) = {
            let app_config = &client_manager.read().await.config;
            let info = app_config.local_config.server_info.iter().find_map(|info | {
                if &info.mqtt_url == &mqtt_url {
                    Some((info.device_id.clone(), info.network_id.clone()))
                }else {
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
            }else {
                anyhow::bail!("There's no mqtt connection: {}", &mqtt_url)
            }
        };

        let url = reqwest::Url::parse(&mqtt_url)?;
        let host = url.host_str().unwrap_or("");
        let port = url.port_or_known_default().unwrap_or(1883);// secret: 8883
        let client_topic = format!("client/{}",device_id);
        let network_topics:Vec<String> = network_ids.into_iter().map(|network_id| format!("network/{}", &network_id)).collect();

        let subscribe_topics:Vec<(String, SubscriptionOptions)> = [vec![client_topic.clone()], network_topics.clone()].concat().into_iter().map(|topic| (topic, SubscriptionOptions{
            qos: QoS::AtLeastOnce,
            ..Default::default()
        })).collect();


        let mqtt_wrapper = if url.scheme() == "mqtts" {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper2 {
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

            network.connect(connection,&mut mqtt_wrapper).await?;
            client.subscribe(subscribe_topics).await?;
            //self.mqtt_connection.insert(mqtt_url.clone(), mqtt_wrapper);
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);

                        break
                    }
                }
            }
            mqtt_wrapper
        } else {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper2 {
                mqtt_client: client.clone(),
                network_topics,
                client_topic,
                deduplication,
                sender,
            };
            let stream = tokio::net::TcpStream::connect((host, port)).await?;
            network.connect(stream, &mut mqtt_wrapper).await?;

            mqtt_wrapper.mqtt_client.subscribe(subscribe_topics).await?;
            //self.mqtt_connection.insert(mqtt_url.clone(), mqtt_wrapper);
            loop {
                match network.poll(&mut mqtt_wrapper).await {
                    Ok(mqrstt::tokio::NetworkStatus::Active) => {
                        continue
                    }
                    other => {
                        tracing::debug!("mqtt network status {:?}", other);
                        break
                    }
                }
            }
            mqtt_wrapper

        };
        //self.mqtt_connection.insert(mqtt_url, mqtt_wrapper.mqtt_client);
        //mqtt_wrapper.mqtt_client.clone();
        Ok(mqtt_wrapper.deduplication)

        //tokio::time::sleep(Duration::from_secs(20)).await;
        //self.connect_mqtt(mqtt_url).await?;

    }


    pub async fn disconnect_mqtt(&mut self, mqtt_url:String) {

    }

    /*
    pub async fn mqtt_connect(&self, server_config:ServerConfig) -> anyhow::Result<()> {
        let mut deduplication = Duplication {
            wr_config: None,
            status: None,
        };
        let _config = self.client_manager.config.clone();

        tokio::spawn(async move {
            loop {
                let _config = _config.clone();
                let v = self.mqtt_reconnect(_config, server_config, &mut deduplication).await;
                tracing::debug!("mqtt connect error, {:?}", v);
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        });
        Ok(())
    }
    async fn mqtt_reconnect(&self, config:Arc<AppConfig>, server_config:ServerConfig, deduplication: &mut Duplication) ->anyhow::Result<()> {
        let url = reqwest::Url::parse(&server_config.mqtt_url.clone())?;
        let host = url.host_str().unwrap_or("");
        let port = url.port_or_known_default().unwrap_or(1883);// secret: 8883
        let username = config.identity.pk_base64.clone();
        let mut options = ConnectOptions::new(server_config.device_id.clone());
        let encrypt = config.identity.sign(vec![server_config.device_id.clone()])?;
        let password = format!("{}|{}|{}", encrypt.nonce, encrypt.timestamp, encrypt.signature);
        options.password = Some(password);
        options.username = Some(username);

        let client_topic = format!("client/{}",&server_config.device_id);
        let network_topics:Vec<String> = server_config.info.iter().map(|info| format!("network/{}", &info.network_id)).collect();

        let subscribe_topics:Vec<(String, SubscriptionOptions)> = [vec![client_topic.clone()], network_topics.clone()].concat().into_iter().map(|topic| (topic, SubscriptionOptions{
            qos: QoS::AtLeastOnce,
            ..Default::default()
        })).collect();

        if url.scheme() == "mqtts" {
            let (mut network, client) = new_tokio(options);
            let mut mqtt_wrapper = MqttWrapper {
                client:client.clone(),
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

            //let connection = connector.connect(domain, stream).await?;

            //let cx = tokio_rustls::TlsConnector::builder().build()?;
            //let cx = tokio_native_tls::TlsConnector::from(cx);
            //let stream = tokio::net::TcpStream::connect((host, port)).await?;
            //let connection = cx.connect(host, stream).await?;

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
                network_topics,
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

    }*/
}

#[cfg(test)]
mod test {
    #[test]
    pub fn test_mqtt_reconnect() {

    }
}