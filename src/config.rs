use pnet::util::MacAddr;
use std::net::Ipv4Addr;

pub struct Params {
    pub interface: String,
    pub host_ip: Ipv4Addr,
    pub target_ip: Ipv4Addr,

    pub gateway_ip: Ipv4Addr,
    pub gateway_mac: MacAddr,

    pub host_mac: MacAddr,
    pub target_mac: MacAddr,
    pub recover: bool,
}
