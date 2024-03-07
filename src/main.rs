use crate::interface::list_interface;
use std::env;

mod interface;
mod protocol;
mod route;
mod speed;

fn main() {
    // Parse command-line arguments using a dedicated library
    let args: Vec<String> = env::args().collect();

    // Handle --list argument
    if args.contains(&"--list".to_string()) {
        let index = list_interface::index_interface();
        for name in index {
            println!("- {}", name);
        }
    } else if args.len() >= 3 && args[1] == "--interface" {
        // Access interface name if provided after --interface
        let int_name = &args[2];

        // Call the interface function to handle packet capturing
        interface::network_interfaces::interface(int_name);
    } else {
        println!("-------------------------------------------------------------------------------------------------------");
        println!(
            " ██████   █████           █████    ██████████                                       
░░██████ ░░███           ░░███    ░░███░░░░███                                      
 ░███░███ ░███   ██████  ███████   ░███   ░░███ █████ ████ █████████████   ████████ 
 ░███░░███░███  ███░░███░░░███░    ░███    ░███░░███ ░███ ░░███░░███░░███ ░░███░░███
 ░███ ░░██████ ░███████   ░███     ░███    ░███ ░███ ░███  ░███ ░███ ░███  ░███ ░███
 ░███  ░░█████ ░███░░░    ░███ ███ ░███    ███  ░███ ░███  ░███ ░███ ░███  ░███ ░███
 █████  ░░█████░░██████   ░░█████  ██████████   ░░████████ █████░███ █████ ░███████ 
░░░░░    ░░░░░  ░░░░░░     ░░░░░  ░░░░░░░░░░     ░░░░░░░░ ░░░░░ ░░░ ░░░░░  ░███░░░  
                                                                           ░███     
                                                                           █████    
                                                                          ░░░░░     "
        );
        println!("-------------------------------------------------------------------------------------------------------");
        println!("Usage: netdump [--list] [--interface <interface_name>] [--ping <ip>] [--traceroute <ip>] [--wifi_speed]");
        println!("-------------------------------------------------------------------------------------------------------");
    }
}
