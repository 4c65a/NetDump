use pnet::datalink::interfaces;

pub fn filter_interfaces(name: &str) -> Result<(), std::io::Error> {
    let inter = interfaces();
    let names = inter.into_iter().find(|a| a.name == *name.to_string());

    match names {
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

    for interfaces in inter {
        println!();
        println!("Name: {}", interfaces.name);
        println!("Description: {}", interfaces.description);
        println!("Index: {}", interfaces.index);
        println!("MAC: {:?}", interfaces.mac);
        println!("IPs: {:?}", interfaces.ips);
        println!("Flags: {}", interfaces.flags);
        println!();
    }
    Ok(())
}
