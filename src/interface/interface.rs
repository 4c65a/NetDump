use crate::protocol::ip::{ipv4_handler, ipv6_handler};
use core::panic;
use pnet::{
    datalink::{self, interfaces, Channel::Ethernet},
    packet::ethernet::{EtherTypes, EthernetPacket},
};

pub fn interface(int_name: &String) {
    let interface = interfaces();

    let inter = interface
        .into_iter()
        .find(|inter| inter.name == *int_name)
        .expect("Failed to get interface");

    let (_, mut rx) = match datalink::channel(&inter, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled"),
        Err(e) => panic!("Failed to channel {e}"),
    };

    loop {
        match rx.next() {
            Ok(packets) => {
                let packets = EthernetPacket::new(packets).unwrap();
                match packets.get_ethertype() {
                    EtherTypes::Ipv4 => ipv4_handler(&packets),
                    EtherTypes::Ipv6 => ipv6_handler(&packets),
                    _ => {
                        println!("Unhandled EtherType: {:?}", packets.get_ethertype());
                    }
                }
            }
            Err(x) => panic!("Failed to ip handler {}", x),
        }
    }
}
