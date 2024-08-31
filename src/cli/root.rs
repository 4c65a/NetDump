use clap::{self, Arg, ArgAction, ArgMatches, Command, Error};

pub fn root() -> Result<ArgMatches, Error> {
    let matches = Command::new("netdump")
        .author("Lean")
        .about("NetDump is a command tool in the terminal, it has multiple functionalities.")
        .version("0.0.1")
        .propagate_version(true)
        .arg_required_else_help(true)
        // cap --interface enp2s0|any --count 10 --cap-send --cap-receive --protocol --port --address 192.168.0.0.1 --save cap.pcap
        .subcommand(
            Command::new("cap").about("Capture packets from your network.").arg(
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
                        .help("Get the list of interfaces of your system")
                        .action(ArgAction::SetFalse),
                )
                .arg(
                    Arg::new("filter")
                        .short('t')
                        .long("filter")
                        .help("Get the list of interfaces on your system by filtering by name.")
                        .action(ArgAction::Set),
                ),
        )
        // ping --address --min-send --count(packet) --ip6 --save
        .subcommand(
            Command::new("ping").about("Ping is a network administration utility used to test the reachability of a host on an IP network. 
                It sends an Internet Control Message Protocol (ICMP) echo request packet to the specified host and waits for an ICMP echo reply.").arg(
                Arg::new("address")
                    .short('a')
                    .long("address")
                    .help("Address server."),
            ), //.arg(Arg::new("min-send").short('m').long("min-send").help(""))
               //.arg(Arg::new("ip6").short('6').long("ip6").help("")),
        )
        .get_matches();
    Ok(matches)
}
