pub struct IgniteClientCfg {
    pub nodes: Vec<IgniteNodeAddress>,
    pub username: String,
    pub password: String,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl IgniteClientCfg {
    pub fn setAddresses(&mut self, nodes:Vec<IgniteNodeAddress>) {
        self.nodes = nodes;
    }
    
    pub fn default(major: u16, minor: u16) -> Self {
        IgniteClientCfg {
            nodes:vec![IgniteNodeAddress::default()],
            username: "".to_string(),
            password: "".to_string(),
            major,
            minor,
            patch: 0,
        }
    }
}

pub struct IgniteNodeAddress {
    pub host: String,
    pub port: String,
}
impl IgniteNodeAddress {
    pub fn new(host: String, port: String) -> Self {
        IgniteNodeAddress { host, port }
    }
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for IgniteNodeAddress {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: "10800".to_string(),
        }
    }
}
