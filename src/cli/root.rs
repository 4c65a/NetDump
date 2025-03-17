use clap::{self, error::Result, Arg, ArgAction, ArgMatches, Command, Error};

pub fn cmd() -> Result<ArgMatches, Error> {
    let matches = Command::new("netdump")
        .author("Lean")
        .about("NetDump is a command tool in the terminal, it has multiple functionalities.")
        .version("0.1.0")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(clap::ColorChoice::Auto)
       
        .subcommand(
            Command::new("cap")
                .about("Capture packets from your network.")
                .arg(
                    Arg::new("interface")
                        .short('i')
                        .long("interface")
                        .value_name("INTERFACE")
                        .action(ArgAction::Set)
                        .required(true)
                        .help("Specify the network interface to capture packets from."),
                )
                .arg(
                    Arg::new("filter")
                        .short('f')
                        .long("filter")
                        .value_name("FILTER")
                        .action(ArgAction::Set)
                        .required(false)
                        .help("Apply a BPF filter ('icmp', 'tcp', 'udp', 'tcp port 80', 'host 192.168.1.1').
                         Use quotes for multi-word filters ('icmp or tcp or udp').")

                ),
        )
        
        .subcommand(
            Command::new("interface")
                .about("Get a list of available network interfaces.")
                .arg(
                    Arg::new("list")
                        .short('l')
                        .long("list")
                        .value_name("LIST")
                        .help("Get the list of interfaces of your system")
                        .action(ArgAction::SetFalse),
                )
                .arg(
                    Arg::new("filter")
                        .short('t')
                        .long("filter")
                        .value_name("FILTER")
                        .help("Get the list of interfaces on your system by filtering by name.")
                        .action(ArgAction::Set),
                ),
        )
        
        .subcommand(
            Command::new("ping")
                .about("Ping sends an Internet Control Message Protocol (ICMP).")
                .arg(
                    Arg::new("destination")
                        .short('d')
                        .required(false)
                        .long("destination")
                        .value_name("DESTINATION")
                        .action(ArgAction::Set)
                        .default_value("127.0.0.1") 
                        .help("Address server."),
                )
                .arg(
                    Arg::new("ttl")
                        .short('t')
                        .required(false)
                        .value_name("TTL <number>")
                        .long("ttl")
                        .action(ArgAction::Set)
                        .default_value("64")
                        .help("Define time to live"),
                )
                .arg(
                    Arg::new("min_send")
                        .short('m')
                        .required(false)
                        .value_name("MIN-SEND <number>")
                        .long("min-send")
                        .action(ArgAction::Set)
                        .default_value("1")
                        .help("Time for each packet to be sent <seconds>."),
                )
                .arg(
                    Arg::new("count")
                        .short('c')
                        .required(false)
                        .value_name("COUNT <number>")
                        .long("count")
                        .action(ArgAction::Set)
                        .help("Stop after <count> replies"),
                )
                .arg(
                    Arg::new("ipv6")
                        .short('6')
                        .required(false)
                        .value_name("DESTINATION IPV6")
                        .long("ipv6")
                        .action(ArgAction::Set)
                        .help("Send ICMPv6 packet to the specified IPv6 address (Note: IPv6 functionality is currently disabled and will be enabled in future versions"),
                )
        )
          .subcommand(
            Command::new("tracerouter")
                .about("Performs a traceroute to the given IP address.")
                .arg(
                    Arg::new("trace")
                        .short('r')
                        .long("trace")
                        .value_name("IP")
                        .help("Add an IP address for the traceroute.")
                        .required(true)
               ),
        )
        .subcommand(
            Command::new("resolve")
            .about("Resolve the IP of a host")
            .arg(
                Arg::new("host")
                .short('d')
                .value_name("HOST")
                .help("Resolve the IP of a host")
            ),
        )
        .get_matches();
    Ok(matches)
}
