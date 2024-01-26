use pnet::packet::{ethernet::EthernetPacket, icmp::Icmp, tcp::Tcp, udp::Udp};

/// Struct for filter protocol
pub struct Filter {
    tcp: Tcp,
    udp: Udp,
    icmp: Icmp
}

impl Filter {
    fn filter_protocol(){
        
    }
}
