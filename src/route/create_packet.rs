use pnet::{
    self,
    packet::{
        icmp::{echo_request, IcmpTypes, MutableIcmpPacket},
        ip::IpNextHeaderProtocols,
        ipv4, util, Packet,
    },
};

use std::{io, net::Ipv4Addr};

const ICMP_SIZE: usize = 64;
const IPV4_SIZE: usize = 64;

//Add echo_reply

//Create packet IPV4
pub fn create_packet(destination: Ipv4Addr) -> Result<Vec<u8>, io::Error> {
    let mut ipv4_header = [0; IPV4_SIZE];

    let icmp_packet = create_packet_icmp(&mut ipv4_header)?;

    let mut raw_packet: Vec<u8> =
        vec![0; ipv4::MutableIpv4Packet::minimum_packet_size() + icmp_packet.len()];
    let mut ipv4_packet = ipv4::MutableIpv4Packet::new(&mut raw_packet).unwrap();

    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_total_length(ipv4_packet.packet().len() as u16);
    ipv4_packet.set_identification(257u16.to_be());
    ipv4_packet.set_flags(ipv4::Ipv4Flags::DontFragment);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);

    let checksum = ipv4_packet.packet();
    ipv4_packet.set_checksum(util::checksum(checksum, 1));
    ipv4_packet.set_source(Ipv4Addr::new(192, 168, 0, 1));
    ipv4_packet.set_destination(destination);

    Ok(ipv4_packet.packet().to_vec())
}

fn create_packet_icmp(header: &mut [u8; ICMP_SIZE]) -> Result<Vec<u8>, io::Error> {
    let mut raw_packet: Vec<u8> = vec![0; MutableIcmpPacket::minimum_packet_size() + header.len()];
    let mut icmp_packet = MutableIcmpPacket::new(&mut raw_packet).unwrap();
    icmp_packet.payload();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(echo_request::IcmpCodes::NoCode);
    icmp_packet.set_checksum(0);
    icmp_packet.set_checksum(util::checksum(icmp_packet.packet(), 1));

    Ok(icmp_packet.packet().to_vec())
}
