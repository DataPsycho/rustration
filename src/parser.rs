use std::error::Error;
use std::io::{self, BufRead};

pub fn receive_multiline_input_from_user() -> Result<String, Box<dyn Error>> {
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
