use clap::{Arg, Command};

pub fn cmd() {
    let matches = Command::new("netdump")
        .author("Leandro <Kirov>")
        .version("v0.1.0")
        .about("Packet capturer, you can capture packets from three different protocols")
        .arg(Arg::new().short().long().index().help().required(true));
}
