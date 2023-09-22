use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    let re = Regex::new(r"\b\d{4}\b").unwrap();


    let file = File::open("list_mmr.txt")?;
    let reader = io::BufReader::new(file);

    let mut four_digit_numbers = Vec::new();

    for line in reader.lines() {
        let line = line?;
        for capture in re.captures_iter(&line) {
            let number = &capture[0];
            four_digit_numbers.push(number.to_string());
        }
    }

    for number in &four_digit_numbers {
        println!("{}", number);
    }

    Ok(())
}
