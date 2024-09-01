use capture::{cap_packet::cap, interfaces};
use cli::root::cmd;
use route::ping::ping;

mod capture;
mod cli;
mod protocol;
mod route;
mod speed;

#[tokio::main]
async fn main() {
    let matches = cmd().unwrap();
    match matches.subcommand() {
        // Command cap with subcommand --interface
        Some(("cap", cap_matches)) => match cap_matches.get_one::<String>("interface") {
            Some(interface) => {
                cap(interface);
            }
            _ => {
                println!("Error capturing packets.");
            }
        },
        // Command interface with two subcommand --list | --filter
        Some(("interface", inter_matches)) => {
            match inter_matches.get_one::<bool>("list") {
                Some(_) => {
                    let _ = interfaces::index_interface();
                }
                _ => {
                    println!("Error when displaying interfaces.");
                }
            }
            match inter_matches.get_one::<String>("filter") {
                Some(filter) => {
                    interfaces::filter_interfaces(filter).unwrap();
                }
                _ => {
                    println!("Error when displaying interfaces.");
                }
            }
        }
        // Command ping with subcommand --destination
        Some(("ping", ping_matches)) => match ping_matches.get_one::<String>("destination") {
            Some(destination) => {
                ping(destination).await;
            }
            _ => {
                println!("Error send ping.");
            }
        },
        _ => {
            println!("No commands found");
        }
    }
}
