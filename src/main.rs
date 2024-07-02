use route::ping::ping;
//use crate::interface::*;
use std::{env, net::Ipv4Addr};

mod interface;
mod protocol;
mod route;
mod speed;

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--list".to_string()) {
        // Lógica para listar interfaces
        // list_interfaces();
    } else if args.len() >= 3 && args[1] == "--interface" {
        // Lógica para manejar interfaces
        // handle_interface(&args[2]);
    } else if args.len() >= 3 && args[1] == "--ping" {
        // Convertir el argumento del destino en una dirección IPv4
        let destination: String = args[2].parse().expect("Invalid IP address format");
        // Llamar a la función ping
        //let destination_str = destination.to_string();
        ping(&destination).await;
    } else {
        eprintln!("Uso: programa [--list | --interface <name> | -ping <destination>]");
    }
}
