use get_if_addrs::{IfAddr, get_if_addrs};
use std::env;
use std::net::{IpAddr, Ipv4Addr};

fn search_networks() -> Option<IpAddr> {
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

pub fn get_ip() -> Ipv4Addr {
    let args: Vec<String> = env::args().collect();
    let ip: Ipv4Addr = if args.len() < 2 {
        match search_networks() {
            Some(ip) => {
                if let IpAddr::V4(ipv4) = ip {
                    ipv4
                } else {
                    panic!("Found an IP address, but it is not IPv4.");
                }
            }
            None => {
                panic!("No valid IP address found.");
            }
        }
    } else {
        let input_ip = args[1].to_string();
        input_ip.parse::<Ipv4Addr>().unwrap()
    };

    ip
}
