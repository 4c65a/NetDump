use std::io;
use std::net::*;

struct Server {
    addr: Option<String>,
    listener: TcpListener,
}

impl Server {
    pub fn new(target: &str) -> Result<(), io::ErrorKind> {
        let listen = Server {
            addr: Some(target.to_string()),
            listener: TcpListener::bind(target).unwrap(),
        };
        //println!("Connection: {}",target);
        for streams in listen.listener.incoming() {
            match streams {
                Err(expr) => {
                    eprintln!("error: {}", expr)
                }
                Ok(streams) => {
                    eprintln!("stream: {:#?}", streams)
                }
            }
        }
        Ok(())
    }
}
