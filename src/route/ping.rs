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
        icmpv6::{Icmpv6Packet, Icmpv6Types},
        ip::IpNextHeaderProtocols,
        ipv4::{self, Ipv4Packet},
        ipv6::Ipv6Packet,
        Packet,
    },
    transport::{icmp_packet_iter, TransportChannelType},
};

use super::create_packet::*;
use super::resolve_host::*;
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io::{self, Error},
    mem::MaybeUninit,
    net::{IpAddr, Ipv6Addr, SocketAddrV6},
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
async fn ping_ipv6(hostname: Ipv6Addr, min_send: u64, count: Option<i32>) {
    let socket = Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6))
        .expect("Failed to create socket");

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("Failed to set read timeout");

    let _sock6 = SocketAddrV6::new(hostname, 0, 0, 0);

    let mut sequence = 0;
    let mut buf: Vec<u8> = vec![0; 1024];
    let mut recv_buf: &mut [MaybeUninit<u8>] = unsafe {
        std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut MaybeUninit<u8>, buf.len())
    };

    loop {
        let start_time = Instant::now();

        loop {
            match socket.recv_from(&mut recv_buf) {
                Ok((packet_size, _)) => {
                    let ipv6_packet =
                        Ipv6Packet::new(&buf[..packet_size]).expect("Failed to parse IPv6 packet");

                    if let Some(icmp_packet) = Icmpv6Packet::new(ipv6_packet.payload()) {
                        let icmp6_type = icmp_packet.get_icmpv6_type();

                        if icmp6_type == Icmpv6Types::EchoReply {
                            let icmp_payload = ipv6_packet.payload();
                            let icmp_bytes = icmp_payload.len();
                            println!("Received ICMP type: {:?}", icmp6_type);

                            println!(" ");
                            println!(
                                "                  Bytes: {:?} | Destination: {:?} | TTL: {:?} | Icmp_seq: {:?} | Time: {:?} ms",
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
                Err(error) => {
                    println!(
                        "{}Temporarily disabled: This function is currently not working.{}",
                        color::Fg(color::Red),
                        color::Fg(color::Reset)
                    );
                    eprintln!("Error receiving packet: {:?}", error);
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
