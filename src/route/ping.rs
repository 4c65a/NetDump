// https://docs.rs/pnet/latest/pnet/
/*
    Example input
    netdump 192.126.215
    netdump hola.com > 192.126.215
    netdump hola.com -count 10 (packet)
*/

/*
    Example Output
    ping -c 1 google.com
    PING google.com (172.217.172.110) 56(84) bytes of data.
    64 bytes from eze06s02-in-f14.1e100.net (172.217.172.110): icmp_seq=1 ttl=58 time=51.3 ms

    --- google.com ping statistics ---
    1 packets transmitted, 1 received, 0% packet loss, time 0ms
    rtt min/avg/max/mdev = 51.296/51.296/51.296/0.000 ms
*/

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
        icmp::{echo_reply::EchoReplyPacket, echo_request::EchoRequestPacket, IcmpTypes},
        ip::IpNextHeaderProtocols,
        ipv4, Packet,
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

use super::create_packet::create_packet;

async fn resolve_host(hostname: &str) -> Result<Ipv4Addr, io::Error> {
    //if let Ok(ip_addr) = IpAddr::V4(ip_addr) {
    //    return Ok(ip_addr);
    //}
    // if let Ok(ip) = Ipv4Addr::from_str(hostname) {

    // return Ok(ip);
    // }

    let host_port = format!("{hostname}:0");
    let mut addresses = lookup_host(host_port).await?;

    if let Some(addr) = &addresses.next() {
        match addr.ip() {
            IpAddr::V4(ip) => return Ok(ip),
            IpAddr::V6(_) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    "ROS_HOSTNAME resolved to an IPv6 address which is not supported",
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
    let transport_ipv4 = TransportChannelType::Layer4(pnet::transport::TransportProtocol::Ipv4(
        IpNextHeaderProtocols::Icmp,
    ));

    let (mut tx, mut rx) = match pnet::transport::transport_channel(4096, transport_ipv4) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => panic!("ERROR TRANSPORT CHANNEL: {:?}", error),
    };

    let mut sequence = 0;

    loop {
        let destination_ip = resolve_host(hostname)
            .await
            .expect("Failed to resolve hostname");

        match create_packet(destination_ip) {
            Ok(ipv4_packet) => {
                if let Some(packet) = ipv4::Ipv4Packet::new(&ipv4_packet) {
                    match tx.send_to(packet, destination_ip.into()) {
                        Ok(_) => println!("Packet {} sent to {}", sequence + 1, destination_ip),
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
            match iter.next_with_timeout(Duration::from_secs(3)) {
                Ok(Some((packet, addr))) => {
                    if let Some(reply) = EchoReplyPacket::new(packet.packet()) {
                        println!(
                            "ICMP EchoReply received from {:?}: {:?}, Time: {:?}",
                            addr,
                            reply.get_icmp_type(),
                            start_time.elapsed()
                        );
                    } else {
                        println!("Failed to parse EchoReply packet");
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
