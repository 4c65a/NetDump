use std::process::Command;

use clap::{Arg, ArgAction, Command};
use interface::list_interface::index_interface;
use log::info;

mod interface;
mod protocol;

fn main() {
    let matches = cmd().get_matches_from(std::env::args());
    match matches.subcommand() {
        Some(("list",_)) => {
            let index = index_interface();
            for name in index {
                println!("- {}", name);
            }
        }
        _ => info!("Clap don't command"),
    };
}

fn cmd() -> Command {
}


