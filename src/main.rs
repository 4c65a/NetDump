use clap::{Arg, ArgAction, Command};

mod interface;
mod protocol;

fn main() {
    let matches = cmd().get_matches_mut();

    match matches.subcommand() {
        Some() => {
            
        },
        _ => {},
    }
}

fn cmd() -> Command{
    let matches = Command::new("netdump")
        .display_name("netdump")
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
            Arg::new("interface_list")
                .short('l')
                .long("list")
                .help("List interface on your system")
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
        );
    matches
}
