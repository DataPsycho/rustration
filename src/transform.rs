use crate::flag::Flag;
use slug::slugify;

pub fn transform(input: String, flag: Flag) -> String {
    match flag {
        Flag::Lowercase => input.to_lowercase(),
        Flag::Uppercase => input.to_uppercase(),
        Flag::NoSpaces => input.replace(" ", ""),
        Flag::Slugify => slugify(&input),
    }
}