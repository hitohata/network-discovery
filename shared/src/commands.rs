use std::net::Ipv4Addr;

pub enum DiscoveryCommand {
    DeviceInformation(Ipv4Addr),
}
