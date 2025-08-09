use std::net::Ipv4Addr;

pub enum DiscoveryCommand {
    DeviceUsage,
    DeviceInformation(Ipv4Addr),
}
