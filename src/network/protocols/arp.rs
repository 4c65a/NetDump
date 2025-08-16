use std::net::Ipv4Addr;

use pnet::{
    packet::{
        Packet,
        arp::ArpPacket,
        ethernet::{EtherTypes, EthernetPacket},
    },
    util::MacAddr,
};
use termion::color;

fn print_info(
    sender_mac: &MacAddr,
    target_mac: &MacAddr,
    sender_ip: &Ipv4Addr,
    target_ip: &Ipv4Addr,
) {
    println!(
        "{}Sender_MAC: {:>15} | Target_MAC: {:>15} | Sender_IP: {:>3} | Target_IP: {:>2}{}",
        color::Fg(color::Green),
        sender_mac,
        target_mac,
        sender_ip,
        target_ip,
        color::Fg(color::Reset),
    );
}

pub fn arp_handler(ether: &EthernetPacket) {
    match ether.get_ethertype() {
        EtherTypes::Arp => {
            let packet = ArpPacket::new(ether.payload()).unwrap();

            let sender_mac = packet.get_sender_hw_addr();
            let sender_ip = packet.get_sender_proto_addr();
            let target_mac = packet.get_target_hw_addr();
            let target_ip = packet.get_target_proto_addr();

            print_info(&sender_mac, &target_mac, &sender_ip, &target_ip);
        }
        _ => {
            println!("Unhandled EtherType");
        }
    }
}
