#[derive(Debug)]
pub enum Flag {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    CSV,
}

fn get_available_flags() -> String {
    let available_flags = vec![
        "--lowercase/-l",
        "--uppercase/-u",
        "--no-space/-ns",
        "--slugify/-s",
        "--csv",
    ]
    .iter()
    .map(|flag| flag.to_string())
    .collect::<Vec<String>>()
    .join(", ");
    available_flags
}

fn get_flag(flag: &str) -> Result<Flag, String> {
    match flag {
        "--lowercase" | "-l" => Ok(Flag::Lowercase),
        "--uppercase" | "-u" => Ok(Flag::Uppercase),
        "--no-spaces" | "-ns" => Ok(Flag::NoSpaces),
        "--slugify" | "-s" => Ok(Flag::Slugify),
        "--csv" => Ok(Flag::CSV),
        _ => {
            let available_flags = get_available_flags();
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
        let avaliable_flags = get_available_flags();
        let err_text = format!(
            "Please provide a transformation flag to work with the text. Example: {}",
            avaliable_flags
        );
        return Err(err_text);
    }
    get_flag(&args[1])
}
