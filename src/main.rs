extern crate pnet;

use pnet::datalink::*;
use pnet::packet::arp::*;
use pnet::packet::ethernet::*;
use pnet::packet::{FromPacket, MutablePacket, Packet};
use std::net::*;

const BROADCAST: MacAddr = MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);

fn build_arp_packet(
    operation: ArpOperation,
    host_ip: Ipv4Addr,
    host_mac: MacAddr,
    target_ip: Ipv4Addr,
    target_mac: MacAddr,
) -> Vec<u8> {
    let mut a_buffer = [0u8; 28];
    let mut a_packet = MutableArpPacket::new(&mut a_buffer).unwrap();

    a_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    a_packet.set_protocol_type(EtherTypes::Ipv4);
    a_packet.set_operation(operation);
    a_packet.set_hw_addr_len(6);
    a_packet.set_proto_addr_len(4);
    a_packet.set_sender_hw_addr(host_mac);
    a_packet.set_sender_proto_addr(host_ip);
    a_packet.set_target_hw_addr(target_mac);
    a_packet.set_target_proto_addr(target_ip);

    a_packet.packet_mut().to_owned()
}

fn build_ethernet_packet(host_mac: MacAddr, target_mac: MacAddr, arp_packet: Vec<u8>) -> Vec<u8> {
    let mut e_buffer = [0u8; 42];
    let mut e_packet = MutableEthernetPacket::new(&mut e_buffer).unwrap();
    e_packet.set_destination(target_mac);
    e_packet.set_source(host_mac);
    e_packet.set_ethertype(EtherTypes::Arp);
    e_packet.set_payload(&arp_packet);
    e_packet.packet().to_owned()
}

fn get_mac_ip(iface_name: &str) -> (MacAddr, Ipv4Addr) {
    let interface = pnet::datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .expect("Cannot find interface");
    let iface_mac = interface.mac;
    let iface_ip = match interface
        .ips
        .iter()
        .find(|iface| iface.is_ipv4())
        .expect("Cannot find IPv4 address")
        .ip()
    {
        IpAddr::V4(ip) => ip,
        _ => panic!(),
    };
    (iface_mac.unwrap(), iface_ip)
}

fn main() {
    let iface_name = "en0";
    let (host_mac, host_ip) = get_mac_ip(iface_name);
    println!("Your MAC address: {}", host_mac);
    println!("Your IP: {}", host_ip);
    let arp_packet = build_arp_packet(
        ArpOperations::Request,
        host_ip,
        host_mac,
        Ipv4Addr::new(192, 168, 1, 254),
        BROADCAST,
    );
    println!("ARP PACKET: {:?}", arp_packet);
    let ethernet_packet = build_ethernet_packet(host_mac, BROADCAST, arp_packet);
    println!("ETHERNET PACKET: {:?}", ethernet_packet);
}