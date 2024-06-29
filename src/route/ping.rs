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
    packet::{icmp::IcmpTypes, ip::IpNextHeaderProtocols, ipv4},
    transport::{icmp_packet_iter, TransportChannelType},
};

use std::{
    io,
    net::{Ipv4Addr, ToSocketAddrs},
    str::FromStr,
    thread,
    time::{Duration, Instant},
};

use super::create_packet::create_packet;

fn resolve_host(hostname: &str) -> Result<Ipv4Addr, io::Error> {
    if let Ok(ip) = Ipv4Addr::from_str(hostname) {
        return Ok(ip);
    }

    let addr = format!("{}:80", hostname);
    let socket_addr = addr.to_socket_addrs()?.next().unwrap();

    if let std::net::SocketAddr::V4(socket_addr_v4) = socket_addr {
        Ok(*socket_addr_v4.ip())
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid hostname",
        ))
    }
}

//Add
pub fn ping(hostname: &str) {
    let transport_ipv4 = TransportChannelType::Layer4(pnet::transport::TransportProtocol::Ipv4(
        IpNextHeaderProtocols::Icmp,
    ));

    let (mut tx, mut rx) = match pnet::transport::transport_channel(4096, transport_ipv4) {
        Ok((tx, rx)) => (tx, rx),
        Err(error) => panic!("ERROR TRANSPORT CHANNEL: {:?}", error),
    };

    let mut sequence = 0;

    loop {
        let destination_ip = resolve_host(hostname).expect("Failed to resolve hostname");
        //let ipv4_packet: Result<Vec<u8>, std::io::Error> = create_packet_ipv4(destination_ip);
        //let ipv4_packet = create_packet_ipv4(destination_ip);

        //match tx.send_to(&ipv4_packet, destination_ip.into()) {
        //   Ok(_) => println!("Packet {} sent to {}", sequence + 1, destination_ip),
        //    Err(error) => println!("Failed to send packet: {:?}", error),
        //}

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
            match iter.next_with_timeout(Duration::from_secs(1)) {
                Ok(Some((packet, _))) => {
                    if packet.get_icmp_type() == IcmpTypes::EchoReply {
                        println!("Received ICMP echo reply in {:?}", start_time.elapsed());
                        break;
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
