use std::io::Write;
use rustration::MessageType;
use std::net::TcpStream;
use rustration::ServerConfig;


fn main() {
    let my_message = MessageType::Text("Hello, World!".to_string());
    let message_as_bytes = my_message.to_bytes().unwrap();
    println!("Message as bytes: {:?}", message_as_bytes);
    let config = ServerConfig::from_default();
    let mut stream = TcpStream::connect(config.as_string()).unwrap();
    let len = message_as_bytes.len() as u32;
    println!("Lenth of stream is {:?}", len);
    stream.write(&len.to_be_bytes()).unwrap();
    println!("Lenth of stream after being byte {:?}", &len.to_be_bytes());
    stream.write_all(&message_as_bytes).unwrap();
}
