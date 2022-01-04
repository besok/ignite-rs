pub struct IgniteClientCfg{
    pub host:String,
    pub port:String,
    pub username: String,
    pub password: String,
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl IgniteClientCfg{
    pub fn default(major: u16,minor: u16) -> Self{
        IgniteClientCfg{
            host: "127.0.0.1".to_string(),
            port: "10800".to_string(),
            username: "".to_string(),
            password: "".to_string(),
            major,
            minor,
            patch: 0,
        }
    }
    pub fn address(&self) -> String{
        format!("{}:{}",self.host,self.port)
    }
}
