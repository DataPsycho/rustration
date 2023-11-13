pub struct ServerConfig {
    pub addr: String,
    pub port: String,
}

impl ServerConfig {
    pub fn new(addr: String, port: String) -> Self {
        Self { addr, port }
    }

    pub fn from_default() -> Self {
        Self::new("127.0.0.1".to_string(), "8089".to_string())
    }

    pub fn as_string(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}
