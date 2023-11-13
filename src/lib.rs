use serde::{Deserialize, Serialize};
use bincode::Error as BincodeError;


#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType{
    Text(String),
    Image(Vec<u8>),
    File{name: String, data: Vec<u8>},
}

impl MessageType {
    pub fn to_bytes(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }
    
    pub fn from_bytes(data: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(data)
    }
}


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