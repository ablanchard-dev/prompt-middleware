use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: Ipv4Addr,
    pub port: u16,
    pub allowed_origins: Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: Ipv4Addr::LOCALHOST,
            port: 43187,
            allowed_origins: vec![
                "chrome-extension://development".to_owned(),
                "edge-extension://development".to_owned(),
            ],
        }
    }
}
