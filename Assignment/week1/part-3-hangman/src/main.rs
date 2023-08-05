// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn print_vec(v: &Vec<char>) {
    for c in v {
        print!("{}", c);
    }
    println!("");
}

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_input() -> char{
    let mut input = String::new();
    stdin().read_line(& mut input).expect("Input error");

    match input.trim().parse::<char>() {
        Ok(v) => v,
        Err(_) => { println!("Input type error!"); std::process::exit(-1); }
    }
}

fn check_collect(input: char, guessed_word_char: &mut Vec<char>, answer: &Vec<char>) -> i32{
    assert_eq!(guessed_word_char.len(), answer.len());
    
    let mut cnt: i32 = 0;

    for i in 0..answer.len() {
        if input == answer[i] {
            if guessed_word_char[i] != '-' {
                return -1;
            }
            else {
                guessed_word_char[i] = input;
                cnt += 1;
            }
        }
    }

    cnt
}

fn check_clear(guessed_word_char: &Vec<char>) -> bool{
    for c in guessed_word_char.iter() {
        if *c == '-' { return false; }
    }

    return true;
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut guessed_word_chars: Vec<char> = vec!['-'; secret_word_chars.len()];
    let mut selected_chars: Vec<char> = Vec::new();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    let mut guess_count = NUM_INCORRECT_GUESSES;

    println!("Welcome to CS110L Hangman!");

    // Your code here! :)
    while guess_count > 0 {
        print!("The word so far is ");
        print_vec(&guessed_word_chars);

        println!("You have guessed the following letters:");
        let _ = stdout().flush();

        let input_char = get_input();
        selected_chars.push(input_char.clone());
        match check_collect(input_char, &mut guessed_word_chars, &secret_word_chars) {
            -1 => { println!("Already answered character."); guess_count -= 1;}
            0 => { println!("Wrong answer!"); guess_count -= 1;},
            _ => { println!("Correct!") }
        };

        print!("Selected characters : ");
        print_vec(&selected_chars);

        if check_clear(&guessed_word_chars) {
            print!("Game Clear! the answer is : ");
            print_vec(&guessed_word_chars);
            break;
        }
        else {
            println!("You have {} guesses left", guess_count);
        }

        println!();
    }
}
