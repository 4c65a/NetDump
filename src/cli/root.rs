use clap::{self, error::Result, Arg, ArgAction, ArgMatches, Command, Error};

pub fn cmd() -> Result<ArgMatches, Error> {
    let matches = Command::new("netdump")
        .author("Lean")
        .about("NetDump is a command tool in the terminal, it has multiple functionalities.")
        .version("0.1.0")
        .propagate_version(true)
        .arg_required_else_help(true)
        .color(clap::ColorChoice::Auto)
        // cap --interface enp2s0|any --count 10 --cap-send --cap-receive --protocol --port --address 192.168.0.0.1 --save cap.pcap
        .subcommand(
            Command::new("cap")
                .about("Capture packets from your network.")
                .arg(
                    Arg::new("interface")
                        .short('i')
                        .long("interface")
                        .value_name("INTERFACE")
                        .action(ArgAction::Set)
                        .help("Add interface type to capture packets."),
                ),
        )
        // Interface --filter(type interface) --list(interface) --ip(ip private or public)
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
        // ping --address --min-send --count(packet) --ip6 --save --IcmpS --Icmp6S --ttl
        .subcommand(
            Command::new("ping")
                .about("Ping sends an Internet Control Message Protocol (ICMP).")
                .arg(
                    Arg::new("destination")
                        .short('d')
                        .required(true)
                        .long("destination")
                        .value_name("DESTINATION")
                        .action(ArgAction::Set)
                        .help("Address server."),
                )
                .arg(
                    Arg::new("ttl")
                        .short('t')
                        .required(false)
                        .value_name("TTL <number>")
                        .long("ttl")
                        .action(ArgAction::Set)
                        .help("Define time to live"),
                )
                .arg(
                    Arg::new("min-send")
                        .short('m')
                        .required(false)
                        .value_name("MIN-SEND <number>")
                        .long("min-send")
                        .action(ArgAction::Set)
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
                        .help("Send packet icmp6 to ipv6"),
                )
                .arg(Arg::new("save").short('o').long("save").help("Save result"))
                .arg(
                    Arg::new("icmpS")
                        .short('s')
                        .required(false)
                        .value_name("ICMP SIZE <SIZE BYTES>")
                        .long("icmp")
                        .action(ArgAction::Set)
                        .help("Icmp size packet to send"),
                )
                .arg(
                    Arg::new("icmp6S")
                        .short('S')
                        .required(false)
                        .value_name("ICMP <SIZE BYTES>")
                        .long("icmp6")
                        .action(ArgAction::Set)
                        .help("Icmp6 size packet to send"),
                ),
        )
        .get_matches();
    Ok(matches)
}
