use windows::core::GUID;
use crate::device::peer::AllowedIP;
use cidr_utils::cidr::{Ipv4Cidr, Ipv6Cidr};

pub type WritePart = fortun_cli::WriteFile;
pub type ReadPart = fortun_cli::ReadFile;


pub fn create_async_tun(guid:&str, mtu:u32, address:&[AllowedIP]) -> anyhow::Result<(ReadPart, WritePart, String)> {

    let (read_part, write_part, device) = fortun_cli::create_async_tun(&GUID::from(guid), "ForTun", "TODO")?;

    for addr in address {
        let mask = if addr.addr.is_ipv4() {
            Ipv4Cidr::from_str(&addr.to_string()).unwrap().get_mask_as_ipv4_addr().to_string()
        } else {
            Ipv6Cidr::from_str(&addr.to_string()).unwrap().get_mask_as_ipv6_addr().to_string()
        };
        // MTU may be set twice
        fortun_cli::net_config(device.instance_id.clone(), &addr.addr.to_string(), &mask, mtu)?;
    }
    Ok((read_part, write_part, guid.to_string()))

}

#[cfg(test)]
mod test {
    use crate::device::peer::AllowedIP;

    use std::str::FromStr;
    use cidr_utils::cidr::Ipv6Cidr;

    #[test]
    fn test_cidr() {
        let ip = AllowedIP::from_str("10.0.0.1/8").unwrap();

        let cidr = cidr_utils::cidr::Ipv4Cidr::from_str("10.0.0.1/8").unwrap();
        //Ipv6Cidr::from_str("").unwrap().get_mask_as_ipv6_addr().to_string()
        println!("{}", cidr.get_mask_as_ipv4_addr().to_string())

    }
}