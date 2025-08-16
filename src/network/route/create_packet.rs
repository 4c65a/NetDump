use pnet::{
    packet::{
        Packet,
        arp::{ArpHardwareTypes, ArpOperations, MutableArpPacket},
        ethernet::{EtherTypes, MutableEthernetPacket},
        icmp::{
            IcmpTypes,
            echo_request::{self, MutableEchoRequestPacket},
        },
        icmpv6::Icmpv6Types,
        ip::IpNextHeaderProtocols,
        ipv4::{self, MutableIpv4Packet},
        ipv6::MutableIpv6Packet,
        util,
    },
    util::MacAddr,
};

use std::{
    io,
    net::{Ipv4Addr, Ipv6Addr},
};

/*

    IPv4(icmp4):
    +------------------+-----------------+-----------------+
    | 20 bytes (IPv4)  | 8 bytes (ICMP)  | 32 bytes (Payload)|
    +------------------+-----------------+-----------------+
    |             Total: 60 bytes                        |
    +---------------------------------------------------+

    IPv6(icmp6):
    +------------------+-----------------+-----------------+
    | 40 bytes (IPv6)  | 8 bytes (ICMPv6)| 32 bytes (Payload)|
    +------------------+-----------------+-----------------+
    |             Total: 80 bytes                        |
    +---------------------------------------------------+


*/

const IPV6_SIZE: usize = 0x28;
const TOTAL_LENGTH_SIZE_IPV6: usize = IPV6_SIZE + ICMP_HEADER_LEN + ICMP_HEADER_LEN;

const IPV4_HEADER_LEN: usize = MutableIpv4Packet::minimum_packet_size(); // 20 bytes
const ICMP_HEADER_LEN: usize = MutableEchoRequestPacket::minimum_packet_size(); // 8 bytes
const ICMP_PAYLOAD_LEN: usize = 0x38; // 56 bytes

pub fn handle_packet(destination: Ipv4Addr, ttl: u8) -> Result<Vec<u8>, io::Error> {
    let total_len = IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN; // 64 bytes

    let mut packet_buffer = vec![0u8; total_len];

    // Dividir el buffer: cabecera IPv4 y resto (ICMP)
    let (ipv4_buf, icmp_buf) = packet_buffer.split_at_mut(IPV4_HEADER_LEN);

    // Construcción del paquete ICMP
    let mut icmp_packet = MutableEchoRequestPacket::new(icmp_buf).unwrap();
    create_packet_icmp(&mut icmp_packet);

    let icmp_checksum = util::checksum(icmp_packet.packet(), 1);
    icmp_packet.set_checksum(icmp_checksum);

    // Construcción del paquete IPv4
    let mut ipv4_packet = MutableIpv4Packet::new(ipv4_buf).unwrap();
    ipv4_create_packet(&mut ipv4_packet, destination, ttl);

    ipv4_packet.set_total_length(total_len as u16);

    let ipv4_checksum = util::checksum(ipv4_packet.packet(), 1);
    ipv4_packet.set_checksum(ipv4_checksum);

    Ok(packet_buffer)
}
/*
    +-----------------------------------------+
    /            Headers IPV4                 /
    +-----------------------------------------+
    / Version             /                   /
    +-----------------------------------------+
    / Header Lengtg       /                   /
    +-----------------------------------------+
    / Identification      /                   /
    +-----------------------------------------+
    / Flags               /                   /
    +-----------------------------------------+
    / Fragment offset     /                   /
    +-----------------------------------------+
    / Time to Live ttl    /                   /
    +-----------------------------------------+
    / Next Level Protocol /                   /
    +-----------------------------------------+
    / Checksum            /                   /
    +-----------------------------------------+
    / Destination         /                   /
    +-----------------------------------------+
*/

pub fn ipv4_create_packet(ipv4_packet: &mut MutableIpv4Packet, destination: Ipv4Addr, ttl: u8) {
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length((IPV4_HEADER_LEN / 4) as u8);
    ipv4_packet.set_identification(257u16.to_be());
    ipv4_packet.set_flags(ipv4::Ipv4Flags::DontFragment);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(ttl);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);

    let checksum = util::checksum(ipv4_packet.packet(), 1);
    ipv4_packet.set_checksum(checksum);
    ipv4_packet.set_destination(destination);
}

/*
    +-----------------------------------------+
    /    Headers Echo Request ICMP4/ICMP6     /
    +-----------------------------------------+
    / Icmp Type           /                   /
    +-----------------------------------------+
    / Icmp Code           /                   /
    +-----------------------------------------+
    / Identifier          /                   /
    +-----------------------------------------+
    / Sequence Number     /                   /
    +-----------------------------------------+
*/

fn create_packet_icmp(
    echo_packet: &mut pnet::packet::icmp::echo_request::MutableEchoRequestPacket,
) {
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_packet.set_icmp_code(echo_request::IcmpCodes::NoCode);

    echo_packet.set_identifier(1);
    echo_packet.set_sequence_number(1);

    let checksum = util::checksum(echo_packet.packet(), 0);
    echo_packet.set_checksum(checksum)
}

pub fn handle_packet_ipv6(destination: Ipv6Addr) -> Result<Vec<u8>, io::Error> {
    let mut icmp_packet: [u8; ICMP_HEADER_LEN] = [0; ICMP_HEADER_LEN];
    let mut icmp6 =
        pnet::packet::icmpv6::echo_request::MutableEchoRequestPacket::new(&mut icmp_packet)
            .unwrap();
    create_packet_icmp6(&mut icmp6);

    let mut ip6_packet: [u8; TOTAL_LENGTH_SIZE_IPV6] = [0; TOTAL_LENGTH_SIZE_IPV6];
    let mut ipv6 = MutableIpv6Packet::new(&mut ip6_packet).unwrap();

    ipv6_create_packet(&mut ipv6, destination);

    ipv6.set_destination(destination);
    ipv6.set_payload_length(icmp6.packet().len() as u16);

    ipv6.set_payload(icmp6.packet());

    Ok(ipv6.packet().to_vec())
}

