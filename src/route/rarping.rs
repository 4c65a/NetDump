use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

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

use crate::route::create_packet::handle_packet_arp;

#[allow(dead_code)]
pub fn rarping(
    interface: NetworkInterface,
    source_ip: Ipv4Addr,
    source_mac: MacAddr,
    target_ip: Ipv4Addr,
) {
    let (mut tx, mut rx) = match channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to open channel: {:?}", e),
    };

    let packet =
        handle_packet_arp(source_ip, source_mac, target_ip).expect("Failed to build ARP packet");

    match tx.send_to(&packet, Some(interface.clone())) {
        Some(Ok(_)) => {
            println!(
                "ARP packet sent successfully: {:?} from {:?} {:?}",
                target_ip, source_ip, interface
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
                                    "ARP Request from: source_ip: {:?} -> Source_mac: {:?}",
                                    arp.get_target_proto_addr(),
                                    arp.get_target_hw_addr(),
                                );
                                println!(
                                    "ARP Reply from: Target_ip: {:?} -> Target_mac: {:?}",
                                    arp.get_sender_proto_addr(),
                                    arp.get_sender_hw_addr(),
                                );
                                break;
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving packet: {:?}", e);
                break;
            }
        }
    }
}

