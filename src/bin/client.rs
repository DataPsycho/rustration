use clap::Parser;
use env_logger;
use log::{info, error};
use rustration::Args;
use rustration::MessageType;
use rustration::ServerConfig;
use std::io::Write;
use std::net::TcpStream;


fn send_message(msg: MessageType, stream: &mut TcpStream) {
    let message_as_bytes = msg.to_bytes().unwrap();
    let len = message_as_bytes.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(&message_as_bytes).unwrap();
    match msg {
        MessageType::Image(_) => info!("Sent an image"),
        MessageType::Text(_) => info!("Sent a text"),
        MessageType::File { name, .. } => {
            info!("Sent a file with name: {}", name);
        }
        MessageType::Quit => info!("Sent a quit message to shut down the server"),
    }
    info!("Message length was : {}", len);
}


fn main() {
    env_logger::init();
    let args = Args::parse();
    let config = ServerConfig::from_args_or_default(args);
    let messages = [
        MessageType::image_from_path("data/images/test_1.png"),
        MessageType::Text("Hello, World!".to_string()),
        MessageType::file_from_path("data/files/test_1.txt"),
        MessageType::Quit
        ];
    for msg in messages {
        match TcpStream::connect(config.as_string()) {
            Ok(mut stream) => {
                send_message(msg, &mut stream);
            }
            Err(err) => {
                error!("Failed to connect at {}: {}", config.as_string(), err);
            }
            
        }
    }
}
