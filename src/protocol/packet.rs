use pnet::packet::{
    icmp::IcmpPacket, ipv4::Ipv4Packet, ipv6::Ipv6Packet, tcp::TcpPacket, udp::UdpPacket, Packet,
};

/// Get Headers Ipv4
pub trait HeaderDataIpv4<'a> {
    /// Puerto de origen del paquete
    fn get_source(&'a self) -> String;
    /// Puerto de destino del paquete
    fn get_destinations(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> &[u8];
    /// Contiene un conjunto de banderas de control que se utilizan Ipv4 y Tcp para indicar el estado de la conexión TCP
    fn get_flags(&'a self) -> String;
    /// Indica la version del protocolo Ipv4 y Ipv6
    fn get_version(&'a self) -> String;
    /// Time to Live (TTL) Ipv4
    fn get_ttl(&'a self) -> String;
}
/// Get Headers Ipv6
pub trait HeaderDataIpv6<'a> {
    /// Puerto de origen del paquete
    fn get_source(&'a self) -> String;
    /// Puerto de destino del paquete
    fn get_destinations(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> &[u8];
    /// Indica la version del protocolo Ipv4 y Ipv6
    fn get_version(&'a self) -> String;
}
/// Get Headers TCP
pub trait HeaderDataTcp<'a> {
    /// Puerto de origen del paquete
    fn get_source(&'a self) -> String;
    /// Puerto de destino del paquete
    fn get_destinations(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> &[u8];
    /// Contiene un conjunto de banderas de control que se utilizan Ipv4 y Tcp para indicar el estado de la conexión TCP
    fn get_flags(&'a self) -> String;
}
/// Get Headers UDP
pub trait HeaderDataUdp<'a> {
    /// Puerto de origen del paquete
    fn get_source(&'a self) -> String;
    /// Puerto de destino del paquete
    fn get_destinations(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> &[u8];
    /// Tamaño total del paquete, en bytes Udp
    fn get_length(&'a self) -> String;
}
/// Get Headers ICMP
pub trait HeaderDataIcmp<'a> {
    /// Puerto de origen del paquete
    fn get_source(&'a self) -> String;
    /// Puerto de destino del paquete
    fn get_destinations(&'a self) -> String;
    /// Indica el tipo de mensaje ICMP que se está enviando.
    fn get_icmp_types(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> &[u8];
}

/// Implementacion de Ipv4 para obetner los encabezados
impl<'a> HeaderDataIpv4<'a> for Ipv4Packet<'a> {

    fn get_source(&'a self) -> String {
        let source = Ipv4Packet::get_source(&self);
        source.to_string()
    }

    fn get_destinations(&'a self) -> String {
        let destination = Ipv4Packet::get_destination(&self);
        destination.to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        let payload = Ipv4Packet::payload(&self);
        payload
    }
    fn get_version(&'a self) -> String {
        let version = Ipv4Packet::get_version(&self);
        version.to_string()
    }
    fn get_flags(&'a self) -> String {
        let flags = Ipv4Packet::get_flags(&self);
        flags.to_string()
    }
    fn get_ttl(&'a self) -> String {
        let ttl = Ipv4Packet::get_ttl(&self);
        ttl.to_string()
    }
}


/// Implementacion de Ipv6 para obetner los encabezados
impl<'a> HeaderDataIpv6<'a> for Ipv6Packet<'a> {

    fn get_source(&'a self) -> String {
        let source = Ipv6Packet::get_source(&self);
        source.to_string()
    }

    fn get_destinations(&'a self) -> String {
        let destination = Ipv6Packet::get_destination(&self);
        destination.to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        let payload = Ipv6Packet::payload(&self);
        payload
    }

    fn get_version(&'a self) -> String {
        let version = Ipv6Packet::get_version(&self);
        version.to_string()
    }
}

/// Implementacion de Tcp para obtener los encabezados
impl<'a> HeaderDataTcp<'a> for TcpPacket<'a> {
    fn get_source(&'a self) -> String {
        let source = TcpPacket::get_source(&self);
        source.to_string()
    }

    fn get_destinations(&'a self) -> String {
        let destination = TcpPacket::get_destination(&self);
        destination.to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        let payload = TcpPacket::payload(&self);
        payload
    }
    fn get_flags(&'a self) -> String {
        let flags = TcpPacket::get_flags(&self);
        flags.to_string()
    }
}

/// Implementacion de Udp para obtener los encabezados
impl<'a> HeaderDataUdp<'a> for UdpPacket<'a> {
    fn get_source(&'a self) -> String {
        let source = UdpPacket::get_source(&self);
        source.to_string()
    }

    fn get_destinations(&'a self) -> String {
        let destination = UdpPacket::get_destination(&self);
        destination.to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        let payload = UdpPacket::payload(&self);
        payload
    }

    fn get_length(&'a self) -> String {
        let length = UdpPacket::get_length(&self);
        length.to_string()
    }
}

/// Implementacion de Icmp para obtener los encabezados
impl<'a> HeaderDataIcmp<'a> for IcmpPacket<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }
    fn get_destinations(&'a self) -> String {
        self.get_source().to_string()
    }

    fn get_icmp_types(&'a self) -> String {
        self.get_icmp_types().to_string()
    }

    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }
}
