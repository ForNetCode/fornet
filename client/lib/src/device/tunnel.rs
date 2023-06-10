use std::net::{SocketAddr};
use tokio::net::{TcpListener, ToSocketAddrs, UdpSocket};
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

pub fn create_tcp_server(port: Option<u16>, domain: Domain, mark:Option<u32>) ->anyhow::Result<TcpListener>{
    let socket = socket2::Socket::new(domain, Type::STREAM, Some(Protocol::TCP))?;
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
    let tcp_listener = TcpListener::from_std(socket.into())?;
    Ok(tcp_listener)
}

#[cfg(test)]
mod test {
    use socket2::Domain;
    use crate::device::tunnel::create_tcp_server;

    #[tokio::test]
    async fn test_tcp_bind() {
        let ip4_server = create_tcp_server(None, Domain::IPV4, None).unwrap();
        let ip6_server = create_tcp_server(Some(ip4_server.local_addr().unwrap().port()), Domain::IPV6, None).unwrap();

        println!("init ip4/ip6 in same port ok");
    }
}