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
    /// Indica el tipo de mensaje ICMP que se está enviando.
    fn get_icmp_type(&'a self) -> String;
    /// Contiene los datos de la aplicación que se están enviando, la carga util del paquete
    fn get_payload(&'a self) -> String;
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
    fn get_version(&'a self) -> String {
        self.get_version().to_string()
    }
    fn get_ttl(&'a self) -> String {
        self.get_ttl().to_string()
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
}
