use std::io;
use std::net::*;
use std::u8;
use std::vec;

struct Server {
    addr: Option<String>,
    buffer: Vec<u8>,
    listener: UdpSocket,
}

impl Server {
    fn new(target: &str) -> Result<(), io::ErrorKind> {
        let mut listen = Server {
            addr: Some("s".to_string()),
            buffer: vec![0; 1024],
            listener: UdpSocket::bind(target).unwrap(),
        };

        let resquet = listen.listener.recv_from(&mut listen.buffer).unwrap();

        println!("Listen: {:#?}", resquet);

        Ok(())
    }
}