/*
    +-----------------------------------------+
    /            Headers IPV6                 /
    +-----------------------------------------+
    / Version             /                   /
    +-----------------------------------------+
    / Next Header         /                   /
    +-----------------------------------------+
    / Destinantion        /                   /
    +-----------------------------------------+
*/

pub fn ipv6_create_packet(ipv6_packet: &mut MutableIpv6Packet, destination: Ipv6Addr) {
    ipv6_packet.set_version(6);
    ipv6_packet.set_traffic_class(0);
    ipv6_packet.set_flow_label(0);

    ipv6_packet.set_destination(destination);

    ipv6_packet.set_next_header(IpNextHeaderProtocols::Icmpv6);
    ipv6_packet.set_hop_limit(64);
}

fn create_packet_icmp6(
    echo_packet: &mut pnet::packet::icmpv6::echo_request::MutableEchoRequestPacket,
) {
    echo_packet.set_icmpv6_type(Icmpv6Types::EchoRequest);
    echo_packet.set_icmpv6_code(pnet::packet::icmpv6::echo_request::Icmpv6Codes::NoCode);
    echo_packet.set_identifier(1);
    echo_packet.set_sequence_number(1);

    let checksum = util::checksum(echo_packet.packet(), 0);
    echo_packet.set_checksum(checksum);
}

pub fn handle_packet_trace(
    destination: Ipv4Addr,
    ttl: u8,
    identifier: u16,
    sequence_number: u16,
) -> Result<Vec<u8>, io::Error> {
    let mut icmp_packet: [u8; ICMP_HEADER_LEN] = [0; ICMP_HEADER_LEN];
    let total_len = IPV4_HEADER_LEN + ICMP_HEADER_LEN + ICMP_PAYLOAD_LEN;
    let mut packet_buffer = vec![0u8; total_len];

    let mut icmp =
        pnet::packet::icmp::echo_request::MutableEchoRequestPacket::new(&mut icmp_packet).unwrap();
    create_packet_icmp_trace(&mut icmp, identifier, sequence_number);

    let icmp_checksum = util::checksum(icmp.packet(), 1);
    icmp.set_checksum(icmp_checksum);

    //let mut ip_packet: [u8; TOTAL_LENGTH_SIZE] = [0; TOTAL_LENGTH_SIZE];
    let mut ipv4 = MutableIpv4Packet::new(&mut packet_buffer).unwrap();
    ipv4_create_packet(&mut ipv4, destination, ttl);

    ipv4.set_total_length((total_len + icmp.packet().len()) as u16);

    ipv4.set_payload(icmp.packet());

    let ipv4_checksum = util::checksum(ipv4.packet(), 1);
    ipv4.set_checksum(ipv4_checksum);
    Ok(ipv4.packet().to_vec())
}
#[allow(dead_code)]
fn create_packet_icmp_trace(
    echo_packet: &mut pnet::packet::icmp::echo_request::MutableEchoRequestPacket,
    identifier: u16,
    sequence_number: u16,
) {
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_packet.set_icmp_code(echo_request::IcmpCodes::NoCode);
    echo_packet.set_identifier(identifier);
    echo_packet.set_sequence_number(sequence_number);

    let checksum = util::checksum(echo_packet.packet(), 0);
    echo_packet.set_checksum(checksum);
}

/*
    +--------------------------------------------------------+
    /                 ARP Packet Header                      /
    +-------------------------+------------------------------+
    / Hardware Type (HTYPE)   / 2 bytes (0x0001 for Ethernet)/
    +-------------------------+------------------------------+
    / Protocol Type (PTYPE)   / 2 bytes (0x0800 for IPv4)    /
    +-------------------------+------------------------------+
    / Hardware Size (HLEN)    / 1 byte (6 for MAC)           /
    +-------------------------+------------------------------+
    / Protocol Size (PLEN)    / 1 byte (4 for IPv4)          /
    +-------------------------+------------------------------+
    / Operation (Opcode)      / 2 bytes                      /
    +-------------------------+------------------------------+
    / Sender MAC Address      / 6 bytes                      /
    +-------------------------+------------------------------+
    / Sender IP Address       / 4 bytes                      /
    +-------------------------+------------------------------+
    / Target MAC Address      / 6 bytes                      /
    +-------------------------+------------------------------+
    / Target IP Address       / 4 bytes                      /
    +--------------------------------------------------------+
*/

pub fn handle_packet_arp(
    source_ip: Ipv4Addr,
    source_mac: MacAddr,
    target_ip: Ipv4Addr,
) -> Result<Vec<u8>, std::io::Error> {
    let ethernet_header_size = MutableEthernetPacket::minimum_packet_size();
    let arp_packet_size = MutableArpPacket::minimum_packet_size();
    let total_packet_size = ethernet_header_size + arp_packet_size;

    let mut buffer = vec![0u8; total_packet_size];

    let mut ether_packet = MutableEthernetPacket::new(&mut buffer[..ethernet_header_size])
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error al crear el paquete Ethernet",
            )
        })?;

    ether_packet.set_destination(MacAddr::broadcast());
    ether_packet.set_source(source_mac);
    ether_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_packet =
        MutableArpPacket::new(&mut buffer[ethernet_header_size..]).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error al crear el paquete ARP en el payload",
            )
        })?;

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);

    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);

    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    Ok(buffer)
}
