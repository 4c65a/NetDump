use pnet::{
    self,
    packet::{
        icmp::{echo_request, IcmpTypes},
        icmpv6::Icmpv6Types,
        ip::IpNextHeaderProtocols,
        ipv4::{self, MutableIpv4Packet},
        ipv6::MutableIpv6Packet,
        util, Packet,
    },
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

const PAYLOAD_ICMP: usize = 56;
const ICMP_SIZE: usize = 8;
const IPV4_SIZE: usize = 20;
const TOTAL_LENGTH_SIZE: usize = IPV4_SIZE + ICMP_SIZE + PAYLOAD_ICMP;

const IPV6_SIZE: usize = 40;
const TOTAL_LENGTH_SIZE_IPV6: usize = IPV6_SIZE + ICMP_SIZE + PAYLOAD_ICMP;

#[allow(dead_code)]
pub fn handle_packet(destination: Ipv4Addr) -> Result<Vec<u8>, io::Error> {
    // Crear y configurar el paquete ICMP
    let mut icmp_packet: [u8; ICMP_SIZE] = [0; ICMP_SIZE];
    let mut icmp =
        pnet::packet::icmp::echo_request::MutableEchoRequestPacket::new(&mut icmp_packet).unwrap();
    create_packet_icmp(&mut icmp);

    // Calcular el checksum de Icmp
    let icmp_checksum = util::checksum(icmp.packet(), 1);
    icmp.set_checksum(icmp_checksum);

    // Crear y configurar el paquete IPv4
    let mut ip_packet: [u8; TOTAL_LENGTH_SIZE] = [0; TOTAL_LENGTH_SIZE];
    let mut ipv4 = MutableIpv4Packet::new(&mut ip_packet).unwrap();
    ipv4_create_packet(&mut ipv4, destination);

    //Establecer el tamaño total
    ipv4.set_total_length((TOTAL_LENGTH_SIZE + icmp.packet().len()) as u16);

    // Establecer la carga útil del IPv4 con el paquete ICMP
    ipv4.set_payload(icmp.packet());

    // Calcular el checksum del IPv4
    let ipv4_checksum = util::checksum(ipv4.packet(), 1);
    ipv4.set_checksum(ipv4_checksum);
    Ok(ipv4.packet().to_vec())
}

#[allow(dead_code)]
pub fn handle_packet_ipv6(destinantion: Ipv6Addr) -> Result<Vec<u8>, io::Error> {
    // Create packet
    let mut icmp_packet: [u8; ICMP_SIZE] = [0; ICMP_SIZE];
    let mut icmp6 =
        pnet::packet::icmpv6::echo_request::MutableEchoRequestPacket::new(&mut icmp_packet)
            .unwrap();
    create_packet_icmp6(&mut icmp6);

    // Create and configuration of the ipv6 packet
    let mut ip6_packet: [u8; TOTAL_LENGTH_SIZE_IPV6] = [0; TOTAL_LENGTH_SIZE_IPV6];
    let mut ipv6 = MutableIpv6Packet::new(&mut ip6_packet).unwrap();
    ipv6_create_packet(&mut ipv6, destinantion);

    ipv6.set_payload(icmp6.packet());

    Ok(ipv6.packet().to_vec())
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

#[allow(dead_code)]
pub fn ipv4_create_packet(ipv4_packet: &mut MutableIpv4Packet, destination: Ipv4Addr) {
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length((IPV4_SIZE / 4) as u8);
    ipv4_packet.set_identification(257u16.to_be());
    ipv4_packet.set_flags(ipv4::Ipv4Flags::DontFragment);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);

    let checksum = util::checksum(ipv4_packet.packet(), 1);
    ipv4_packet.set_checksum(checksum);
    ipv4_packet.set_destination(destination);
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

#[allow(dead_code)]
pub fn ipv6_create_packet(ipv6_packet: &mut MutableIpv6Packet, destinantion: Ipv6Addr) {
    ipv6_packet.set_version(6);
    ipv6_packet.set_next_header(IpNextHeaderProtocols::Icmpv6);
    ipv6_packet.set_destination(destinantion);
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

#[allow(dead_code)]
fn create_packet_icmp(
    echo_packet: &mut pnet::packet::icmp::echo_request::MutableEchoRequestPacket,
) {
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_packet.set_icmp_code(echo_request::IcmpCodes::NoCode);
    echo_packet.set_identifier(1);
    echo_packet.set_sequence_number(1);
}

#[allow(dead_code)]
fn create_packet_icmp6(
    echo_packet: &mut pnet::packet::icmpv6::echo_request::MutableEchoRequestPacket,
) {
    echo_packet.set_icmpv6_type(Icmpv6Types::EchoRequest);
    echo_packet.set_icmpv6_code(pnet::packet::icmpv6::echo_request::Icmpv6Codes::NoCode);
    echo_packet.set_identifier(1);
    echo_packet.set_sequence_number(1);
}
