#[macro_use] extern crate prettytable;
mod flag;
mod parser;
mod transform;
use parser::receive_multiline_input_from_user;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = flag::parse_flag(args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let input = match receive_multiline_input_from_user() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("\nInput: \n{}", input);

    let result = match transform::transform(input, flag) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    println!("\nTransformed result: \n{}", result);
}
