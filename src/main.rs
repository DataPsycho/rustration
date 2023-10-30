mod flag;
mod multiline_parser;
mod transform;
use multiline_parser::receive_multiline_input_from_user;
use std::env;
use std::thread;
use std::sync::mpsc::channel;

// This is the entry point of the program
// It takes a flag from the command line for example: --lowercase or -l and then transform the input using the flag. It prints the result
// # Example:
// cargo run -- --lowercase
fn main() {
    let (tx, rx) = channel();
    let args: Vec<String> = env::args().collect();
    let flag = flag::parse_flag(args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    println!("Provide a string to transform. \nYou can provide multiline input and finally press enter to finish. \nStart Here: \n");

    let input = match receive_multiline_input_from_user() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("\nInput: \n{}", input);

    thread::spawn(move || {
        let result = match transform::transform(input, flag) {
            Ok(result) => result,
            Err(e) => {
                e.to_string()
            }
        };
        tx.send(result).unwrap();
    });


    let recever = rx.recv().unwrap();
    println!("\nOutput: \n{}", recever);
}
