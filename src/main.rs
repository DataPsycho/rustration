mod flag;
mod transform;
use std::env;
use std::error::Error;
use std::io::{self, BufRead};

// fn receive_input_from_user() -> String {
//     eprintln!("Enter a text:");
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     input.trim().to_string()
// }

fn receive_multiline_input_from_user() -> Result<String, Box<dyn Error>> {
    let mut lines = io::stdin().lock().lines();
    let mut input = String::new();
    while let Some(line) = lines.next() {
        match line {
            Ok(l) => {
                if l.len() == 0 {
                    break;
                }
                if input.len() > 0 {
                    input.push_str("\n");
                }
                input.push_str(&l);
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    let input = input.trim().to_string();
    Ok(input)
}

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
    let result = transform::transform(input, flag);
    println!("\nTransformed result: \n{}", result);
}
