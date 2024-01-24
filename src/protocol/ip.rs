use pnet::packet::{ethernet::{EthernetPacket,EtherTypes}, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, Packet};
use super::packet::{HeaderDataIpv4, HeaderDataTcp};
use log::{self, info};

fn ipv4(ether: &EthernetPacket, headertcp: &dyn HeaderDataTcp, headeripv4: &dyn HeaderDataIpv4){
    match ether.get_ethertype() {
        EtherTypes::Ipv4 => {
            if let Some(packet) = Ipv4Packet::new(ether.payload()) {
                match packet.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let source = headertcp.get_source();
                        let source_ipv4 = headeripv4.get_source(); 
                        let destination = headertcp.get_destinations();
                        let destination_ipv4 = headeripv4.get_destinations();
                        let flag = headertcp.get_flags();
                        let flag_ipv4 = headeripv4.get_flags();
                        let payload = headertcp.get_payload();
                        let ttl = headeripv4.get_ttl();
                        let version = headeripv4.get_version();
                        info!("Source: {}|Ipv4{} | Destination: {}|Ipv4{} | Flag: {}|Ipv4{} | Payload: {:#?} | ttl: {} | Version: {} ",
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
                            
                    }
                }
            }
        }
    }
}

fn ipv6() {
    
}


