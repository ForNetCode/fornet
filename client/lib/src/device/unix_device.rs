use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;
use boringtun::noise::rate_limiter::RateLimiter;
use socket2::Domain;
use tokio::sync::{Mutex, RwLock};
use tokio::io::{AsyncReadExt};
use tokio::task::JoinHandle;
use crate::device;
use crate::device::{DeviceData, Peers, HANDSHAKE_RATE_LIMIT, MAX_UDP_SIZE};
use crate::device::peer::AllowedIP;
use crate::device::script_run::{run_opt_script, run_opt_script_with_param, Scripts};
use crate::device::tun::create_async_tun;
use crate::device::tunnel::{create_tcp_server, create_udp_socket};
use nix::unistd::Uid;
use crate::protobuf::config::{Protocol, NodeType};

pub struct Device {
    pub device_data:DeviceData,
    task:JoinHandle<()>,
    protocol:Protocol,
}

impl Device {
    pub fn new(
        name: Option<String>,
        address:&[AllowedIP],
        //allowed_ip: &[AllowedIP],
        key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
        port: Option<u16>,
        mtu: u32,
        scripts:Scripts,
        protocol: Protocol,
        node_type: NodeType,
    ) -> anyhow::Result<Device> {
        run_opt_script(&scripts.pre_up)?;
        tracing::debug!("begin to create tun");
        let (mut iface_reader, iface_writer,pi, name) = create_async_tun(name, mtu, address)?;
        tracing::debug!("finish to create tun");
        let iface_writer = Arc::new(Mutex::new(iface_writer));
        let rate_limiter = Arc::new(RateLimiter::new(&key_pair.1, HANDSHAKE_RATE_LIMIT));
        let peers: Arc<RwLock<Peers>> = Arc::new(RwLock::new(Peers::default()));

        let mut tun_src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
        let mut tun_dst_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
        let key_pair1 = key_pair.clone();
        let peers1 = peers.clone();
        // create tcp/udp server
        let (port,task) = match protocol {
            Protocol::Udp => {
                let udp4 = create_udp_socket(port, Domain::IPV4, None)?;
                let port = udp4.local_addr()?.port();
                let udp6 = create_udp_socket(Some(port), Domain::IPV6, None)?;
                let task:JoinHandle<()> = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                    _ = device::rate_limiter_timer(&rate_limiter) => {}
                    _ = device::peers_timer(&peers,&udp4, &udp6) => {}
                    // iface listen
                    Ok(len) = iface_reader.read(&mut tun_src_buf) => {
                            let src_buf = if pi {
                                &tun_src_buf[4..(len+4)]
                            } else {
                                &tun_src_buf[0..len]
                            };
                            device::tun_read_handle(&peers, &udp4, &udp6, src_buf, &mut tun_dst_buf).await;
                        }
                    // udp listen
                    _ =  device::udp_handler(&udp4, &key_pair, rate_limiter.as_ref(), Arc::clone(&peers), Arc::clone(&iface_writer), pi) => break,
                    _ =  device::udp_handler(&udp6, &key_pair, rate_limiter.as_ref(), Arc::clone(&peers), Arc::clone(&iface_writer), pi) => break,
                }
                    }

                });
                (port, task)
            }
            Protocol::Tcp => {
                let ip = address[0].addr.clone();
                let tcp6 = create_tcp_server(port, Domain::IPV6, None)?;
                let port = tcp6.local_addr()?.port();
                let key_pair = Arc::new(key_pair);

                let task:JoinHandle<()> = tokio::spawn(async move {
                    loop {

                        tokio::select! {
                            _ = device::rate_limiter_timer(&rate_limiter) => {}
                            _ = device::tcp_peers_timer(
                                &ip,
                                &peers,
                                key_pair.clone(),
                                rate_limiter.clone(),
                                iface_writer.clone(),
                                pi,
                                node_type,
                            ) => {}
                            // iface listen
                            Ok(len) = iface_reader.read(&mut tun_src_buf) => {
                                if len > 0 {
                                    let src_buf = if pi {
                                        &tun_src_buf[4..(len+4)]
                                    } else {
                                        &tun_src_buf[0..len]
                                    };
                                    device::tun_read_tcp_handle(&peers, src_buf, &mut tun_dst_buf).await;
                                }
                            }
                            //_ = device::tcp_listener_handler(&tcp4, key_pair.clone(), rate_limiter.clone(), Arc::clone(&peers), Arc::clone(&iface_writer), pi) => {break}
                            _ = device::tcp_listener_handler(&tcp6, key_pair.clone(), rate_limiter.clone(), Arc::clone(&peers), Arc::clone(&iface_writer), pi) => {
                                break;
                            }

                        }
                    }
                });
                (port, task)
            }
        };
        let device = Device {
            device_data: DeviceData::new(name,peers1, key_pair1, port, scripts, node_type,
            ),
            task,
            protocol,
        };


        let mut script_params:HashMap<&str, String> = HashMap::new();
        script_params.insert("tun", device.name.clone());
        run_opt_script_with_param(&device.scripts.post_up, &script_params)?;
        Ok(device)
    }

    pub async fn close(&mut self) {
        self.task.abort();// close all connections.
        //tracing::debug!("tun/rpc task is finish: {}", self.task.is_finished());
        self.device_data.close().await;
        // task abort would cost some time.
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

impl Deref for Device {
    type Target = DeviceData;

    fn deref(&self) -> &Self::Target {
        &self.device_data
    }
}

impl DerefMut for Device {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device_data
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.task.is_finished() {
            self.task.abort();
        }
        tracing::debug!("device has been dropped");
    }
}

//auto start when server up
pub fn check_permission() -> bool {
    Uid::effective().is_root()
}
