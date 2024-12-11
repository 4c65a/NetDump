pub struct Handle {
    ttl: u8,
    payload_icmp: usize,
    icmp_size: usize,
    ipv4_size: usize,
    ipv6_size: usize,
    // total_length_size: Option<usize>,
    // total_length_size_ipv6: Option<usize>,
}

impl Default for Handle {
    fn default() -> Self {
        Handle {
            ttl: 65,
            payload_icmp: 56,
            icmp_size: 8,
            ipv4_size: 20,
            ipv6_size: 40,
            // total_length_size: None,
            // total_length_size_ipv6: None,
        }
    }
}

impl Handle {
    // pub fn new()-> Self {
    // Handle { ttl: , payload_icmp:  , icmp_size: , ipv4_size: , ipv6_size:  }
    // }

    pub fn set_ttl(mut self, value: u8) {
        self.ttl = value;
    }

    pub fn set_payload_icmp(mut self, value: usize) {
        self.payload_icmp = value;
    }

    pub fn set_icmp_size(mut self, value: usize) {
        self.icmp_size = value;
    }

    pub fn set_ipv4_size(mut self, value: usize) {
        self.ipv4_size = value;
    }

    pub fn set_ipv6_size(mut self, value: usize) {
        self.ipv6_size = value;
    }

    pub fn get_ttl(&self) -> u8 {
        return self.ttl;
    }

    pub fn get_payload_icmp(&self) -> usize {
        return self.payload_icmp;
    }

    pub fn get_icmp_size(&self) -> usize {
        return self.icmp_size;
    }

    pub fn get_ipv4_size(&self) -> usize {
        return self.ipv4_size;
    }

    pub fn get_ipv6_size(&self) -> usize {
        return self.ipv6_size;
    }
}
#[allow(dead_code)]
pub fn configurations_ipv4(
    ttl_value: u8,
    payload_icmp_value: usize,
    icmp_size_value: usize,
    ipv4_size_value: usize,
) {
    let configuration = Handle::default();
}
