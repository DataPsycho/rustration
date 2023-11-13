use std::{
    // io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use std::io::{Read, Write};

mod config;
use config::ServerConfig;

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let http_request = buf_reader
//     .lines()
//     .map(|result| result.unwrap())
//     .take_while(|line| !line.is_empty())
//     .collect::<Vec<_>>();
//     println!("Request: {:#?}", http_request);
// }


fn main() {
    let config = ServerConfig::from_default();
    let mut stream = TcpStream::connect(config.as_string()).unwrap();
    stream.write_all(b"Hello World!").unwrap();

    let mut buffer = [0; 128];

    stream.read(&mut buffer).unwrap();
    println!("Response: {}", String::from_utf8_lossy(&buffer));
    // for stream in listner.incoming() {
    //     let stream = stream.unwrap();
        // handle_connection(stream);
    // }
}
