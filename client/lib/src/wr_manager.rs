use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};
use crate::config::{Config, Identity, NetworkInfo};
use crate::device::peer::AllowedIP;
use crate::protobuf::config::{Protocol, WrConfig, NodeType};
use crate::device::Device;
use crate::device::script_run::Scripts;

//WireGuard Manager
// rewrite boring/Device, mainly change thread pool to tokio.
pub struct WRManager {
    device: Option<Device>,
}

impl WRManager {
    pub fn new() -> Self {
        WRManager {
            device: None,
        }
    }

    pub async fn remove_peer(&mut self, public_key: &x25519_dalek::PublicKey) {
        if let Some(device) = &mut self.device {
            device.remove_peer(public_key).await;
        } else {
            tracing::warn!("there's no active device when remove peer")
        }
    }

    pub async fn add_peer(&mut self,
                          pub_key: x25519_dalek::PublicKey,
                          endpoint: Option<SocketAddr>,
                          allowed_ips: &[AllowedIP],
                          ip:IpAddr,
                          keepalive: Option<u16>) {
        if let Some(device) = &mut self.device {
            device.update_peer(
                pub_key,
                false,
                endpoint,
                allowed_ips,
                keepalive,
                ip,
                None,
            ).await;
        } else {
            tracing::warn!("there's no active device when add/update peer")
        }
    }


    pub async fn start(&mut self, network_token_id:String, config: &Config, wr_config: WrConfig) -> anyhow::Result<()> {
        let interface = wr_config.interface.unwrap();
        //let address = AllowedIP::from_str(interface.address.as_str()).map_err(|e| anyhow!(e))?;
        let mut address: Vec<AllowedIP> = Vec::new();

        for addr in &interface.address {
            address.push(AllowedIP::from_str(addr).map_err(|e| anyhow!(e))?);
        }

        //TODO: check if need restart
        // if interface not equal, restart
        // check peers, remove or add new ones.
        let has_alive = self.is_alive();
        if has_alive {
            let node_type = self.device.as_ref().map(|x|x.node_type).unwrap_or(NodeType::NodeClient);
            tracing::info!("close device");
            self.close().await;
            let  sleep_time = if node_type == NodeType::NodeRelay {10} else {20};
            tokio::time::sleep(Duration::from_secs(sleep_time)).await;
        }

        let tun_name = config.get_tun_name(&network_token_id).await;
        let protocol = Protocol::from_i32(interface.protocol).unwrap_or(Protocol::Udp);
        let node_type = NodeType::from_i32(wr_config.r#type).unwrap();

        let scripts = Scripts::load_from_interface(&interface);
        let key_pair = (config.identity.x25519_sk.clone(), config.identity.x25519_pk.clone());
        tracing::debug!("begin to start device");
        let wr_interface = Device::new(
            tun_name,
            &address,
            key_pair,
            Some(interface.listen_port as u16),
            interface.mtu.unwrap_or(1420) as u32,
            scripts,
            protocol,
            node_type,
        )?;

        {
            let mut need_save = false;
            let server_config = config.server_config.clone();
            let mut server_config = server_config.write().await;
            if server_config.info.iter().find(|x| &x.network_id == &network_token_id).is_some() {
                for v in server_config.info.iter_mut() {
                    if v.network_id == network_token_id {
                        let old = v.tun_name.clone();
                        v.tun_name = Some(wr_interface.name.clone());
                        need_save = old != v.tun_name;
                        break;
                    }
                }
            } else {
                server_config.info.push(NetworkInfo {
                    tun_name: Some(wr_interface.name.clone()),
                    network_id: network_token_id
                });
                need_save = true;
            }

            if need_save {
                let _ = server_config.save_config(&config.config_path);
            }
        }

        self.device = Some(wr_interface);
        for peer in wr_config.peers {
            let (x_pub_key,_) = Identity::get_pub_identity_from_base64(&peer.public_key)?;
            let endpoint = peer.endpoint.map(|v| SocketAddr::from_str(&v).unwrap());
            let allowed_ip:Vec<AllowedIP> = peer.allowed_ip.into_iter().map(|ip| AllowedIP::from_str(&ip).unwrap()).collect();
            let ip:IpAddr = peer.address.first().unwrap().parse().unwrap();
            self.add_peer(
                x_pub_key,
                endpoint,
                allowed_ip.as_slice(),
                ip,
                Some(peer.persistence_keep_alive as u16),
            ).await;
            tracing::debug!("peer: {} join network", peer.public_key);
        }
        Ok(())
    }

    pub fn is_alive(&self) -> bool { self.device.is_some() }

    pub async fn close(&mut self) {
        if let Some(ref mut device) = self.device.take() {
            device.close().await
        }
    }

    pub fn device_info(&self) -> Vec<DeviceInfoResp> {
        self.device.as_ref().map_or(vec![], |device| {
            vec![DeviceInfoResp {
                name: device.name.clone()
            }]
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInfoResp {
    pub name: String,
}