use std::io::*;
use std::net::*;

struct Server {
    addr: Option<String>,
    listener: UdpSocket,
}


impl Server {
    fn new(target: &str) -> Result<(),io::ErrorKind>{
        let listener = Server{
            addr: Some("somewhere".to_string()),
            listener: UdpSocket::bind(target).unwrap(),
        } 
        
        Ok(())       
    }
}
