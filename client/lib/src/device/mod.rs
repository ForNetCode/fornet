mod allowed_ips;
pub mod peer;
mod tunnel;
mod tun;
pub mod auto_launch;
pub mod script_run;

cfg_if! {
     if #[cfg(target_os="windows")] {
        mod windows_device;
        pub use windows_device::{Device, check_permission};
    } else {
        mod unix_device;
        pub use unix_device::{Device, check_permission};
    }

}

use std::collections::HashMap;
use cfg_if::cfg_if;
use rand::RngCore;
use rand::rngs::OsRng;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use boringtun::noise::errors::WireGuardError;
use boringtun::noise::rate_limiter::RateLimiter;
use boringtun::noise::{Packet, Tunn, TunnResult};
use boringtun::noise::handshake::parse_handshake_anon;
use prost::bytes::BufMut;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::{Mutex, RwLock};
use tokio::time;
use tokio::io::{AsyncReadExt, AsyncWriteExt};//keep
use tokio::net::tcp::{OwnedReadHalf, ReadHalf, WriteHalf};

use allowed_ips::AllowedIps;
use peer::{AllowedIP, Peer};
use script_run::Scripts;
use crate::device::script_run::run_opt_script;
use self::tun::WritePart;

const HANDSHAKE_RATE_LIMIT: u64 = 100; // The number of handshakes per second we can tolerate before using cookies

const MAX_UDP_SIZE: usize = (1 << 16) - 1;
const MAX_TCP_SIZE: usize = (1 << 16) -1;
// const MAX_ITR: usize = 100; // Number of packets to handle per handler call

#[derive(Debug)]
pub enum Error {
    Socket(String),
    Bind(String),
    FCntl(String),
    EventQueue(String),
    IOCtl(String),
    Connect(String),
    SetSockOpt(String),
    InvalidTunnelName,
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    GetSockOpt(String),
    GetSockName(String),
    UDPRead(i32),
    #[cfg(target_os = "linux")]
    Timer(String),
    IfaceRead(i32),
    DropPrivileges(String),
    ApiSocket(std::io::Error),
}

/// A basic linear-feedback shift register implemented as xorshift, used to
/// distribute peer indexes across the 24-bit address space reserved for peer
/// identification.
/// The purpose is to obscure the total number of peers using the system and to
/// ensure it requires a non-trivial amount of processing power and/or samples
/// to guess other peers' indices. Anything more ambitious than this is wasted
/// with only 24 bits of space.
struct IndexLfsr {
    initial: u32,
    lfsr: u32,
    mask: u32,
}

impl IndexLfsr {
    /// Generate a random 24-bit nonzero integer
    fn random_index() -> u32 {
        const LFSR_MAX: u32 = 0xffffff; // 24-bit seed
        loop {
            let i = OsRng.next_u32() & LFSR_MAX;
            if i > 0 {
                // LFSR seed must be non-zero
                return i;
            }
        }
    }

    /// Generate the next value in the pseudorandom sequence
    fn next(&mut self) -> u32 {
        // 24-bit polynomial for randomness. This is arbitrarily chosen to
        // inject bitflips into the value.
        const LFSR_POLY: u32 = 0xd80000; // 24-bit polynomial
        let value = self.lfsr - 1; // lfsr will never have value of 0
        self.lfsr = (self.lfsr >> 1) ^ ((0u32.wrapping_sub(self.lfsr & 1u32)) & LFSR_POLY);
        assert!(self.lfsr != self.initial, "Too many peers created");
        value ^ self.mask
    }
}

impl Default for IndexLfsr {
    fn default() -> Self {
        let seed = Self::random_index();
        IndexLfsr {
            initial: seed,
            lfsr: seed,
            mask: Self::random_index(),
        }
    }
}



cfg_if! {
    if #[cfg(target_os="windows")] {
        // This would used
        //windows driver: ws2def.h
        const IP4_HEADER: [u8; 4] = [0, 0, 0,  windows::Win32::Networking::WinSock::AF_INET.0 as u8];// AF_INET
        const IP6_HEADER: [u8; 4] = [0, 0, 0, windows::Win32::Networking::WinSock::AF_INET6.0 as u8];// AF_INET6
    } else {
        const IP4_HEADER: [u8; 4] = [0, 0, 0, libc::PF_INET as u8];
        const IP6_HEADER: [u8; 4] = [0, 0, 0, libc::PF_INET6 as u8];
    }
}

