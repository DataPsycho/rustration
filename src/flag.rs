#[derive(Debug)]
pub enum Flag {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
}

pub fn get_flag(flag: &str) -> Result<Flag, String> {
    match flag {
        "--lowercase" | "-l" => Ok(Flag::Lowercase),
        "--uppercase" | "-u" => Ok(Flag::Uppercase),
        "--no-spaces" | "-ns" => Ok(Flag::NoSpaces),
        "--slugify" | "-s" => Ok(Flag::Slugify),
        _ => {
            let available_flags =
                "cargo run --lowercase/-l or --uppercase/-u or --no-space/-ns or --slugify/-s";
            let err_text = format!(
                "{} is not a valid flag. Available flags are: {}",
                flag, available_flags
            );
            Err(err_text)
        }
    }
}

pub fn parse_flag(args: Vec<String>) -> Result<Flag, String> {
    if args.len() < 2 {
        let example_txt =
            "cargo run --lowercase/-l or --uppercase/-u or --no-space/-ns or --slugify/-s";
        let err_text = format!(
            "Please provide a transformation flag to work with the text. Example: {}",
            example_txt
        );
        return Err(err_text);
    }
    get_flag(&args[1])
}
