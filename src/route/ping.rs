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
    net::{IpAddr, Ipv4Addr},
    thread,
    time::{Duration, Instant},
};

use tokio::net::lookup_host;

use super::create_packet::handle_packet;

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

pub async fn ping(hostname: &str) {
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

        match handle_packet(destination_ip) {
            Ok(ipv4_packet) => {
                if let Some(packet) = ipv4::Ipv4Packet::new(&ipv4_packet) {
                    match tx.send_to(packet, destination_ip.into()) {
                        Ok(_) => println!("---------------------------------------- Packet {} sent to {} ----------------------------------------", sequence + 1, destination_ip),
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
            //Envia paquete cada 3 o X segundos
            //Utilizar echo_reply para recibir la respuesta del dispositivo
            match iter.next_with_timeout(Duration::from_secs(3)) {
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
                                destination_ip,
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
        thread::sleep(Duration::from_secs(1));
    }
}
