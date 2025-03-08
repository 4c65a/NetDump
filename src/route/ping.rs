use pnet::{
    self,
    packet::{
        icmp::{IcmpPacket, IcmpTypes},
        icmpv6::{Icmpv6Packet, Icmpv6Types},
        ip::IpNextHeaderProtocols,
        ipv4::{self, Ipv4Packet},
        ipv6::{self},
        Packet,
    },
    transport::{icmp_packet_iter, icmpv6_packet_iter, transport_channel, TransportChannelType},
};

use super::create_packet::*;
use super::resolve_host::*;
use std::{
    io::{self, Error},
    net::{IpAddr, Ipv6Addr},
    thread,
    time::{Duration, Instant},
};
use termion::color;

pub async fn ping(
    hostname: &str,
    ttl: u8,
    min_send: u64,
    count: Option<i32>,
) -> Result<(), std::io::Error> {
    let ip: IpAddr = hostname.parse().expect("Error: Ip don't working");
    match ip {
        IpAddr::V4(ipv4) => {
            ping_ipv4(&ipv4.to_string(), ttl, min_send, count).await;
        }
        IpAddr::V6(ipv6) => {
            ping_ipv6(ipv6, min_send, count).await;
        }
    };
    Err(Error::new(
        io::ErrorKind::Other,
        "No valid IP address found",
    ))
}

async fn ping_ipv4(hostname: &str, ttl: u8, min_send: u64, count: Option<i32>) {
    let transport_layer3 = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);

    let (mut tx, mut rx) = match pnet::transport::transport_channel(4096, transport_layer3) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => panic!("ERROR TRANSPORT CHANNEL IPV4: {:?}", error),
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
                        Ok(_) => {
                            println!(
                                "{:>15}Packet ipv4 {} sent to {}{}",
                                color::Fg(color::Green),
                                sequence + 1,
                                destination_ip,
                                color::Fg(color::Reset)
                            )
                        }
                        Err(error) => println!("Failed to send packet ipv4: {:?}", error),
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
                                "{}Bytes: {:>2?} | Destination: {:>2?} | TTL: {:>2?} |  Icmp_seq: {:>2?} | Time: {:>2?} ms{}",
                                color::Fg(color::Blue),
                                icmp_bytes,
                                ipv4_packet.get_destination(),
                                ipv4_packet.get_ttl(),
                                sequence + 1,
                                start_time.elapsed().as_millis(),
                                color::Fg(color::Reset)
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


// Temporarily disabled: This function is currently not working.
// I can't test the IPv6 ping because I believe my ISP has not configured IPv6 or doesn't support it.
async fn ping_ipv6(hostname: Ipv6Addr, min_send: u64, count: Option<i32>) {
    
    
    let transport_layer3 = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmpv6);
    let (mut tx, mut rx) = match transport_channel(4096, transport_layer3) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => panic!("Error al crear el canal de transporte: {}", e),
    };


    let mut sequence = 0;
    loop {
        match handle_packet_ipv6(hostname) {
            Ok(ipv6_packet) => {
                if let Some(packet) = ipv6::Ipv6Packet::new(&ipv6_packet) {
                    match tx.send_to(packet, hostname.into()) {
                        Ok(_) => {
                            println!(
                                "{:>15}Packet ipv6 {} sent to {}{}",
                                color::Fg(color::Green),
                                sequence + 1,
                                hostname,
                                color::Fg(color::Reset)
                            )
                        }
                        Err(error) => println!("Failed to send packet ipv6: {:?}", error),
                    }
                } else {
                    println!("Failed to create IPv6 packet");
                }
            }
            Err(error) => println!("Failed to create ipv4_packet: {:?}", error),
        }

        sequence += 1;
        let mut iter = icmpv6_packet_iter(&mut rx);
        let start_time = Instant::now();

        loop {
            match iter.next_with_timeout(Duration::from_secs(1)) {
                Ok(Some((packet, _))) => {
                    let ipv6_packet = ipv6::Ipv6Packet::new(packet.packet())
                        .expect("Failed to parse IPv6 packet");

                    if let Some(icmp_packet) = Icmpv6Packet::new(ipv6_packet.payload()) {
                        let icmp_type = icmp_packet.get_icmpv6_type();

                        if icmp_type == Icmpv6Types::EchoReply {
                            let icmp_payload = ipv6_packet.payload();
                            let icmp_bytes = icmp_payload.len();

                            println!(" ");
                            println!(
                                "{}Bytes: {:>2?} | Destination: {:>2?} | TTL: {:>2?} |  Icmp_seq: {:>2?} | Time: {:>2?} ms{}",
                                color::Fg(color::Blue),
                                icmp_bytes,
                                ipv6_packet.get_destination(),
                                ipv6_packet.get_hop_limit(),
                                sequence + 1,
                                start_time.elapsed().as_millis(),
                                color::Fg(color::Reset)
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
