use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use boringtun::noise::rate_limiter::RateLimiter;
use socket2::Domain;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;
use crate::device;
use crate::device::{DeviceData, Peers};
use crate::device::{HANDSHAKE_RATE_LIMIT, MAX_UDP_SIZE};
use crate::device::peer::AllowedIP;
use crate::device::tun::{create_async_tun, ReadPart, WritePart};
use crate::device::script_run::{run_opt_script, Scripts};
use crate::device::tunnel::{create_tcp_server, create_udp_socket};
use crate::protobuf::config::{NodeType, Protocol};


pub struct Device {
    pub device_data: DeviceData,
    read_task:JoinHandle<()>,
    write_task:JoinHandle<()>,
    pub protocol: Protocol,
}

impl Device {
    pub fn new(
        name: &str,
        address: &[AllowedIP],
        //allowed_ip: &[AllowedIP],
        key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
        port: Option<u16>,
        mtu: u32,
        scripts:Scripts,
        protocol: Protocol,
        node_type:NodeType,
        driver_path: String,
    ) -> anyhow::Result<Self>{
        run_opt_script(&scripts.pre_up)?;

        let (iface_reader, iface_writer, name) = create_async_tun(name, mtu, address, driver_path)?;
        let iface_writer = Arc::new(Mutex::new(iface_writer));

        let rate_limiter = Arc::new(RateLimiter::new(&key_pair.1, HANDSHAKE_RATE_LIMIT));
        let peers: Arc<RwLock<Peers>> = Arc::new(RwLock::new(Peers::default()));
        let peers1 = peers.clone();
        let key_pair1 = key_pair.clone();
        let (read_task, write_task, port) = match protocol {
            Protocol::Udp => {
                let udp4 = Arc::new(create_udp_socket(port, Domain::IPV4, None)?);
                let port = udp4.local_addr()?.port();
                let udp6 = Arc::new(create_udp_socket(Some(port), Domain::IPV6, None)?);

                let read_task = udp_read_tun(iface_reader, peers.clone(), udp4.clone(), udp6.clone());
                let write_task = udp_and_timer(key_pair.clone(), peers.clone(), rate_limiter.clone(), udp4.clone(), udp6.clone(), iface_writer);
                (read_task, write_task, port)
            },
            Protocol::Tcp => {
                let ip = address[0].addr.clone();
                let tcp6 = create_tcp_server(port, Domain::IPV6, None)?;
                let port = tcp6.local_addr()?.port();
                let key_pair = Arc::new(key_pair);
                let read_task = tcp_read_tun(iface_reader, peers.clone());
                let write_task:JoinHandle<()> = tokio::spawn(async move {
                    loop {
                        tokio::select! {
                            _ = device::rate_limiter_timer(&rate_limiter) => {}
                            _ = device::tcp_peers_timer(
                                &ip,
                                &peers,
                                key_pair.clone(),
                                rate_limiter.clone(),
                                iface_writer.clone(),
                                false,
                                node_type,
                            ) => {}
                            _ = device::tcp_listener_handler(&tcp6, key_pair.clone(), rate_limiter.clone(), Arc::clone(&peers), iface_writer.clone(), false) => {
                                break;
                            }
                        }
                    }
                });
                (read_task, write_task, port)
            }
        };

        //let read_task = read_tun(iface_reader, peers.clone(), udp4.clone(), udp6.clone());
        //let write_task = udp_and_timer(key_pair.clone(), peers.clone(), rate_limiter.clone(), udp4.clone(), udp6.clone(), Arc::new(Mutex::new(iface_writer)));
        let device = Self {
            device_data:DeviceData::new(name, peers1, key_pair1, port, scripts, node_type),
            read_task,
            write_task,
            protocol,
        };
        run_opt_script(&device.scripts.post_up)?;
        Ok(device)
    }

    pub async fn close(&mut self) {
        self.read_task.abort();
        self.write_task.abort();
        self.device_data.close().await;
    }
}

fn udp_read_tun(mut read_tun: ReadPart, peers: Arc<RwLock<Peers>>, udp4: Arc<UdpSocket>, udp6: Arc<UdpSocket>) -> JoinHandle<()> {
    return tokio::spawn(async move {
        let mut dst_buf: Vec<u8>= vec![0; MAX_UDP_SIZE];
        let mut src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];

        while let Ok(size) = read_tun.read(&mut src_buf) {
            device::tun_read_handle(&peers, &udp4, &udp6, &src_buf[..size], &mut dst_buf).await;
        }
    });
}

fn tcp_read_tun(mut read_tun: ReadPart, peers: Arc<RwLock<Peers>>) ->JoinHandle<()>{
    return tokio::spawn(async move {
        let mut dst_buf: Vec<u8>= vec![0; MAX_UDP_SIZE];
        let mut src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
        while let Ok(size) = read_tun.read(&mut src_buf) {
            device::tun_read_tcp_handle(&peers, &src_buf[..size], &mut dst_buf).await;
        }
    })
}

fn udp_and_timer(
    key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
    peers: Arc<RwLock<Peers>>,
    rate_limiter: Arc<RateLimiter>, udp4: Arc<UdpSocket>,
    udp6: Arc<UdpSocket>,
    tun_writer: Arc<Mutex<WritePart>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let peers1 = peers.clone();
        loop {
            tokio::select! {
                _ = device::rate_limiter_timer(&rate_limiter) => {},
                _ = device::peers_timer(&peers1, &udp4,&udp6) => {},
                _  = device::udp_handler(&udp4, &key_pair, &rate_limiter, peers.clone(), tun_writer.clone(),false) => {break},
                _ = device::udp_handler(&udp6, &key_pair, &rate_limiter, peers.clone(),  tun_writer.clone(),false) => {break},
            }
        }
    })
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


pub fn check_permission() -> bool {
    true
}