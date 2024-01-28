use crate::protocol::{
    ip::{self, ipv4_handler, ipv6_handler},
    packet::HeaderDataTcp,
};
use core::panic;
use pnet::{
    datalink::{self, interfaces, Channel::Ethernet},
    packet::ethernet::{EtherTypes, EthernetPacket},
};

pub fn interface(int_name: &str) {
    let interface = interfaces();

    let inter = interface
        .into_iter()
        .filter(|inter| inter.name == *int_name)
        .next()
        .expect("Failed to get interface");

    let (tx, mut rx) = match datalink::channel(&inter, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled"),
        Err(e) => panic!("Failed to channel {e}"),
    };

    loop {
        match rx.next() {
            Some(packets) => {
                let packets = EthernetPacket::new(packets).unwrap();

                match packets.get_ethertype() {
                    EtherTypes::Ipv4 => ipv4_handler(&packets),
                    EtherTypes::Ipv6 => ipv6_handler(&packets),
                }
            }
            Err(x) => panic("Failed to ip handler {}", x),
        }
    }
}
