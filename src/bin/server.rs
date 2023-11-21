use clap::Parser;
use env_logger;
use log::{info, error};
use rustration::Args;
use rustration::MessageType;
use rustration::ServerConfig;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use uuid::Uuid;



fn handle_client_connection(mut stream: TcpStream) -> MessageType {
    // Create a process ID
    let id = Uuid::new_v4();
    // Read the length of the message
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;
    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).expect("InternalError: Failed to read message");
    let my_message = MessageType::from_bytes(&buffer).unwrap();
    match &my_message {
        MessageType::Quit => info!("{} Received quit message", id),
        MessageType::File { name, ..} => info!("{} Received file: {}", id, name),
        MessageType::Image(_) => { info!("{} Received image", id)},
        MessageType::Text(_) => { info!("{} Received text", id)},
    }
    info!("{} Message length: {}", id, len);
    my_message
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let config = ServerConfig::from_args_or_default(args);
    let listner = TcpListener::bind(config.as_string()).unwrap();
    info!("Server listening on {}", config.as_string());
    for stream in listner.incoming() {
        match stream {
            Ok(stream) => {
                let result = std::thread::spawn(|| handle_client_connection(stream));
                let msg = result.join().unwrap();
                if msg == MessageType::Quit {
                    break;
                }
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}
