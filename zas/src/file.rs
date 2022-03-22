use colored::*;

use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn read_file(filename: String) -> Vec<String> {
    let file = match File::open(filename.clone()) {
        Ok(fl) => fl,
        Err(why) => {
            eprintln!("{}: couldn't open {} cause {}", "ERROR".bright_red(), filename.italic().bold(), why);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        data.push(line);
    }
    data
}
