use pnet::datalink::interfaces;

pub fn filter_interfaces(name: &str) -> Result<(), std::io::Error> {
    let inter = interfaces();
    let interfaces = inter.into_iter().find(|a| a.name == *name.to_string());

    match interfaces {
        Some(interface) => {
            println!();
            println!("Name: {}", interface.name);
            println!("Description: {}", interface.description);
            println!("Index: {}", interface.index);
            println!("MAC: {:?}", interface.mac);
            println!("IPs: {:?}", interface.ips);
            println!("Flags: {}", interface.flags);
            println!();
        }
        None => {
            println!("Interface '{}' not found.", name);
        }
    }
    Ok(())
}

pub fn index_interface() -> Result<(), std::io::Error> {
    let inter = interfaces();

    for interface in inter {
        println!();
        println!("Name: {}", interface.name);
        println!("Description: {}", interface.description);
        println!("Index: {}", interface.index);
        println!("MAC: {:?}", interface.mac);
        println!("IPs: {:?}", interface.ips);
        println!("Flags: {}", interface.flags);
        println!();
    }
    Ok(())
}
