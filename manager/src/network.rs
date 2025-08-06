use get_if_addrs::{get_if_addrs, IfAddr};
use std::net::IpAddr;

pub fn search_networks() -> Option<IpAddr> {
    let if_addrs = get_if_addrs().ok().or_else(|| {
        eprintln!("Failed to retrieve network interfaces.");
        None
    })?;

    for if_addr in if_addrs {
        if if_addr.is_loopback() {
            continue;
        }

        if let IfAddr::V4(ipv4) = if_addr.addr {
            let ip = ipv4.ip;
            if ip.is_loopback() || ip.is_unspecified() {
                continue;
            }
            if ip.is_private() {
                return Some(IpAddr::V4(ip));
            }
        }
    }

    None
}
