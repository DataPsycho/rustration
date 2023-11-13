use rustration::ServerConfig;
use rustration::MessageType;
use std::io::Read;
use std::net::TcpListener;


fn main(){
    let config = ServerConfig::from_default();
    let listner = TcpListener::bind(config.as_string()).unwrap();
    println!("Server listening on {}", config.as_string());
    for connection in listner.incoming(){
        let mut connection = connection.unwrap();
        let mut len_bytes = [0u8; 4];
        connection.read_exact(&mut len_bytes).unwrap();
        let len = u32::from_be_bytes(len_bytes) as usize;
        let mut buffer = vec![0u8; len];
        connection.read_exact(&mut buffer).unwrap();
        let my_message = MessageType::from_bytes(&buffer).unwrap();
        println!("{:?}", my_message);
    }
}