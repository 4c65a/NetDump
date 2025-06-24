use std::{
    io::{self, Error},
    net::{IpAddr, Ipv4Addr},
};

use tokio::net::lookup_host;

pub async fn resolve_host(hostname: &str) -> Result<Ipv4Addr, io::Error> {
    let host_port = format!("{hostname}:0");
    let mut addresses = lookup_host(host_port).await?;

    if let Some(addr) = &addresses.next() {
        match addr.ip() {
            IpAddr::V4(ip) => return Ok(ip),
            IpAddr::V6(_) => {
                return Err(Error::new(
                    io::ErrorKind::Other,
                    "Resolved to an IPv6 address which is not supported",
                ));
            }
        }
    }

    Err(Error::new(
        io::ErrorKind::Other,
        "No valid IP address found",
    ))
}
