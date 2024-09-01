use cli::root::cmd;

mod capture;
mod cli;
mod protocol;
mod route;
mod speed;

#[tokio::main]
async fn main() {
    let matches = cmd().unwrap();
    match matches.subcommand() {
        // Command cap with subcommand --interface
        Some() => {}
        // Command interface with two subcommand --list | --filter
        Some() => {}
        // Command ping with subcommand --destination
        Some() => {}
        _ => {
            println!("NONE COMMANDS");
        }
    }
}
