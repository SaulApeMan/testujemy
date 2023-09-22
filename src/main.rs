use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> io::Result<()> {
    // Define a regular expression to match 4-digit numbers
    let re = Regex::new(r"\b\d{4}\b").unwrap();

    // Open the text file
    let file = File::open("list_mmr.txt")?;
    let reader = io::BufReader::new(file);

    // Create a vector to store the matching numbers
    let mut four_digit_numbers = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?;
        // Find all 4-digit numbers in the line and add them to the vector
        for capture in re.captures_iter(&line) {
            let number = &capture[0];
            four_digit_numbers.push(number.to_string());
        }
    }

    // Print the extracted numbers
    for number in &four_digit_numbers {
        println!("{}", number);
    }

    Ok(())
}
