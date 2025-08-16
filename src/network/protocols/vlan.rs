use pnet::{
    datalink::EtherType,
    packet::{
        Packet,
        ethernet::{EtherTypes, EthernetPacket},
        vlan::VlanPacket,
    },
};
use termion::color;

fn print_info(tpid: &EtherType, vlan_id: &u16, dei: &u8) {
    println!(
        "{}TPID: {:?} | VLAN ID: {:>3} | DEI: {}{}",
        color::Fg(color::Green),
        tpid,
        vlan_id,
        dei,
        color::Fg(color::Reset),
    );
}

pub fn vlan_handler(ether: &EthernetPacket) {
    match ether.get_ethertype() {
        EtherTypes::Vlan => {
            let packet = VlanPacket::new(ether.payload()).unwrap();

            let ether_type = packet.get_ethertype();
            let tpid = ether_type.0;
            let vlan_id = packet.get_vlan_identifier();
            let dei = packet.get_drop_eligible_indicator();

            print_info(&tpid, &vlan_id, &dei);
        }

        _ => {
            println!("Unhandled EtherType");
        }
    }
}
