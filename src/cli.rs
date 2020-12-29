use clap::{App, Arg};
use pnet::packet::arp::{ArpOperation, ArpOperations};
use pnet::util::{MacAddr, ParseMacAddrErr};
use std::net::{AddrParseError, Ipv4Addr};

use crate::config;

pub fn command_line_start() -> config::Params {
    let matches = App::new("Arp_spoof")
        .author("Felipe Brasileiro")
        .about(
            "
                Tool for MITM attacks
            ",
        )
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .required(true)
                .takes_value(true)
                .help("Provide the interface to be used to send packets"),
        )
        .arg(
            Arg::with_name("gateway")
                .short("g")
                .long("gateway-ip")
                .required(true)
                .takes_value(true)
                .help("Set the gateway ip"),
        )
        .arg(
            Arg::with_name("source_ip")
                .short("s")
                .long("source-ip")
                .required(true)
                .takes_value(true)
                .help("Set the source ip"),
        )
        .arg(
            Arg::with_name("source_mac")
                .long("source-mac")
                .short("m")
                .required(true)
                .takes_value(true)
                .help("Set the source mac address"),
        )
        .arg(
            Arg::with_name("target_ip")
                .short("T")
                .long("target-ip")
                .required(true)
                .takes_value(true)
                .help("Set the target ip"),
        )
        .arg(
            Arg::with_name("target_mac")
                .short("M")
                .long("target-mac")
                .required(true)
                .takes_value(true)
                .help("Set the target mac address"),
        )
        .get_matches();

    let interface: String = matches.value_of("interface").unwrap().trim().to_string();

    let gateway: Result<Ipv4Addr, AddrParseError> =
        matches.value_of("gateway").unwrap().trim().parse();
    let host_ip: Result<Ipv4Addr, AddrParseError> =
        matches.value_of("source_ip").unwrap().trim().parse();
    let host_mac: Result<MacAddr, ParseMacAddrErr> =
        matches.value_of("source_mac").unwrap().trim().parse();
    let target_ip: Result<Ipv4Addr, AddrParseError> =
        matches.value_of("target_ip").unwrap().trim().parse();
    let target_mac: Result<MacAddr, ParseMacAddrErr> =
        matches.value_of("target_mac").unwrap().trim().parse();

    let params: config::Params = config::Params {
        interface: interface,
        gateway_ip: gateway.unwrap(),
        host_ip: host_ip.unwrap(),
        host_mac: host_mac.unwrap(),
        target_ip: target_ip.unwrap(),
        target_mac: target_mac.unwrap(),
    };

    params
}
