use std::fs::File;
use std::io::{BufReader, read_to_string};

#[allow(unused)]
pub fn parse_textfile(filename: String) -> Vec<u32> {
    let fptr = File::open(filename).expect("File open err.");
    let reader = BufReader::new(fptr);
    let text = read_to_string(reader).expect("File read err.");

    let ret: Vec<u32> = text.trim().split_whitespace().map(|c| match c.parse::<u32>() {
        Ok(v) => v,
        Err(_) => 0
    }).collect();

    ret
}