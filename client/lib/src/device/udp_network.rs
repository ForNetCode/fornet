use std::net::SocketAddr;
use tokio::net::UdpSocket;
use socket2::{Type, Protocol, Domain};

pub fn create_udp_socket(port: Option<u16>, domain: Domain, mark:Option<u32>) -> anyhow::Result<UdpSocket> {
    let socket = socket2::Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_nonblocking(true)?;

    #[cfg(target_os = "linux")]
    {
        socket.set_reuse_address(true)?; // On Linux SO_REUSEPORT won't prefer a connected IPv6 socket
        if let Some(mark) = mark {
            socket.set_mark(mark)?;
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    socket.set_reuse_port(true)?;

    let port = port.unwrap_or(0);

    let address: SocketAddr = match domain {
        Domain::IPV4 =>
            format!("0.0.0.0:{}", port),
        Domain::IPV6 =>
            format!("[::]:{}", port),
        _ => panic!("udp client don't support Domain::Unix")
    }.parse()?;
    socket.bind(&address.into())?;
    Ok(UdpSocket::from_std(socket.into())?)
}