pub struct DeviceData {
    pub name: String,
    pub peers: Arc<RwLock<Peers>>,
    next_index: IndexLfsr,
    pub key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
    pub listen_port: u16,
    pub scripts: Scripts,
}

impl DeviceData {
    pub fn new(name: String,
               peers: Arc<RwLock<Peers>>,
               key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
               listen_port: u16,
               scripts:Scripts,
    ) -> Self {
        Self {
            name,
            peers,
            next_index: Default::default(),
            key_pair,
            listen_port,
            scripts,
        }
    }
    pub fn next_index(&mut self) -> u32 {
        self.next_index.next()
    }

    pub async fn remove_peer(&mut self, pub_key: &x25519_dalek::PublicKey) {
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.by_key.remove(pub_key) {
            let mut p = peer.lock().await;
            p.shutdown_endpoint();
            peers.by_idx.remove(&p.index());
            peers.by_ip.remove(&|p: &Arc<Mutex<Peer>>| Arc::ptr_eq(&peer, p));
            //crate::device::tun::sys::remove_route()
            tracing::info!("Peer removed");
        }
    }


    pub async fn update_peer(
        &mut self,
        pub_key: x25519_dalek::PublicKey,
        _replace_ips: bool,
        endpoint: Option<SocketAddr>,
        allowed_ips: &[AllowedIP],
        keepalive: Option<u16>,
        preshared_key: Option<[u8; 32]>,
    ) {
        // Update an existing peer
        if self.peers.read().await.by_key.get(&pub_key).is_some() {
            // We already have a peer, we need to merge the existing config into the newly created one
            panic!("Modifying existing peers is not yet supported. Remove and add again instead.");
        }

        let next_index = self.next_index();
        let device_key_pair = &self.key_pair;

        let tunn = Tunn::new(
            device_key_pair.0.clone(),
            pub_key,
            preshared_key,
            keepalive,
            next_index,
            None,
        )
            .unwrap();

        let peer = Peer::new(tunn, next_index, endpoint, allowed_ips, preshared_key);
        let peer = Arc::new(Mutex::new(peer));
        let mut peers = self.peers.write().await;

        peers.by_key.insert(pub_key, Arc::clone(&peer));
        peers.by_idx.insert(next_index, Arc::clone(&peer));

        for AllowedIP { addr, cidr } in allowed_ips {
            peers.by_ip
                .insert(*addr, *cidr as _, Arc::clone(&peer));
        }
        tracing::info!("Peer added");
    }

    pub async fn close(&mut self) {
        let _ = run_opt_script(&self.scripts.pre_down);
        let mut peers = self.peers.write().await;
        peers.by_idx.clear();
        peers.by_ip.clear();
        peers.by_key.clear();
    }
}

impl Drop for DeviceData {
    fn drop(&mut self) {
        let _ = run_opt_script(&self.scripts.post_down);
    }
}

pub async fn tun_read_handle(peers: &Arc<RwLock<Peers>>, udp4: &UdpSocket, udp6: &UdpSocket, src_buf: &[u8], dst_buf: &mut [u8]) {
    //tracing::debug!("tun read:{:x?},{:?}", Tunn::dst_address(src_buf), src_buf);
    if let Some(dst_addr) = Tunn::dst_address(src_buf) {
        if let Some(peer) = peers.read().await.by_ip.find(dst_addr) {
            let mut peer = peer.lock().await;
            match peer.tunnel.encapsulate(src_buf, &mut dst_buf[..]) {
                TunnResult::Done => {
                    // tracing::debug!("done");
                }
                TunnResult::Err(e) => {
                    tracing::error!(message = "Encapsulate error", error = ?e)
                }
                TunnResult::WriteToNetwork(packet) => {
                    let endpoint = peer.endpoint();
                    if let Some(addr @ SocketAddr::V4(_)) = endpoint.addr {
                        //tracing::debug!("send:{}, size:{}",addr,packet.len());
                        let _ = udp4.send_to(packet, addr).await;
                    } else if let Some(addr @ SocketAddr::V6(_)) = endpoint.addr {
                        let _ = udp6.send_to(packet, addr).await;
                    } else {
                        tracing::error!("No endpoint");
                    }
                    //TODO: get tcp socket from peers and send
                }
                _ => panic!("Unexpected result from encapsulate"),
            };
        }
    }
}

