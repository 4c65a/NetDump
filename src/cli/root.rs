use clap::{self, Arg, ArgAction, ArgMatches, Command, Error, error::Result, value_parser};

pub fn cmd() -> Result<ArgMatches, Error> {
    let matches = Command::new("netdump")
        .author("Lean")
        .about("NetDump is a command tool in the terminal, it has multiple functionalities.")
        .version("0.2.0")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(clap::ColorChoice::Auto)

        .subcommand(
            Command::new("cap")
                .about("Capture packets from your network.")
                .long_about(
                    "Capture packets from your network interface using BPF filters. Example filters include 'icmp', 'tcp', or 'host 192.168.1.1'."
                )
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
                        .help("Apply a BPF filter ('icmp', 'tcp', 'udp', 'tcp port 80', 'host 192.168.1.1'). Use quotes for multi-word filters ('icmp or tcp or udp').")
                )
                .after_help(
                    "Example usage: netdump cap -i eth0 -f 'tcp or udp'"
                ),
        )

        .subcommand(
            Command::new("interface")
                .about("Get a list of available network interfaces.")
                .long_about("Get a list of network interfaces on your system. Optionally, filter by name.")
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
                        .short('f')
                        .long("filter")
                        .value_name("FILTER")
                        .help("Get the list of interfaces on your system by filtering by name.")
                        .action(ArgAction::Set),
                )
                .after_help(
                    "Example usage: netdump interface -l\nExample with filter: netdump interface -t eth0"
                ),
        )

        .subcommand(
            Command::new("ping")
                .about("Ping sends an Internet Control Message Protocol (ICMP).")
                .long_about(
                    "Ping a destination IP address using ICMP protocol. By default, it pings '127.0.0.1'. You can define TTL, count, and destination IP."
                )
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
                        .value_parser(value_parser!(u32).range(1..))
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
                        .help("Send ICMPv6 packet to the specified IPv6 address (Note: IPv6 functionality is currently disabled and will be enabled in future versions)")
                )
                .hide(true)
                .after_help(
                    "Example usage: netdump ping -d 8.8.8.8 -t 64\nExample with IPv6 (currently disabled): netdump ping -d 2001:db8::1 --ipv6"
                ),
        )

        .subcommand(
            Command::new("tracerouter")
                .about("Performs a traceroute to the given IP address.")
                .long_about("Perform a traceroute to an IP address to discover the route packets take.")
                .arg(
                    Arg::new("trace")
                        .short('r')
                        .long("trace")
                        .value_name("IP")
                        .help("Add an IP address for the traceroute.")
                        .required(true)
               )
               .after_help(
                    "Example usage: netdump tracerouter -r 8.8.8.8"
               ),
        )

        .subcommand(
            Command::new("resolve")
                .about("Resolve the IP of a host")
                .long_about("Resolve the IP address of a hostname.")
                .arg(
                    Arg::new("host")
                        .short('d')
                        .value_name("HOST")
                        .help("Resolve the IP of a host")
                        .required(true)
                )
                .after_help(
                    "Example usage: netdump resolve -d example.com"
                ),
        )

        .subcommand(
                  Command::new("rarping")
                .about("Send an ARP request to a target IP to retrieve its MAC address.")
                .arg(
                    Arg::new("interface")
                        .short('i')
                        .long("interface")
                        .value_name("INTERFACE")
                        .help("The network interface to use (e.g., eth0)")
                        .required(false),
                )
                .arg(
                    Arg::new("ip")
                        .short('s')
                        .long("ip")
                        .value_name("IP_ADDR")
                        .help("The source IP address (e.g., 192.168.1.10)")
                        .required(false),
                )
                .arg(
                    Arg::new("mac")
                        .short('m')
                        .long("mac")
                        .value_name("MAC_ADDR")
                        .help("The source MAC address (e.g., 00:00:00:00:00:00)")
                        .required(false),
                )
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target-ip")
                        .value_name("IP_ADDR")
                        .help("The target IP address (e.g., 192.168.1.10)")
                        .required(false),
                )

                .after_help(
                    "Example usage:\n  netdump rarping -i eth0 -s 192.168.1.10 -m 00:00:00:00:00:00 -t  10.0.0.0"
                ),

            )

        .get_matches();
    Ok(matches)
}
