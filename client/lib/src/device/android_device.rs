use crate::device::DeviceData;
use crate::protobuf::config::{NodeType, Protocol};
use std::os::fd::RawFd;
use crate::device::tun::create_async_tun;

pub struct Device {
    pub device_data: DeviceData
}

impl Device {
    pub fn new(
        key_pair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
        port: Option<u16>,
        mtu: u32,
        protocol: Protocol,
        node_type: NodeType,
        raw_fd: RawFd,
    ) -> anyhow::Result<Self>{
        let (tun_read, tun_write) = create_async_tun(raw_fd)?;


        todo!()
    }
}