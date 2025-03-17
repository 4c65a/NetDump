use crate::protocol::ip::ip_handler;
use core::panic;
use pcap::{Capture, Device};
use pnet::
    packet::ethernet::{EtherTypes, EthernetPacket}
;

pub fn cap(int_name: &str,filter: Option<String>) {
    let interfaces =  Device::list().expect("No se pudieron listar las interfaces");

    let inter = interfaces
    .into_iter()
    .find(|d| d.name == int_name)
    .expect("No se encontró la interfaz especificada");

    println!("Listening on interface {}", int_name);

    let mut cap = Capture::from_device(inter.name.as_str()).unwrap().immediate_mode(true).promisc(true).snaplen(65535).open().unwrap();
    
    if let Some(filter_string) = filter {
        cap.filter(&filter_string, true).expect("Error al aplicar el filtro BPF");
    }

    
    loop {
        match cap.next_packet() {
            Ok(packets) => {
                let packets = EthernetPacket::new(&packets).unwrap();
                match packets.get_ethertype() {
                    EtherTypes::Ipv4 => ip_handler(&packets),
                    EtherTypes::Ipv6 => ip_handler(&packets),

                    _ => {
                        println!("Unhandled EtherType: {:?}", packets.get_ethertype());
                    }
                }
            }
            Err(x) => panic!("Failed to ip handler {}", x),
        }
    }
}
