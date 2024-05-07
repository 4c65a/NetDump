use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    icmp::IcmpPacket,
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

pub fn ipv4_handler(ether: &EthernetPacket) {
    if ether.get_ethertype() == EtherTypes::Ipv4 {
        let packet = Ipv4Packet::new(ether.payload()).unwrap();
        let tcp = TcpPacket::new(ether.payload()).unwrap();
        let udp = UdpPacket::new(ether.payload()).unwrap();
        let icmp = IcmpPacket::new(ether.payload()).unwrap();
        // Handler TCP
        if packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
            let source = tcp.get_source();
            let source_ipv4 = packet.get_source();
            let destination = tcp.get_destination();
            let destination_ipv4 = packet.get_destination();
            let flag = tcp.get_flags();
            let flag_ipv4 = packet.get_flags();
            //let payload = tcp.get_payload();
            let ttl = packet.get_ttl();
            let version = packet.get_version();
            println!("Protocol: TCP | Source:{} | Ipv4 {}| Destination:{} | Ipv4 {} | Flag:{} | Ipv4 {} | ttl: {} | Version: {} ",
                            source,
                            source_ipv4,
                            destination,
                            destination_ipv4,
                            flag,
                            flag_ipv4,
                             ttl,
                            version
                        );
        // Handler UDP
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
            let source = udp.get_source();
            let source_ipv4 = packet.get_source();
            let destination = udp.get_destination();
            let destination_ipv4 = packet.get_destination();
            let flag_ipv4 = packet.get_flags();
            //let payload = udp.get_payload();
            let length = udp.get_length();
            let ttl = packet.get_ttl();
            let version = packet.get_version();
            println!(
                "Protocol: UDP | Source: {} | Ipv4 {} | Destination: {} | Ipv4 {} | Flag: {} | Ipv4 {} | ttl: {} | Version: {} ",
                source, source_ipv4, destination, destination_ipv4, length, flag_ipv4, ttl, version
            );
        // Handler ICMP
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Icmp {
            let source_ipv4 = packet.get_source();
            let icmp_type = icmp.get_icmp_type();
            let destination_ipv4 = packet.get_destination();
            let flag_ipv4 = packet.get_flags();
            //let payload = icmp.get_payload();
            let ttl = packet.get_ttl();
            let version = packet.get_version();
            println!(
                "Protocol: ICMP | Source_Ipv4: {} | IcmpType: {:#?} | Destination Ipv4: {} | Flag: {} | ttl: {} | Version: {} ",
                source_ipv4, icmp_type, destination_ipv4, flag_ipv4, ttl, version
            );
        }
    }
}
pub fn ipv6_handler(ether: &EthernetPacket) {
    if ether.get_ethertype() == EtherTypes::Ipv6 {
        let packet = Ipv6Packet::new(ether.payload()).unwrap();
        let tcp = TcpPacket::new(ether.payload()).unwrap();
        let udp = UdpPacket::new(ether.payload()).unwrap();
        let icmp = IcmpPacket::new(ether.payload()).unwrap();

        if packet.get_next_header() == IpNextHeaderProtocols::Tcp {
            let source = tcp.get_source();
            let source_ipv6 = packet.get_source();
            let destination = tcp.get_destination();
            let destination_ipv6 = packet.get_destination();
            let flag = tcp.get_flags();
            // let payload = tcp.get_payload();
            let version = packet.get_version();
            println!(
                "Source: {} | Ipv6: {} | Destination: {} | Ipv6: {} | flag: {} | Version: {} ",
                source, source_ipv6, destination, destination_ipv6, flag, version
            );
        } else if packet.get_next_header() == IpNextHeaderProtocols::Udp {
            let source = udp.get_source();
            let source_ipv6 = packet.get_source();
            let destination = udp.get_destination();
            let destination_ipv6 = packet.get_destination();
            //let payload = udp.get_payload();
            let length = udp.get_length();
            let version = packet.get_version();
            println!(
                "Source: {} | Ipv6: {} | Destination: {} | Ipv6: {} | Length: {} | Version: {} ",
                source, source_ipv6, destination, destination_ipv6, length, version
            );
        } else if packet.get_next_header() == IpNextHeaderProtocols::Icmp {
            let source_ipv6 = packet.get_source();
            let icmp_type = icmp.get_icmp_type();
            let destination_ipv6 = packet.get_destination();
            //let payload = icmp.get_payload();
            let version = packet.get_version();
            println!(
                "Source_Ipv6:{} | IcmpType: {:#?} | Destination Ipv6: {} | Version: {} ",
                source_ipv6, icmp_type, destination_ipv6, version
            );
        }
    }
}
