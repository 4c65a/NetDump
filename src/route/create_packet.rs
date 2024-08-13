use pnet::{
    self,
    packet::{
        icmp::{
            echo_request::{self, MutableEchoRequestPacket},
            IcmpTypes,
        },
        ip::IpNextHeaderProtocols,
        ipv4::{self, MutableIpv4Packet},
        util, Packet,
    },
};

use std::{io, net::Ipv4Addr};

const ICMP_SIZE: usize = 8;
const IPV4_SIZE: usize = 20;
const TOTAL_SIZE: usize = IPV4_SIZE + ICMP_SIZE;

pub fn handle_packet(destination: Ipv4Addr) -> Result<Vec<u8>, io::Error> {
    // Crear y configurar el paquete ICMP
    let mut icmp_packet: [u8; ICMP_SIZE] = [0; ICMP_SIZE];
    let mut icmp = MutableEchoRequestPacket::new(&mut icmp_packet).unwrap();
    create_packet_icmp(&mut icmp);

    // Calcular el checksum de Icmp
    let icmp_checksum = util::checksum(icmp.packet(), 1);
    icmp.set_checksum(icmp_checksum);

    // Crear y configurar el paquete IPv4
    //let mut ip_packet: [u8; IPV4_SIZE + ICMP_SIZE] = [0; IPV4_SIZE + ICMP_SIZE];
    let mut ip_packet: [u8; TOTAL_SIZE] = [0; TOTAL_SIZE];
    let mut ipv4 = MutableIpv4Packet::new(&mut ip_packet).unwrap();
    ipv4_create_packet(&mut ipv4, destination);

    // Establecer la carga útil del IPv4 con el paquete ICMP
    ipv4.set_payload(icmp.packet());

    // Calcular el checksum del IPv4
    let ipv4_checksum = util::checksum(ipv4.packet(), 1);
    ipv4.set_checksum(ipv4_checksum);

    Ok(ipv4.packet().to_vec())
}

pub fn ipv4_create_packet(ipv4_packet: &mut MutableIpv4Packet, destination: Ipv4Addr) {
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_total_length(TOTAL_SIZE as u16);
    ipv4_packet.set_identification(257u16.to_be());
    ipv4_packet.set_flags(ipv4::Ipv4Flags::DontFragment);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);

    //let checksum = ipv4_packet.packet();
    //ipv4_packet.set_payload(&icmp.packet());
    let checksum = util::checksum(ipv4_packet.packet(), 1);
    ipv4_packet.set_checksum(checksum);
    //ipv4_packet.set_checksum(util::checksum(checksum, 1));
    //ipv4_packet.set_source(Ipv4Addr::new(192, 168, 1, 200));
    ipv4_packet.set_destination(destination);
    //ipv4_packet.set_payload(ipv4_packet.packet());
    //Ok(ipv4_packet.packet().to_vec())
}

fn create_packet_icmp(icmp_packet: &mut MutableEchoRequestPacket) {
    //let mut raw_packet: Vec<u8> = vec![0; ICMP_SIZE];
    //let mut icmp_packet = MutableIcmpPacket::new(header).unwrap();
    //let mut icmp_packet = MutableIcmpPacket::new(&mut raw_packet).unwrap();
    //icmp_packet.payload();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(echo_request::IcmpCodes::NoCode);
    //icmp_packet.set_checksum(0);
    icmp_packet.set_identifier(1);
    icmp_packet.set_sequence_number(1);
    //icmp_packet.set_checksum(util::checksum(icmp_packet.packet(), 1));

    //Ok(icmp_packet.packet().to_vec())
}
