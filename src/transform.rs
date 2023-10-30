use crate::flag::Flag;
use csv;
use slug::slugify;

pub fn transform(input: String, flag: Flag) -> Result<String, Box<dyn std::error::Error + Send>> {
    match flag {
        Flag::Lowercase => Ok(input.to_lowercase()),
        Flag::Uppercase => Ok(input.to_uppercase()),
        Flag::NoSpaces => Ok(input.replace(" ", "")),
        Flag::Slugify => Ok(slugify(&input)),
        Flag::CSV => csv_as_str(input),
    }
}

fn csv_as_str(input: String) -> Result<String, Box<dyn std::error::Error + Send>> {
    let mut content: Vec<String> = Vec::new();
    let mut reader = csv::Reader::from_reader(input.as_bytes());
    let headers = reader.headers().unwrap().clone();
    let headers = headers.into_iter().collect::<Vec<&str>>();
    let headers = headers.join(",");
    content.push(headers);
    for result in reader.records() {
        let record = result.unwrap();
        let record = record.into_iter().collect::<Vec<&str>>();
        let record = record.join(",");
        content.push(record);
    }
    Ok(content.join("\n"))
}