pub async fn tun_read_tcp_handle(peers: &Arc<RwLock<Peers>>, src_buf: &[u8], dst_buf: &mut [u8]) {
    //tracing::debug!("tun read:{:x?},{:?}", Tunn::dst_address(src_buf), src_buf);
    if let Some(dst_addr) = Tunn::dst_address(src_buf) {
        if let Some(peer) = peers.read().await.by_ip.find(dst_addr) {
            let mut peer = peer.lock().await;
            match peer.tunnel.encapsulate(src_buf, &mut dst_buf[..]) {
                TunnResult::Done => {
                    // tracing::debug!("done");
                }
                TunnResult::Err(e) => {
                    tracing::error!(message = "Encapsulate error", error = ?e)
                }
                TunnResult::WriteToNetwork(packet) => {
                    let endpoint = peer.endpoint();
                    if let Some(conn) = &mut endpoint.tcp_conn {
                        let _ = conn.write_all(packet).await;
                    } else {
                        tracing::info!("no endpoint of {:?}", endpoint.addr);
                    }
                }
                _ => panic!("Unexpected result from encapsulate"),
            };
        }
    }
}

pub async fn rate_limiter_timer(rate_limiter: &Arc<RateLimiter>) {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        rate_limiter.reset_count();
    }
}

pub async fn peers_timer(peers: &Arc<RwLock<Peers>>, udp4: &UdpSocket, udp6: &UdpSocket) {
    let mut interval = time::interval(Duration::from_millis(250));
    let mut dst_buf: Vec<u8>= vec![0; MAX_UDP_SIZE];

    loop {
        interval.tick().await;
        let peer_map = &peers.read().await.by_key;
        for peer in peer_map.values() {
            let mut p = peer.lock().await;
            let endpoint_addr = match p.endpoint().addr {
                Some(addr) => addr,
                None => continue,
            };

            match p.update_timers(&mut dst_buf) {
                TunnResult::Done => {}
                TunnResult::Err(WireGuardError::ConnectionExpired) => {
                    p.shutdown_endpoint(); // close open udp socket
                }
                TunnResult::Err(e) => tracing::error!(message = "Timer error", error = ?e),
                TunnResult::WriteToNetwork(packet) => {

                    let _ = match endpoint_addr {
                        SocketAddr::V4(_) => udp4.send_to(packet, endpoint_addr).await,
                        SocketAddr::V6(_) => udp6.send_to(packet, endpoint_addr).await,
                    };
                }
                _ => panic!("Unexpected result from update_timers"),
            };
        }
    }
}

pub async fn tcp_peers_timer(peers: &Arc<RwLock<Peers>>) {
    let mut interval = time::interval(Duration::from_millis(250));
    let mut dst_buf: Vec<u8>= vec![0; MAX_UDP_SIZE];

    loop {
        interval.tick().await;
        let peer_map = &peers.read().await.by_key;
        for peer in peer_map.values() {
            let mut p = peer.lock().await;
            //TODO: if needs to create tcp when p.endpoint().addr.is_some()
            if p.endpoint().tcp_conn.is_none() {
                continue
            }
            match p.update_timers(&mut dst_buf) {
                TunnResult::Done => {}
                TunnResult::Err(WireGuardError::ConnectionExpired) => {
                    p.shutdown_endpoint(); // close open udp socket
                }
                TunnResult::Err(e) => tracing::error!(message = "Timer error", error = ?e),
                TunnResult::WriteToNetwork(packet) => {
                    if let Some(conn) = &mut p.endpoint().tcp_conn {
                        let _ = conn.write_all(packet).await;
                    }

                }
                _ => panic!("Unexpected result from update_timers"),
            };
        }
    }
}


