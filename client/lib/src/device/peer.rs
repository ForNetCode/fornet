// Copyright (c) 2019 Cloudflare, Inc. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause



use std::fmt::{Debug, Formatter};
use std::net::{IpAddr};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

use boringtun::noise::{Tunn, TunnResult};
use tokio::io::AsyncWriteExt;
use tokio::net::{UdpSocket};
use tokio::net::tcp::OwnedWriteHalf;
use crate::device::allowed_ips::AllowedIps;

#[derive(Debug)]
pub  enum TcpConnection {
    Nothing,
    Connecting(SystemTime),
    Connected(OwnedWriteHalf),
    ConnectedFailure(std::io::Error)
}
#[derive(Debug)]
pub struct Endpoint {
    pub addr: Option<SocketAddr>,
    pub udp_conn: Option<Arc<UdpSocket>>,
    pub tcp_conn: TcpConnection,
}

impl Endpoint {
    pub async fn tcp_write(&mut self, bytes:&[u8]) {
        if let TcpConnection::Connected(conn) = &mut self.tcp_conn {
            match conn.write_all(bytes).await {
                Ok(_) =>  {
                    // do nothing
                },
                Err(e) => {
                    tracing::error!("tcp conn of {:?} fail, error: {}", conn.peer_addr(), e);
                    self.tcp_conn = TcpConnection::ConnectedFailure(e);
                }
            };
        }
    }
}

pub struct Peer {
    /// The associated tunnel struct
    pub(crate) tunnel: Tunn,
    /// The index the tunnel uses
    index: u32,
    pub endpoint: Endpoint,
    allowed_ips: AllowedIps<()>,
    pub ip: IpAddr,
    preshared_key: Option<[u8; 32]>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct AllowedIP {
    pub addr: IpAddr,
    pub cidr: u8,
}

impl FromStr for AllowedIP {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ip: Vec<&str> = s.split('/').collect();
        if ip.len() != 2 {
            return Err("Invalid IP format".to_owned());
        }

        let (addr, cidr) = (ip[0].parse::<IpAddr>(), ip[1].parse::<u8>());
        match (addr, cidr) {
            (Ok(addr @ IpAddr::V4(_)), Ok(cidr)) if cidr <= 32 => Ok(AllowedIP { addr, cidr }),
            (Ok(addr @ IpAddr::V6(_)), Ok(cidr)) if cidr <= 128 => Ok(AllowedIP { addr, cidr }),
            _ => Err("Invalid IP format".to_owned()),
        }
    }
}

impl std::fmt::Display for AllowedIP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}",&self.addr, self.cidr)
    }
}

impl Peer {
    pub fn new(
        tunnel: Tunn,
        index: u32,
        endpoint: Option<SocketAddr>,
        allowed_ips: &[AllowedIP],
        ip:IpAddr,
        preshared_key: Option<[u8; 32]>,
    ) -> Peer {
        Peer {
            tunnel,
            index,
            endpoint: Endpoint {
                addr: endpoint,
                udp_conn: None,
                tcp_conn: TcpConnection::Nothing,
            },
            ip,
            allowed_ips: allowed_ips.iter().map(|ip| (ip, ())).collect(),
            preshared_key,
        }
    }

    pub fn update_timers<'a>(&mut self, dst: &'a mut [u8]) -> TunnResult<'a> {
        self.tunnel.update_timers(dst)
    }

    pub fn endpoint(&mut self) -> &mut Endpoint {
        &mut self.endpoint
    }

    pub fn shutdown_endpoint(&mut self) {
        if let Some(_) = &mut self.endpoint.udp_conn.take() {
            tracing::info!("disconnecting from endpoint");
        } else if let TcpConnection::Connected(_) = &mut self.endpoint.tcp_conn {
            tracing::info!("disconnecting tcp connection");
        }
        self.endpoint.tcp_conn = TcpConnection::Nothing;
    }

    pub fn set_endpoint(&mut self, addr: SocketAddr) {
        if self.endpoint.addr != Some(addr) {
            // We only need to update the endpoint if it differs from the current one
            self.shutdown_endpoint();
            self.endpoint.addr = Some(addr);
        };
    }


    pub fn is_allowed_ip<I: Into<IpAddr>>(&self, addr: I) -> bool {
        self.allowed_ips.find(addr.into()).is_some()
    }

    pub fn allowed_ips(&self) -> impl Iterator<Item = (IpAddr, u8)> + '_ {
        self.allowed_ips.iter().map(|(_, ip, cidr)| (ip, cidr))
    }

    pub fn time_since_last_handshake(&self) -> Option<std::time::Duration> {
        self.tunnel.time_since_last_handshake()
    }

    pub fn persistent_keepalive(&self) -> Option<u16> {
        self.tunnel.persistent_keepalive()
    }

    pub fn preshared_key(&self) -> Option<&[u8; 32]> {
        self.preshared_key.as_ref()
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, SocketAddr};
    use crate::device::peer::AllowedIP;

    #[test]
    fn allow_ip_debug() {
        let ip_v4: AllowedIP = "10.0.0.0/32".parse().unwrap();
        assert_eq!(ip_v4.to_string(), String::from("10.0.0.0/32"));
        assert_eq!(ip_v4.addr.to_string(), String::from("10.0.0.0"));
    }

    #[test]
    fn ip_compare() {
        let ip = "10.0.0.1".parse::<IpAddr>();
        //println!("123 {:?}", ip);
        let ip1:IpAddr = "10.0.0.1".parse().unwrap();
        let ip2:IpAddr = "10.0.0.2".parse().unwrap();
        println!("should be false {}", ip1 == ip2);
        println!("should be true {}", ip1 < ip2);
    }
}
