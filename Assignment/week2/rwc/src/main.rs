use std::env;
use std::process;
use std::fs::File;
use std::io::{read_to_string, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)

    let fptr = File::open(filename).expect("File open err");
    let reader = BufReader::new(fptr);

    let text = read_to_string(reader).expect("File read err");

    let _lines: Vec<String> = text.split('\n').map(str::to_string).collect();
    let _words: Vec<String> = text.trim().split_whitespace().map(str::to_string).collect();
    let _chars = text.chars();

    println!("lines: {:?}", _lines);
    println!("words: {:?}", _words);
    println!("chars: {:?}", _chars);
}
