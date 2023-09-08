
use ::tun as tun_crate;
use tun_crate::{AsyncDevice, Configuration};

use std::os::fd::RawFd;
pub type WritePart = WriteHalf<AsyncDevice>;
pub type ReadPart = ReadHalf<AsyncDevice>;
pub type TunSocket = (ReadPart, WritePart);

pub fn create_async_tun(file_descriptor:RawFd) ->anyhow::Result<()>{
    let mut config = Configuration::default();
    config.raw_fd = Some(file_descriptor);
    let device = tun_crate::create_as_async(&config).context(format!("create tun/tap fail"))?;
    //let pi = device.get_mut().has_packet_information();

    let (tun_read,tun_write) = tokio::io::split(device);
    Ok((tun_read, tun_write))
}