use pnet::packet::{
    icmp::IcmpPacket,
    ipv4::Ipv4Packet,
    ipv6::Ipv6Packet,
    tcp::TcpPacket,
    udp::UdpPacket,
    Packet,
};

pub trait NetworkData<'a> {
    fn get_source(&'a self) -> String;
    fn get_destinations(&'a self) -> String;
    fn get_payload(&'a self) -> &[u8];
    fn get_port(&'a self) -> String;
    fn get_protocol(&'a self) -> String;
}

impl<'a> NetworkData<'a> for Ipv4Packet<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_destinations(&'a self) -> String {
        self.get_destinations().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }

    fn get_port(&'a self) -> String {
        self.get_port().to_string()
    }

    fn get_protocol(&'a self) -> String {
        self.get_protocol().to_string()
    }
}

impl<'a> NetworkData<'a> for Ipv6Packet<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_destinations(&'a self) -> String {
        self.get_destinations().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }

    fn get_port(&'a self) -> String {
        self.get_port().to_string()
    }

    fn get_protocol(&'a self) -> String {
        self.get_protocol().to_string()
    }
}

impl<'a> NetworkData<'a> for TcpPacket<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_destinations(&'a self) -> String {
        self.get_destinations().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }

    fn get_port(&'a self) -> String {
        self.get_port().to_string()
    }

    fn get_protocol(&'a self) -> String {
        self.get_protocol().to_string()
    }
}

impl<'a> NetworkData<'a> for UdpPacket<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_destinations(&'a self) -> String {
        self.get_destinations().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }

    fn get_port(&'a self) -> String {
        self.get_port().to_string()
    }

    fn get_protocol(&'a self) -> String {
        self.get_protocol().to_string()
    }
}

impl<'a> NetworkData<'a> for IcmpPacket<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_destinations(&'a self) -> String {
        self.get_destinations().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }

    fn get_port(&'a self) -> String {
        self.get_port().to_string()
    }

    fn get_protocol(&'a self) -> String {
        self.get_protocol().to_string()
    }
}
