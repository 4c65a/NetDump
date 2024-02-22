use crate::interface::list_interface;
use clap::{Arg, Command};
use log::info;
mod interface;
mod protocol;

fn main() {
    // let matches = cmd().get_matches_from(std::env::args());
    let _matches = Command::new("NetDump")
        .version("0.0.1")
        .about("Capture networks packet")
        .author("Leandro")
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List of system interfaces"),
        )
        .subcommand(
            Command::new("interface")
                .about("Sets the network interface for packet capture")
                .arg(
                    Arg::new("interface")
                        .short('i')
                        .long("interface")
                        .value_name("INTERFACE")
                        .help("Specifies the network interface"),
                ),
        )
        .get_matches();

    match _matches.subcommand() {
        Some(("list", _)) => {
            let index = list_interface::index_interface();
            for name in index {
                println!("- {}", name);
            }
        }
        Some(("interface", m)) => {
            let int_name = m;
            interface::interface::interface(int_name);
        }
        _ => info!("Clap don't command"),
    };
}
