use super::packet::{HeaderDataIcmp, HeaderDataIpv4, HeaderDataIpv6, HeaderDataTcp, HeaderDataUdp};
use log::{self, info};
use pnet::packet::{
    ethernet::{EtherTypes, EthernetPacket},
    ip::IpNextHeaderProtocols,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    Packet,
};

pub fn ipv4_handler(
    ether: &EthernetPacket,
    headertcp: &'static dyn HeaderDataTcp,
    headeripv4: &'static dyn HeaderDataIpv4,
    headerudp: &'static dyn HeaderDataUdp,
    headericmp: &'static dyn HeaderDataIcmp,
) {
    if ether.get_ethertype() == EtherTypes::Ipv4 {
        let packet = Ipv4Packet::new(ether.payload()).unwrap();
        if packet.get_next_level_protocol() == IpNextHeaderProtocols::Tcp {
            let source = headertcp.get_source();
            let source_ipv4 = headeripv4.get_source();
            let destination = headertcp.get_destinations();
            let destination_ipv4 = headeripv4.get_destinations();
            let flag = headertcp.get_flags();
            let flag_ipv4 = headeripv4.get_flags();
            let payload = headertcp.get_payload();
            let ttl = headeripv4.get_ttl();
            let version = headeripv4.get_version();
            info!("Source: {} | Ipv4: {} | Destination: {} | Ipv4: {} | Flag: {} | Ipv4: {} | Payload: {:#?} | ttl: {} | Version: {} ",
                            source,
                            source_ipv4,
                            destination,
                            destination_ipv4,
                            flag,
                            flag_ipv4,
                            payload,
                            ttl,
                            version
                        );
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
            let source = headerudp.get_source();
            let source_ipv4 = headeripv4.get_source();
            let destination = headerudp.get_destinations();
            let destination_ipv4 = headeripv4.get_destinations();
            let flag_ipv4 = headeripv4.get_flags();
            let payload = headerudp.get_payload();
            let length = headerudp.get_length();
            let ttl = headeripv4.get_ttl();
            let version = headeripv4.get_version();
            info!(
                "Source: {} | Ipv4: {} | Destination: {} | Ipv4: {} | Flag
     : {} | Ipv4 {} | Payload: {:#?} | ttl: {} | Version: {} ",
                source,
                source_ipv4,
                destination,
                destination_ipv4,
                length,
                flag_ipv4,
                payload,
                ttl,
                version
            );
        } else if packet.get_next_level_protocol() == IpNextHeaderProtocols::Icmp {
            let source_ipv4 = headeripv4.get_source();
            let icmp_type = headericmp.get_icmp_types();
            let destination_ipv4 = headeripv4.get_destinations();
            let flag_ipv4 = headeripv4.get_flags();
            let payload = headericmp.get_payload();
            let ttl = headeripv4.get_ttl();
            let version = headeripv4.get_version();
            info!(
                "Source Ipv4:{} | IcmpType: {:#?}| Destination Ipv4: {} | Flag
       : {} | Payload: {:#?} | ttl: {} | Version: {} ",
                source_ipv4, icmp_type, destination_ipv4, flag_ipv4, payload, ttl, version
            );
        }
    }
}

pub fn ipv6_handler(
    ether: &EthernetPacket,
    headertcp: &'static dyn HeaderDataTcp,
    headeripv6: &'static dyn HeaderDataIpv6,
    headerudp: &'static dyn HeaderDataUdp,
    headericmp: &'static dyn HeaderDataIcmp,
) {
    if ether.get_ethertype() == EtherTypes::Ipv6 {
        let packet = Ipv6Packet::new(ether.payload()).unwrap();
        if packet.get_next_header() == IpNextHeaderProtocols::Tcp {
            let source = headertcp.get_source();
            let source_ipv6 = headeripv6.get_source();
            let destination = headertcp.get_destinations();
            let destination_ipv6 = headeripv6.get_destinations();
            let flag = headertcp.get_flags();
            let payload = headertcp.get_payload();
            let version = headeripv6.get_version();
            info!("Source: {}| Ipv6: {} | Destination: {} | Ipv6: {} | flag: {} | Payload: {:#?} | Version: {} ",
                            source,
                            source_ipv6,
                            destination,
                            destination_ipv6,
                            flag,
                            payload,
                            version
                        );
        } else if packet.get_next_header() == IpNextHeaderProtocols::Udp {
            let source = headerudp.get_source();
            let source_ipv6 = headeripv6.get_source();
            let destination = headerudp.get_destinations();
            let destination_ipv6 = headeripv6.get_destinations();
            let payload = headerudp.get_payload();
            let length = headerudp.get_length();
            let version = headeripv6.get_version();
            info!(
                "Source: {}| Ipv6: {} | Destination: {}| Ipv6: {} | Length: {} | Payload: {:#?} | Version: {} ",
                source,
                source_ipv6,
                destination,
                destination_ipv6,
                length,
                payload,
                version
            );
        } else if packet.get_next_header() == IpNextHeaderProtocols::Icmp {
            let source_ipv6 = headeripv6.get_source();
            let icmp_type = headericmp.get_icmp_types();
            let destination_ipv6 = headeripv6.get_destinations();
            let payload = headericmp.get_payload();
            let version = headeripv6.get_version();
            info!(
                "Source Ipv6:{} | IcmpType: {:#?}| Destination Ipv6: {} | Payload: {:#?} | Version: {} ",
                source_ipv6, icmp_type, destination_ipv6, payload, version
            );
        }
    }
}
