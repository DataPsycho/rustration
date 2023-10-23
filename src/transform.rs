use crate::flag::Flag;
use csv;
use slug::slugify;
use std::error::Error;
use prettytable::Table;

pub fn transform(input: String, flag: Flag) -> Result<String, Box<dyn Error>> {
    match flag {
        Flag::Lowercase => Ok(input.to_lowercase()),
        Flag::Uppercase => Ok(input.to_uppercase()),
        Flag::NoSpaces => Ok(input.replace(" ", "")),
        Flag::Slugify => Ok(slugify(&input)),
        Flag::CSV => show_csv_from_input(input),
    }
}

fn show_csv_from_input(input: String) -> Result<String, Box<dyn Error>> {
    let mut reader = csv::Reader::from_reader(input.as_bytes());

    let headers = reader.headers()?;
    println!("{:?}", headers);
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    // let mut writer = csv::Writer::from_writer(vec![]);
    // writer.write_record(&["input", "output"]).unwrap();
    // for result in reader.deserialize() {
    //     let record = result.unwrap();
    //     writer.write_record(&[record.input, record.output]).unwrap();
    // }
    // String::from_utf8(writer.into_inner().unwrap()).unwrap()
    Ok("Temp".to_string())
}
