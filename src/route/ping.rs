/*
    Input IP
    Create packet ICMP >> Ipv4 >> icmp6 >> ipv6
    Socket connection
    Send packet ICMP
    Receive packet ICMP
    Output
*/

use pnet::{
    self,
    packet::{
        icmp::{IcmpPacket, IcmpTypes},
        ip::IpNextHeaderProtocols,
        ipv4::{self, Ipv4Packet},
        Packet,
    },
    transport::{icmp_packet_iter, TransportChannelType},
};
use std::{
    io::{self, Error},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    thread,
    time::{Duration, Instant},
    str::FromStr,
};

use super::create_packet::*;
use tokio::net::lookup_host;


async fn ping(hostname: &str, ttl: u8, min_send: u64, count: Option<i32>){



}





async fn resolve_host(hostname: &str) -> Result<Ipv4Addr, io::Error> {
    let host_port = format!("{hostname}:0");
    let mut addresses = lookup_host(host_port).await?;

    if let Some(addr) = &addresses.next() {
        match addr.ip() {
            IpAddr::V4(ip) => return Ok(ip),
            IpAddr::V6(_) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    "Resolved to an IPv6 address which is not supported",
                ))
            }
        }
    }

    Err(Error::new(
        io::ErrorKind::Other,
        "No valid IP address found",
    ))
}

async fn ping_ipv4(hostname: &str, ttl: u8, min_send: u64, count: Option<i32>) {
    let transport_layer3 = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);

    let (mut tx, mut rx) = match pnet::transport::transport_channel(4096, transport_layer3) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => panic!("ERROR TRANSPORT CHANNEL: {:?}", error),
    };

    let mut sequence = 0;


    loop {
        let destination_ip = resolve_host(hostname)
            .await
            .expect("Failed to resolve hostname");

        match handle_packet(destination_ip, ttl) {
            Ok(ipv4_packet) => {
                if let Some(packet) = ipv4::Ipv4Packet::new(&ipv4_packet) {
                    match tx.send_to(packet, destination_ip.into()) {
                        Ok(_) => println!("------------------------------ Packet ipv4 {} sent to {} ------------------------------", sequence + 1, destination_ip),
                        Err(error) => println!("Failed to send packet: {:?}", error),
                    }
                } else {
                    println!("Failed to create IPv4 packet");
                }
            }
            Err(error) => println!("Failed to create ipv4_packet: {:?}", error),
        }

        let mut iter = icmp_packet_iter(&mut rx);
        let start_time = Instant::now();

        loop {

            match iter.next_with_timeout(Duration::from_secs(1)) {
                Ok(Some((packet, _))) => {
                    let ipv4_packet =
                        Ipv4Packet::new(packet.packet()).expect("Failed to parse IPv4 packet");
                    if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                        let icmp_type = icmp_packet.get_icmp_type();
                        if icmp_type == IcmpTypes::EchoReply {
                            let icmp_payload = ipv4_packet.payload();
                            let icmp_bytes = icmp_payload.len();

                            println!(" ");
                            println!(
                                "                  Bytes: {:?} | Destination: {:?} | TTL: {:?} |  Icmp_seq: {:?} | Time: {:?} ms",
                                icmp_bytes,
                                ipv4_packet.get_destination(),
                                ipv4_packet.get_ttl(),
                                sequence + 1,
                                start_time.elapsed().as_millis()
                            );
                            println!(" ");
                        }
                        break;
                    } else {
                        eprintln!("Received a non-ICMP packet");
                    }
                }
                Ok(None) => {
                    println!("Timeout waiting for ICMP echo reply");
                    break;
                }
                Err(e) => {
                    println!("An error occurred while waiting for ICMP echo reply: {}", e);
                    break;
                }
            }
        }

        sequence += 1;

        if let Some(max_count) = count {
            if sequence >= max_count {
                println!("Count Packet: {}", max_count);
                break;
            }
        }

        thread::sleep(Duration::from_secs(min_send));
    }
}


async fn ping_ipv6(hostname: &str, min_send: u64, count: Option<i32>) {
    let transport_layer3 = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);

    let (mut tx, mut rx) = match pnet::transport::transport_channel(4096, transport_layer3) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => panic!("ERROR TRANSPORT CHANNEL: {:?}", error),
    };
    
    let destination_ip6 = Ipv6Addr::from_str(hostname).expect("Failed to parse destination IP as IPv6");



    let mut sequence = 0;


    loop {
    match handle_packet_ipv6(destination_ip6) {
            Ok(ipv6_packet) => {
                if let Some(packet) = pnet::packet::ipv6::Ipv6Packet::new(&ipv6_packet) {
                    match tx.send_to(packet, destination_ip6.into()) {
                        Ok(_) => println!("------------------------------ Packet ipv6 {} sent to {} ------------------------------", sequence + 1, destination_ip6),
                        Err(error) => println!("Failed to send packet: {:?}", error),
                    }
                } else {
                    println!("Failed to create IPv4 packet");
                }
            }
            Err(error) => println!("Failed to create ipv4_packet: {:?}", error),
        }

        let mut iter = pnet::transport::icmpv6_packet_iter(&mut rx);
        let start_time = Instant::now();

        loop {

            match iter.next_with_timeout(Duration::from_secs(1)) {
                Ok(Some((packet, _))) => {
                    let ipv6_packet =
                        pnet::packet::ipv6::Ipv6Packet::new(packet.packet()).expect("Failed to parse IPv4 packet");
                    if let Some(icmp_packet) = IcmpPacket::new(ipv6_packet.payload()) {
                        let icmp_type = icmp_packet.get_icmp_type();
                        if icmp_type == IcmpTypes::EchoReply {
                            let icmp_payload = ipv6_packet.payload();
                            let icmp_bytes = icmp_payload.len();

                            println!(" ");
                            println!(
                                "                  Bytes: {:?} | Destination: {:?} | TTL: {:?} |  Icmp_seq: {:?} | Time: {:?} ms",
                                icmp_bytes,
                                ipv6_packet.get_destination(),
                                ipv6_packet.get_hop_limit(),
                                sequence + 1,
                                start_time.elapsed().as_millis()
                            );
                            println!(" ");
                        }
                        break;
                    } else {
                        eprintln!("Received a non-ICMP packet");
                    }
                }
                Ok(None) => {
                    println!("Timeout waiting for ICMP echo reply");
                    break;
                }
                Err(e) => {
                    println!("An error occurred while waiting for ICMP echo reply: {}", e);
                    break;
                }
            }
        }

        sequence += 1;

        if let Some(max_count) = count {
            if sequence >= max_count {
                println!("Count Packet: {}", max_count);
                break;
            }
        }

        thread::sleep(Duration::from_secs(min_send));
    }
}
