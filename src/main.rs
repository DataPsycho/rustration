use slug::slugify;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let example_txt = "cargo run --lowercase/-l or --no-space/-ns or --slugify/-s";
        panic!("Please provide a transformation flag to work with the text. Example: {}", example_txt);
    }

    println!("Enter a text:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let flag = &args[1];
    let flag = flag.as_str();

    match flag{
        "--lowercase" | "-l" => {
            println!("Lowercased: {}", input.to_lowercase());
        },
        "--uppercase" | "-u" => {
            println!("Uppercased: {}", input.to_uppercase());
        },
        "--no-spaces" | "-ns" => {
            println!("No spaced: {}", input.replace(" ", ""));
        },
        "--slugify" | "-s" => {
            println!("Slugified: {}", slugify(&input));
        }
        _ => {
            let flag_list = "--lowercase/-l, --uppercase/-u, --no-spaces/-ns, --slugify/-s";
            panic!("Transformation flag {} is not exceptable flag! Available flags are: {}", flag, flag_list);
        },
        
    }

}
