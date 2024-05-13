use pnet::datalink::interfaces;

pub fn filter_interfaces(name: &str) -> Result<(), std::io::Error> {
    let inter = interfaces()
        .into_iter()
        .filter(|a| a.name == name.to_string());
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

pub fn index_interface() -> Result<(), std::io::Error> {
    let inter = interfaces();
    //let interface = vec![inter];
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
