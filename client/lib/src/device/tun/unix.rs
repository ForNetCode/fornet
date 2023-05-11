

use ::tun as tun_crate;
use anyhow::Context;
use tokio::io::{ReadHalf, WriteHalf};
use tun::Device;
use tun_crate::{AsyncDevice, Configuration};
use crate::device::peer::AllowedIP;
use crate::device::tun::sys;

pub type WritePart = WriteHalf<AsyncDevice>;
pub type ReadPart = ReadHalf<AsyncDevice>;
pub type TunSocket = (ReadPart, WritePart,bool, String);

pub fn create_async_tun(name: &str, mtu: u32, address:&[AllowedIP],
) -> anyhow::Result<TunSocket> {
    let mut config = Configuration::default();

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        if name.starts_with("utun") {
            config.name(name);
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        config.name(name);// target macos ios must be utun[0-9]+
    }
    //config.netmask();

    config.mtu(
        mtu as i32,
    ).up();
    #[cfg(target_os = "linux")]
    config.platform(|config| {
        // IFF_NO_PI preventing excessive buffer reallocating
        config.packet_information(false);
    });
    let mut device = tun::create_as_async(&config).context(format!("create tun/tap fail"))?;
    let pi = device.get_mut().has_packet_information();
    let name = device.get_ref().name().to_string();

    for add in address {
        sys::set_alias(&name, add)?;
        sys::set_route(&name, add)?;
        tracing::info!("set alias and route:{}", &add.to_string());
    }

    let (tun_read,tun_write) = tokio::io::split(device);
    Ok((tun_read, tun_write, pi, name))
}
