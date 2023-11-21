use bincode::Error as BincodeError;
use clap::Parser;
use log::info;
use serde::{Deserialize, Serialize};
use image::io::Reader as ImageReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    host: Option<String>,
    #[arg(short, long)]
    port: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File { name: String, data: Vec<u8> },
    Quit,
}

impl MessageType {
    pub fn to_bytes(&self) -> Result<Vec<u8>, BincodeError> {
        bincode::serialize(&self)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, BincodeError> {
        bincode::deserialize(data)
    }

    pub fn to_text(&self) -> String {
        match self {
            MessageType::Text(text) => text.clone(),
            _ => "".to_string(),
        }
    }

    pub fn file_from_path(path: &str) -> Self {
        let name = path.split('/').last().unwrap();
        let data = std::fs::read(path).unwrap();
        Self::File { name: name.to_string(), data }
    }

    pub fn image_from_path(path: &str) -> Self {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        Self::Image(img.into_bytes())
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

    pub fn from_args_or_default(args: Args) -> Self {
        let arg_config = Self::new(
            args.host.unwrap_or("127.0.0.1".to_string()),
            args.port.unwrap_or("8089".to_string()),
        );
        let default_config = Self::from_default();
        if arg_config.as_string() == default_config.as_string() {
            info!(
                "Server config is created from default: {}",
                default_config.as_string()
            );
            default_config
        } else {
            info!(
                "Server config is created from user input: {}",
                arg_config.as_string()
            );
            arg_config
        }
    }
}
