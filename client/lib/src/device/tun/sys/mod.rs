use crate::device::peer::AllowedIP;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;


pub fn set_alias(iface_name:&str, address: &AllowedIP) -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    macos::set_alias(iface_name, address)?;

    #[cfg(target_os = "linux")]
    linux::add_addr(iface_name, address)?;

    Ok(())
}

pub fn set_route(iface_name:&str, allowed_ip: &AllowedIP) -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    macos::set_route(iface_name, allowed_ip)?;

    #[cfg(target_os = "linux")]
    linux::set_route(iface_name, allowed_ip)?;
    Ok(())
}

pub fn remove_route(iface_name:&str, allowed_ip:&AllowedIP) -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    macos::remove_route(iface_name, allowed_ip)?;

    #[cfg(target_os ="linux")]
    linux::remove_route(iface_name, allowed_ip)?;

    Ok(())
}