pub async fn udp_handler(udp: &UdpSocket,
                         key_pair: &(x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
                         rate_limiter: &RateLimiter,
                         peers: Arc<RwLock<Peers>>,
                         iface: Arc<Mutex<WritePart>>,
                         pi: bool,
) {
    let mut src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
    let mut dst_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
    let (private_key, public_key) = key_pair;
    while let Ok((size, addr)) = udp.recv_from(&mut src_buf).await {
        //tracing::debug!("recv: {addr:?}, {size}");
        let parsed_packet =
            match rate_limiter.verify_packet(Some(addr.ip()), &src_buf[..size], &mut dst_buf) {
                Ok(packet) => packet,
                Err(TunnResult::WriteToNetwork(cookie)) => {
                    let _ = udp.send_to(cookie, addr).await;
                    continue;
                }
                Err(_) => continue,
            };
        let peer = match &parsed_packet {
            Packet::HandshakeInit(p) => {
                if let Ok(hh) = parse_handshake_anon(private_key, public_key, p) {
                    let by_key = &peers.read().await.by_key;
                    by_key.get(&x25519_dalek::PublicKey::from(hh.peer_static_public)).map(Arc::clone)
                } else {
                    None
                }
            }
            Packet::HandshakeResponse(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
            Packet::PacketCookieReply(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
            Packet::PacketData(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
        };
        let peer = match peer {
            None => continue,
            Some(peer) => peer,
        };

        let mut p = peer.lock().await;

        // We found a peer, use it to decapsulate the message+
        let mut flush = false; // Are there packets to send from the queue?
        match p
            .tunnel
            .handle_verified_packet(parsed_packet, &mut dst_buf[..])
        {
            TunnResult::Done => {}
            TunnResult::Err(_) => continue,
            TunnResult::WriteToNetwork(packet) => {
                flush = true;
                let _ = udp.send_to(packet, addr).await;
            }
            TunnResult::WriteToTunnelV4(packet, addr) => {
                // tracing::debug!("{addr:?}");
                if p.is_allowed_ip(addr)                                                                                                                                                                          {
                    if pi {
                        let mut buf: Vec<u8> = Vec::new();
                        buf.put_slice(&IP4_HEADER);
                        buf.put_slice(&packet);
                        cfg_if! {
                            if  #[cfg(target_os="windows")]  {
                                let _ = iface.lock().await.write(&buf);
                            } else {
                                let _ = iface.lock().await.write(&buf).await;
                            }
                        }
                    } else {
                        cfg_if! {
                            if  #[cfg(target_os="windows")]  {
                                let _ = iface.lock().await.write(&packet);
                            } else {
                                let _ = iface.lock().await.write(&packet).await;
                            }
                        }
                    }
                } else {}
            }
            TunnResult::WriteToTunnelV6(packet, addr) => {
                if p.is_allowed_ip(addr) {
                    if pi {
                        let mut buf: Vec<u8> = Vec::new();
                        buf.put_slice(&IP6_HEADER);
                        buf.put_slice(&packet);
                        cfg_if! {
                                if  #[cfg(target_os="windows")]  {
                                    let _ = iface.lock().await.write(&buf);
                                } else {
                                    let _ = iface.lock().await.write(&buf).await;
                                }
                            }
                    } else {
                        cfg_if! {
                                if  #[cfg(target_os="windows")]  {
                                    let _ = iface.lock().await.write(packet);
                                } else {
                                    let _ = iface.lock().await.write(packet).await;
                                }
                            }
                    };

                }
            }
        };

        if flush {
            // Flush pending queue

            while let TunnResult::WriteToNetwork(packet) =
                p.tunnel.decapsulate(None, &[], &mut dst_buf[..])
             {
                let _ = udp.send_to(packet, addr).await;
            }


        }
        p.set_endpoint(addr);
    }
}

