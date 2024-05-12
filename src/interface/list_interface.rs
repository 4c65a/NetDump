use pnet::datalink::interfaces;

pub fn index_interface() -> Vec<String> {
    let interfaces = interfaces();
    // Filter and collect interface names
    let filtered_interfaces = interfaces
        .iter()
        .filter(|e| e.is_up() && !e.ips.is_empty())
        .map(|e| e.name.to_string())
        .collect();
    filtered_interfaces
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_index_interface() {
        //let fn_interface = index_interface();
        let interfaces = interfaces();
        let filter = interfaces.iter().filter(|e| e.is_up() && !e.ips.is_empty());
        for inter in filter {
            println!("- {}", inter.name);
        }
    }
}
