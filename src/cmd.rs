use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn args() -> ArgMatches {
    let matches = Command::new("netdump")
        .author("Leandro <Kirov>")
        .version("v0.1.0")
        .about("Packet capturer, you can capture packets from three different protocols")
        .arg(
            Arg::new("Interfaces")
                .short('i')
                .long("interface")
                .help("Specifying the Interface")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("Protocol")
                .short('p')
                .long("protocol")
                 .help("Protocol filter"),
        )
        .get_matches();
    matches
}
