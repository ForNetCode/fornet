use std::process::Command;
use crate::device::peer::AllowedIP;

pub fn set_route(iface_name:&str, address: &AllowedIP) -> anyhow::Result<()> {

    let inet = if address.addr.is_ipv4() { "-4" } else { "-6" };
    Command::new("ip").args(&[inet, "address", "add", &address.to_string(), "dev", iface_name]).status()?;
    Ok(())
}
pub fn remove_route(iface_name:&str, address: &AllowedIP) -> anyhow::Result<()> {
    let inet = if address.addr.is_ipv4() { "-4" } else { "-6" };
    Command::new("ip").args(&[inet, "address", "remove", &address.to_string(), "dev", iface_name]).status()?;
    Ok(())
}
/*
pub fn set_route(iface_name:&str, allowed_ip: &AllowedIP) -> anyhow::Result<()> {
    //TODO: support allowed_ip is 0.0.0.0/0
    // ip -4 route add  10.0.0.1/24 dev ForT
    //let inet = if allowed_ip.addr.is_ipv4() { "-4" } else { "-6" };
    //Command::new("ip").args(&[inet, "route", "add", &allowed_ip.to_string(), "dev", iface_name]).status()?;
    Ok(())
}

pub fn remove_route(iface_name:&str, allowed_ip:&AllowedIP) -> anyhow::Result<()> {
    //let inet = if allowed_ip.addr.is_ipv4() { "-net" } else { "-6net" };
    //Command::new("route").args(&["del", inet,&allowed_ip.to_string(),"dev", iface_name]).status()?;
    Ok(())
}
*/