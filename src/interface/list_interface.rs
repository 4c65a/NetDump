
use pnet::datalink::interfaces;

pub fn index_interface() {
    let interfaces = interfaces();
    let filtered_interfaces = interfaces.iter().filter(|e| e.is_up() && !e.ips.is_empty());

    println!("List of network interfaces on your system:");
    for inter in filtered_interfaces{
        println!("- {}",inter.name);
    }
}

#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_index_interface(){
        let fn_interface = index_interface();
        let interfaces = interfaces();
        let filter = interfaces.iter().filter(|e| e.is_up() && !e.ips.is_empty());
        let println_for = for inter in filter {
            println!("- {}", inter.name);
        };
        assert!();
    }
}
