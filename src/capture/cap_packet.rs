use crate::protocol::ip::{ipv4_handler, ipv6_handler};
use core::panic;
use pnet::{
    datalink::{self, interfaces, Channel::Ethernet, NetworkInterface},
    packet::ethernet::{EtherTypes, EthernetPacket},
};

//Change name file network_interface.rs to packet_capture.rs
#[allow(dead_code)]
pub fn cap(int_name: &str) {
    let interface = interfaces();
    // let int_name_str = int_name.to_string();
    let inter = interface
        .into_iter()
        .find(|inters: &NetworkInterface| inters.name == int_name)
        .expect("Failed to get interface");

    let (_tx, mut rx) = match datalink::channel(&inter, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled"),
        Err(e) => panic!("Failed to channel {e}"),
    };
    println!("Listening on interface {}", int_name);
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
