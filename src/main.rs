
use std::io::Write;
use rustration::MessageType;
use std::net::TcpStream;
use rustration::ServerConfig;


fn main() {
    let my_message = MessageType::Text("Hello, World!".to_string());
    let message_as_bytes = my_message.to_bytes().unwrap();
    let config = ServerConfig::from_default();
    let mut stream = TcpStream::connect(config.as_string()).unwrap();
    let len = message_as_bytes.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(&message_as_bytes).unwrap();
}