pub async fn tcp_handler(mut tcp: OwnedReadHalf,
                         key_pair: &(x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
                         rate_limiter: &RateLimiter,
                         peers: Arc<RwLock<Peers>>,
                         iface: Arc<Mutex<WritePart>>,
                         pi: bool,
) {


    let mut src_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
    let mut dst_buf: Vec<u8> = vec![0; MAX_UDP_SIZE];
    let (private_key, public_key) = key_pair;

    while let Ok(size) = tcp.read(&mut src_buf).await {
        //tracing::debug!("recv: {addr:?}, {size}");
        let parsed_packet =
            match rate_limiter.verify_packet(Some(addr.ip()), &src_buf[..size], &mut dst_buf) {
                Ok(packet) => packet,
                Err(TunnResult::WriteToNetwork(cookie)) => {
                    //TODO: send
                    continue;
                }
                Err(_) => continue,
            };
        let peer = match &parsed_packet {
            Packet::HandshakeInit(p) => {
                if let Ok(hh) = parse_handshake_anon(private_key, public_key, p) {
                    let by_key = &peers.read().await.by_key;
                    by_key.get(&x25519_dalek::PublicKey::from(hh.peer_static_public)).map(Arc::clone)
                } else {
                    None
                }
            }
            Packet::HandshakeResponse(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
            Packet::PacketCookieReply(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
            Packet::PacketData(p) => peers.read().await.by_idx.get(&(p.receiver_idx >> 8)).map(Arc::clone),
        };
        let peer = match peer {
            None => continue,
            Some(peer) => peer,
        };

        let mut p = peer.lock().await;

        // We found a peer, use it to decapsulate the message+
        let mut flush = false; // Are there packets to send from the queue?
        match p
            .tunnel
            .handle_verified_packet(parsed_packet, &mut dst_buf[..])
        {
            TunnResult::Done => {}
            TunnResult::Err(_) => continue,
            TunnResult::WriteToNetwork(packet) => {
                flush = true;
                let _ = udp.send_to(packet, addr).await;
            }
            TunnResult::WriteToTunnelV4(packet, addr) => {
                // tracing::debug!("{addr:?}");
                if p.is_allowed_ip(addr)                                                                                                                                                                          {
                    if pi {
                        let mut buf: Vec<u8> = Vec::new();
                        buf.put_slice(&IP4_HEADER);
                        buf.put_slice(&packet);
                        cfg_if! {
                            if  #[cfg(target_os="windows")]  {
                                let _ = iface.lock().await.write(&buf);
                            } else {
                                let _ = iface.lock().await.write(&buf).await;
                            }
                        }
                    } else {
                        cfg_if! {
                            if  #[cfg(target_os="windows")]  {
                                let _ = iface.lock().await.write(&packet);
                            } else {
                                let _ = iface.lock().await.write(&packet).await;
                            }
                        }
                    }
                } else {}
            }
            TunnResult::WriteToTunnelV6(packet, addr) => {
                if p.is_allowed_ip(addr) {
                    if pi {
                        let mut buf: Vec<u8> = Vec::new();
                        buf.put_slice(&IP6_HEADER);
                        buf.put_slice(&packet);
                        cfg_if! {
                                if  #[cfg(target_os="windows")]  {
                                    let _ = iface.lock().await.write(&buf);
                                } else {
                                    let _ = iface.lock().await.write(&buf).await;
                                }
                            }
                    } else {
                        cfg_if! {
                                if  #[cfg(target_os="windows")]  {
                                    let _ = iface.lock().await.write(packet);
                                } else {
                                    let _ = iface.lock().await.write(packet).await;
                                }
                            }
                    };

                }
            }
        };

        if flush {
            // Flush pending queue

            while let TunnResult::WriteToNetwork(packet) =
                p.tunnel.decapsulate(None, &[], &mut dst_buf[..])
            {
                let _ = udp.send_to(packet, addr).await;
            }


        }
        p.set_endpoint(addr);
    }
}


pub struct Peers {
    pub by_key: HashMap<x25519_dalek::PublicKey, Arc<Mutex<Peer>>>,
    pub by_ip: AllowedIps<Arc<Mutex<Peer>>>,
    pub by_idx: HashMap<u32, Arc<Mutex<Peer>>>,
}

impl Default for Peers {
    fn default() -> Self {
        Peers {
            by_key: Default::default(),
            by_ip: AllowedIps::new(),
            by_idx: Default::default(),
        }
    }
}
