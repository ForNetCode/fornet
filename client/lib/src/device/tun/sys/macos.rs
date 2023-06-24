use std::process::Command;
use crate::device::peer::AllowedIP;
use cmd_lib::run_cmd;


pub fn set_alias(iface_name: &str, address: &AllowedIP) -> anyhow::Result<()> {
    if address.addr.is_ipv4() {
        Command::new("ifconfig").args(&[iface_name, "inet", &address.to_string(), &address.addr.to_string(), "alias"]).status()?;
    } else {
        Command::new("ifconfig").args(&[iface_name, "inet6", &address.addr.to_string()]).status()?;
    }
    Ok(())
}

pub fn set_route(iface_name: &str, allowed_ip: &AllowedIP) -> anyhow::Result<()> {
    //TODO: support allowed_ip is 0.0.0.0/0
    let inet = if allowed_ip.addr.is_ipv4() { "-inet" } else { "-inet6" };
    Command::new("route").args(&["-q", "-n", "add", inet, &allowed_ip.to_string(), "-interface", iface_name]).status()?;
    Ok(())
}

pub fn remove_route(iface_name:&str, allowed_ip:&AllowedIP) -> anyhow::Result<()> {
    let inet = if allowed_ip.addr.is_ipv4() { "-inet" } else { "-inet6" };
    //Command::new("route").args(&["-q", "-n", "delete", inet, &allowed_ip.to_string(), "-interface", iface_name]).status()?;
    Ok(())
}

pub fn destroy_iface(iface_name:&str) -> anyhow::Result<()> {
    let r = run_cmd!(ifconfig ${iface_name} delete)?;
    Ok(())
}
