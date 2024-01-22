use super::packet::{HeaderDataTcp, HeaderDataIpv4};
use log::{info, warn};

pub fn tcp_handler(packet: &dyn HeaderDataIpv4<'2>) -> bool {
    let tcp = pnet::packet::tcp::TcpPacket::new(packet.get_payload());


    let source_ip = packet.get_source();
    let destination_ip = packet.get_destinations();
    let flags_connection = packet.get_flags();

    
    if let Some(tcp) = tcp{
        info!("Source: {:?} | Destination: {} | flag: {} | Protocol: TCP",source_ip,destination_ip,flags_connection);
    } else {
        warn!("No se esta recibiendo ningun paquete");
    }
    true
}
