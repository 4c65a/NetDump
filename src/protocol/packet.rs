use pnet::packet::{ipv4::Ipv4Packet, ipv6::Ipv6Packet, Packet};

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

/// Implementacion de Ipv4 para obetner los encabezados
impl<'a> HeaderDataIpv4<'a> for Ipv4Packet<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }
    fn get_destinations(&'a self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }
    fn get_version(&'a self) -> String {
        self.get_version().to_string()
    }
    fn get_flags(&'a self) -> String {
        self.get_flags().to_string()
    }
    fn get_ttl(&'a self) -> String {
        self.get_ttl().to_string()
    }
}

/// Implementacion de Ipv6 para obetner los encabezados
impl<'a> HeaderDataIpv6<'a> for Ipv6Packet<'a> {
    fn get_source(&'a self) -> String {
        self.get_source().to_string()
    }
    fn get_destinations(&'a self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&'a self) -> &[u8] {
        self.payload()
    }
    fn get_version(&'a self) -> String {
        self.get_version().to_string()
    }
}
