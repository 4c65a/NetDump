use pnet::datalink::interfaces;

pub fn interface(int_name: &str){
    let interface = interfaces();
    interface.into_iter().filter(|inter| inter.name == *int_name ).next().expect("Failed to get interface");

    
}
