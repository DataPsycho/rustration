mod flag;
mod transform;
use std::env;

fn receive_input_from_user() -> String {
    eprintln!("Enter a text:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let flag = flag::parse_flag(args).unwrap();

    let input = receive_input_from_user();
    eprint!("Input: {}\n", input);

    let result = transform::transform(input, flag);
    eprintln!("Transformed result: {}", result);
}
