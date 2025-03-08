use capture::{
    cap_packet::cap,
    interfaces::{self},
};
use cli::root::cmd;
use route::{ping::*, resolve_host, tracerouter::trace};

mod capture;
mod cli;
mod protocol;
mod route;

#[tokio::main]
async fn main() {
    let matches = cmd().unwrap();

    match matches.subcommand() {
        // Command cap with subcommand --interface
        Some(("cap", cap_matches)) => match cap_matches.get_one::<String>("interface") {
            Some(interface) => {
                cap(interface);
            }
            _ => {
                println!("Error capturing packets.");
            }
        },

        // Command interface with two subcommand --list | --filter
        Some(("interface", inter_matches)) => {
            if let Some(filter) = inter_matches.get_one::<String>("filter") {
                interfaces::filter_interfaces(filter).unwrap();
            } else if inter_matches.contains_id("list") {
                interfaces::index_interface().unwrap();
            } else {
                println!("No commands found.");
            }
        }

        // Command ping with subcommand --destination
        Some(("ping", ping_matches)) => {
            let destination = if ping_matches.contains_id("ipv6") {
                ping_matches
                    .get_one::<String>("ipv6")
                    .unwrap_or(&"::1".to_string())
                    .clone()
            } else {
                ping_matches
                    .get_one::<String>("destination")
                    .unwrap_or(&"127.0.0.1".to_string())
                    .clone()
            };

            let ttl = ping_matches
                .get_one::<String>("ttl")
                .unwrap_or(&"64".to_string())
                .parse::<u8>()
                .unwrap_or(64);

            let min_send = ping_matches
                .get_one::<String>("min_send")
                .unwrap_or(&"1".to_string())
                .parse::<u64>()
                .unwrap_or(1);

            let count = ping_matches
                .get_one::<String>("count")
                .unwrap_or(&"10".to_string())
                .parse::<i32>()
                .unwrap();

            let ping_task = tokio::spawn(async move {
                if let Err(e) = ping(destination.as_str(), ttl, min_send, Some(count)).await {
                    eprintln!("Error executing ping: {:?}", e);
                }

                tokio::task::yield_now().await;
            });

            let _ = tokio::join!(ping_task);
        }

        Some(("tracerouter", trace_matches)) => {
            let ip = trace_matches
                .get_one::<String>("trace")
                .unwrap_or(&"127.0.0.1".to_string())
                .clone();

            let trace_route = tokio::spawn(async move {
                trace(&ip).await;
            });

            if let Err(e) = trace_route.await {
                eprintln!("Tracing task failed: {:?}", e);
            }
        }
        Some(("resolve", resolve_matches)) => {
            let host = resolve_matches.get_one::<String>("host").unwrap().clone();
            let resolve = resolve_host::resolve_host(&host).await;

            if let Ok(ip) = resolve {
                println!("HOST: {:?} => IP: {:?}", host, ip);
            } else if let Err(e) = resolve {
                eprintln!("Resolve host failed: {:?}", e);
            }
        }
        _ => {
            println!("No commands found");
        }
    }
}
