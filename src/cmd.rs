use clap::{Arg, ArgAction, Command};

pub fn args() {
    Command::new("netdump")
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
                .help("Protocol filter")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("Save")
                .short('s')
                .long("save")
                .help("Save")
                .required(false)
                .action(ArgAction::Set),
        )
        .get_matches();
}
