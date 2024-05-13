// https://docs.rs/pnet/latest/pnet/
/*
    Example input
    netdump 192.126.215
    netdump hola.com > 192.126.215
    netdump hola.com -count 10 (packet)
*/

/*
    Example Output
    ping -c 1 google.com
    PING google.com (172.217.172.110) 56(84) bytes of data.
    64 bytes from eze06s02-in-f14.1e100.net (172.217.172.110): icmp_seq=1 ttl=58 time=51.3 ms

    --- google.com ping statistics ---
    1 packets transmitted, 1 received, 0% packet loss, time 0ms
    rtt min/avg/max/mdev = 51.296/51.296/51.296/0.000 ms
*/

/*
    Input IP
    Create packet ICMP >> Ipv4 >> icmp6 >> ipv6
    Socket connection
    Send packet ICMP
    Receive packet ICMP
    Output
*/

use pnet::packet::{
    icmp::{
        echo_request::{IcmpCodes, MutableEchoRequestPacket},
        IcmpTypes,
    },
    ip::IpNextHeaderProtocols,
    ipv4::MutableIpv4Packet,
    util, Packet,
};
use rand::random;

fn create_packet_ipv4<'a>(header: &'a mut [u8]) -> Result<MutableIpv4Packet<'a>, std::io::Error> {
    let mut ipv4_packet = MutableIpv4Packet::new(header).unwrap();
    ipv4_packet.set_version();
    ipv4_packet.set_header_length();
    ipv4_packet.set_total_length();
    ipv4_packet.set_identification();
    ipv4_packet.set_flags();
    ipv4_packet.set_fragment_offset();
    ipv4_packet.set_ttl();
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_checksum();
    ipv4_packet.set_source();
    ipv4_packet.set_destination();

    Ok(ipv4_packet)
}

fn create_packet_icmp<'a>(
    header: &'a mut [u8],
) -> Result<MutableEchoRequestPacket<'a>, std::io::Error> {
    // Código para crear y devolver el paquete MutableEchoRequestPacket

    let mut icmp_packet = MutableEchoRequestPacket::new(header).unwrap();
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_icmp_code(IcmpCodes::NoCode);
    icmp_packet.set_checksum(util::checksum(icmp_packet.packet(), 1));
    icmp_packet.set_identifier(random::<u16>());
    icmp_packet.set_sequence_number(1);

    Ok(icmp_packet)
}
