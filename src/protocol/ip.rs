use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket}, icmp::IcmpPacket, icmpv6::Icmpv6Packet, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet
};



pub fn ipv4_handler(ether: &EthernetPacket) {
    if ether.get_ethertype() == EtherTypes::Ipv4 {

        let packet = Ipv4Packet::new(ether.payload()).unwrap();
    
        let tcp = TcpPacket::new(ether.payload()).unwrap();
        let _udp = UdpPacket::new(ether.payload()).unwrap();
        let _icmp = IcmpPacket::new(ether.payload()).unwrap();

  
        // Handler TCP
        if packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {

     
             let source_port = tcp.get_source().to_string();
             let destination_port = tcp.get_destination();
    
             let source_ipv4 = packet.get_source();
             let destination_ipv4 = packet.get_destination();

             let ttl = packet.get_ttl();
             let version = packet.get_version();

             println!(
                 "Protocol: TCP | Source: {:>15} | Destination: {:>15} | TTL: {:>3} | Version: {:>2} | Src Port: {} | Dst Port: {}",
                 source_ipv4,
                 destination_ipv4,
                 ttl,
                 version,
                 source_port,
                 destination_port
             );

        // Handler UDP
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
            //let source = udp.get_source();
            let source_ipv4 = packet.get_source();
            //let destination = udp.get_destination();
            let destination_ipv4 = packet.get_destination();
            //let flag_ipv4 = packet.get_flags();
            //let payload = udp.get_payload();
            //let length = udp.get_length();
            let ttl = packet.get_ttl();
            let version = packet.get_version();

            println!(
                "Protocol: UDP | Source: {:>15} | Destination: {:>15} | ttl: {:>3} | Version: {:>2}",
                source_ipv4,
                destination_ipv4,
                ttl,
                version
            );

        // Handler ICMP
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Icmp {
            let source_ipv4 = packet.get_source();
            //let icmp_type = icmp.get_icmp_type();
            let destination_ipv4 = packet.get_destination();
            //let flag_ipv4 = packet.get_flags();
            //let payload = icmp.get_payload();
            let ttl = packet.get_ttl();
            let version = packet.get_version();

  
            println!(
                "Protocol: ICMP | Source: {:>15} | Destination: {:>15} | ttl: {:>3} | Version: {:>2}",
                source_ipv4,
                destination_ipv4,
                ttl,
                version
            );
        }
    }
}


pub fn ipv6_handler(ether: &EthernetPacket) {

    if ether.get_ethertype() == EtherTypes::Ipv6 {

        let packet = Ipv6Packet::new(ether.payload()).unwrap();
        let tcp = TcpPacket::new(packet.payload()).unwrap();
        let udp = UdpPacket::new(packet.payload()).unwrap();
        let icmp = Icmpv6Packet::new(packet.payload()).unwrap();


        if packet.get_next_header() == IpNextHeaderProtocols::Tcp {

            let source = tcp.get_source();
            let source_ipv6 = packet.get_source();
            let destination = tcp.get_destination();
            let destination_ipv6 = packet.get_destination();
            let flag = tcp.get_flags();
            // let payload = tcp.get_payload();
            let version = packet.get_version();

            println!(
                "Source: {:>15} | Ipv6: {:>15} | Destination: {:>15} | Ipv6: {:>15} | flag: {:>3} | Version: {:>2} ",
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
                "Source: {:>15} | Ipv6: {:>15} | Destination: {:>15} | Ipv6: {:>15} | Length: {:>3} | Version: {:>2} ",
                source, source_ipv6, destination, destination_ipv6, length, version
            );

        } else if packet.get_next_header() == IpNextHeaderProtocols::Icmpv6 {

            let source_ipv6 = packet.get_source();
            let icmp_type = icmp.get_icmpv6_type();
            let destination_ipv6 = packet.get_destination();
            //let payload = icmp.get_payload();
            let version = packet.get_version();

            println!(
                "Source_Ipv6:{:>15} | IcmpType: {:>15?} | Destination Ipv6: {:>15} | Version: {:>2} ",
                source_ipv6, icmp_type, destination_ipv6, version
            );

        }
    }
}
