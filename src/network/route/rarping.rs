use core::str;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

use pnet::datalink;
use pnet::packet::arp::ArpOperations;
use pnet::{
    datalink::{Channel, NetworkInterface, channel},
    packet::{
        Packet,
        arp::ArpPacket,
        ethernet::{EtherTypes, EthernetPacket},
    },
    util::MacAddr,
};
use termion::color;

use crate::route::create_packet::handle_packet_arp;

#[allow(dead_code)]
pub fn rarping(interface: &str, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr) {
    let interfaces = datalink::interfaces();

    let inter = interfaces
        .into_iter()
        .find(|i: &NetworkInterface| i.name == *interface);

    let (mut tx, mut rx) = match channel(&inter.clone().unwrap(), Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to open channel: {:?}", e),
    };

    let packet =
        handle_packet_arp(source_ip, source_mac, target_ip).expect("Failed to build ARP packet");

    match tx.send_to(&packet, Some(inter.expect(""))) {
        Some(Ok(_)) => {
            println!(
                "{}ARP packet sent successfully: {:?} from {:?} on interface {}{}",
                color::Fg(color::Green),
                target_ip,
                source_ip,
                interface,
                color::Fg(color::Reset),
            )
        }
        Some(Err(e)) => eprintln!("Failed to send ARP packet: {:?}", e),
        None => eprintln!("Could not send ARP packet."),
    }

    let start = Instant::now();
    let timeout = Duration::from_secs(5);

    loop {
        if start.elapsed() > timeout {
            println!("ARP response timeout.");
            break;
        }

        match rx.next() {
            Ok(received) => {
                if let Some(ethernet) = EthernetPacket::new(received) {
                    if ethernet.get_ethertype() == EtherTypes::Arp {
                        if let Some(arp) = ArpPacket::new(ethernet.payload()) {
                            if arp.get_operation() == ArpOperations::Reply
                                && arp.get_sender_proto_addr() == target_ip
                            {
                                println!(
                                    "{}ARP Reply from: IP: {:?} -> MAC: {:?}{}",
                                    color::Fg(color::Magenta),
                                    arp.get_sender_proto_addr(),
                                    arp.get_sender_hw_addr(),
                                    color::Fg(color::Reset),
                                );
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "{}Error receiving packet: {:?}{}",
                    color::Fg(color::Red),
                    e,
                    color::Fg(color::Reset),
                );
                break;
            }
        }
    }
}
