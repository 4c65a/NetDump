use std::time::{Duration, Instant};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::{icmp::IcmpTypes, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, Packet};
use pnet::transport::{icmp_packet_iter, transport_channel, TransportChannelType};
use super::create_packet::*;
use super::ping::resolve_host;

pub async fn trace(ip: &str) {
    let transport_layer = TransportChannelType::Layer3(IpNextHeaderProtocols::Icmp);

    let (mut tx, mut rx) = match transport_channel(8192, transport_layer) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => {
            eprintln!("Error creating transport channel: {:?}", error);
            return;
        }
    };

    let mut ttl: u8 = 1;
    let max_hops = 30;
    let timeout_duration = Duration::from_secs(5);
    let identifier: u16 = 1;
    let mut sequence: u16 = 0;

    println!("Tracing route to {} with a maximum of {} hops:", ip, max_hops);

    loop {
        let destination_ip = match resolve_host(ip).await {
            Ok(ip) => ip,
            Err(_) => {
                eprintln!("Error resolving IP address.");
                break;
            }
        };

        match handle_packet_trace(destination_ip, ttl, identifier, sequence) {
            Ok(packet) => {
                let ipv4_packet = Ipv4Packet::new(&packet).unwrap();

                match tx.send_to(ipv4_packet, destination_ip.into()) {
                    Ok(_) => {
                        println!("Sending ICMP packet to {} (TTL: {})", destination_ip, ttl);
                    }
                    Err(error) => {
                        eprintln!("Error sending ICMP packet: {:?}", error);
                        break;
                    }
                }
            }
            Err(error) => {
                eprintln!("Error creating ICMP packet: {:?}", error);
                break;
            }
        }

        let start_time = Instant::now();
        let mut iter = icmp_packet_iter(&mut rx);

        match iter.next_with_timeout(timeout_duration) {
            Ok(Some((packet, addr))) => {
                if let Some(ipv4_packet) = Ipv4Packet::new(packet.packet()) {
                    if let Some(icmp_packet) = IcmpPacket::new(ipv4_packet.payload()) {
                        match icmp_packet.get_icmp_type() {
                            IcmpTypes::EchoReply => {
                                println!(
                                    "{:>3}. Host: {:?} | Time: {:?} ms, destination: {:?}, ip: {:?}",
                                    ttl,
                                    addr,
                                    start_time.elapsed().as_millis(),
                                    destination_ip,
                                    ip
                                );
                                break;
                            }
                            IcmpTypes::TimeExceeded => {
                                println!(
                                    "{:>3}. Intermediate hop: {:?} | Time: {:?} ms",
                                    ttl,
                                    addr,
                                    start_time.elapsed().as_millis()
                                );
                            }
                            _ => {
                                println!(
                                    "{:>3}. Unexpected ICMP type: {:?}, raw bytes: {:?}",
                                    ttl,
                                    icmp_packet.get_icmp_type(),
                                    icmp_packet.packet()
                                );
                            }
                        }
                    } else {
                        println!("{:>3}. Failed to parse ICMP packet, raw bytes: {:?}", ttl, ipv4_packet.payload());
                    }
                } else {
                    println!("{:>3}. Failed to parse IPv4 packet, raw bytes: {:?}", ttl, packet.packet());
                }
            }
            Ok(None) => {
                println!("{:>3}. Timeout for this request", ttl);
            }
            Err(error) => {
                eprintln!("Error receiving packet: {:?}", error);
                break;
            }
        }

        ttl += 1;
        sequence += 1;

        if ttl > max_hops {
            println!("Reached the maximum of {} hops.", max_hops);
            break;
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
