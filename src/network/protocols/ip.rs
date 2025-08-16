use std::net::{Ipv4Addr, Ipv6Addr};

use pnet::packet::{
    Packet,
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
};
use termion::color;

fn print_info(protocols: &str, source: &Ipv4Addr, destination: &Ipv4Addr, ttl: &u8, version: &u8) {
    println!(
        "{}Protocol: {:<6} | Source: {:>15} | Destination: {:>15} | TTL: {:>3} | Version: {:>2}{}",
        color::Fg(color::Magenta),
        protocols,
        source,
        destination,
        ttl,
        version,
        color::Fg(color::Reset),
    );
}

fn print_info_ipv6(
    protocols: &str,
    source: &Ipv6Addr,
    destination: &Ipv6Addr,
    ttl: &u8,
    version: &u8,
) {
    println!(
        "{}Protocol: {:<6} | Source: {:>15} | Destination: {:>15} | TTL: {:>3} | Version: {:>2}{}",
        color::Fg(color::LightMagenta),
        protocols,
        source,
        destination,
        ttl,
        version,
        color::Fg(color::Reset),
    );
}

pub fn ip_handler(ether: &EthernetPacket) {
    match ether.get_ethertype() {
        EtherTypes::Ipv4 => {
            let packet = Ipv4Packet::new(ether.payload()).unwrap();

            let source = packet.get_source();
            let destination = packet.get_destination();
            let ttl = packet.get_ttl();
            let version = packet.get_version();
            let protocol = packet.get_next_level_protocol();

            match protocol {
                IpNextHeaderProtocols::Tcp => {
                    print_info("TCP", &source, &destination, &ttl, &version);
                }
                IpNextHeaderProtocols::Udp => {
                    print_info("UDP", &source, &destination, &ttl, &version);
                }
                IpNextHeaderProtocols::Icmp => {
                    print_info("ICMP", &source, &destination, &ttl, &version);
                }
                other => {
                    println!("Unhandled protocol: {:?}", other);
                }
            }
        }

        /*
        IPv6 packet capture may not be supported on all interfaces.
        Ensure that your interface supports IPv6 traffic.
        */
        EtherTypes::Ipv6 => {
            let packet = Ipv6Packet::new(ether.payload()).unwrap();

            let source = packet.get_source();
            let destination = packet.get_destination();
            let ttl = packet.get_hop_limit();
            let version = packet.get_version();
            let protocol = packet.get_next_header();

            match protocol {
                IpNextHeaderProtocols::Tcp => {
                    print_info_ipv6("TCP", &source, &destination, &ttl, &version);
                }
                IpNextHeaderProtocols::Udp => {
                    print_info_ipv6("UDP", &source, &destination, &ttl, &version);
                }
                IpNextHeaderProtocols::Icmpv6 => {
                    print_info_ipv6("ICMPv6", &source, &destination, &ttl, &version);
                }
                other => {
                    println!("Unhandled protocol: {:?}", other);
                }
            }
        }
        _ => {
            println!("Unhandled EtherType");
        }
    }
}
