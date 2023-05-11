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
use crate::device::udp_network::create_udp_socket;



pub struct Device {
    pub device_data: DeviceData,
    read_task:JoinHandle<()>,
    write_task:JoinHandle<()>,
}

impl Device {
    pub fn new(
        name: &str,
        address: &[AllowedIP],
        //allowed_ip: &[AllowedIP],
        key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
        port: Option<u16>,
        mtu: u32,
        pub_key: String,
        scripts:Scripts,
    ) -> anyhow::Result<Self>{
        run_opt_script(&scripts.pre_up)?;

        let (mut iface_reader, iface_writer, name) = create_async_tun(name, mtu, address)?;

        let udp4 = Arc::new(create_udp_socket(port, Domain::IPV4, None)?);

        let port = udp4.local_addr()?.port();
        let udp6 = Arc::new(create_udp_socket(Some(port), Domain::IPV6, None)?);


        let rate_limiter = Arc::new(RateLimiter::new(&key_pair.1, HANDSHAKE_RATE_LIMIT));
        let peers: Arc<RwLock<Peers>> = Arc::new(RwLock::new(Peers::default()));

        let read_task = read_tun(iface_reader, peers.clone(), udp4.clone(), udp6.clone());
        let write_task = udp_and_timer(key_pair.clone(), peers.clone(), rate_limiter.clone(), udp4.clone(), udp6.clone(), Arc::new(Mutex::new(iface_writer)));
        let device = Self {
            device_data:DeviceData::new(name, peers, key_pair, port, scripts),
            read_task,
            write_task,
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

fn read_tun(mut read_tun: ReadPart, peers: Arc<RwLock<Peers>>, udp4: Arc<UdpSocket>, udp6: Arc<UdpSocket>) -> JoinHandle<()> {
    return tokio::spawn(async move {
        let mut dst_buf: Vec<u8>= vec![0; MAX_UDP_SIZE];
        let mut src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];

        while let Ok(size) = read_tun.read(&mut src_buf) {
            device::tun_read_handle(&peers, &udp4, &udp6, &src_buf[..size], &mut dst_buf).await;
        }
    });
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


//auto start when server up
pub fn config_start_up(auto:bool) {

}

pub fn check_permission() -> bool {
    true
}