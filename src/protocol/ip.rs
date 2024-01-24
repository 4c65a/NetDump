use pnet::packet::{ethernet::{EthernetPacket,EtherTypes}, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, tcp::TcpPacket, Packet};
use super::packet::{HeaderDataIpv4, HeaderDataTcp};
use log;

fn ipv4(ether: &EthernetPacket, headertcp: &dyn HeaderDataTcp){
    match ether.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(packet) = Ipv4Packet::new(ether.payload()) {
                match packet.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        
                        
                            
                    }
                }
            }
        }
    }
}

fn ipv6() {
    
}


