#![allow(unused)]

use std::path::Path;
use std::process;
use std::fs;
use std::env;
use std::io::{self, Write};
use rand::Rng;

const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
const ALIVE_CELL: &str = "\u{2588}\u{2588}";
const DEAD_CELL: &str = "  ";

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path: &str;

    if args.len() < 2 {
        file_path = "data/dictionary.txt";
    } else {
        file_path = &args[1];
    }

    let file_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        let binding = env::current_dir().unwrap();
        let path = binding.display();
        eprintln!("Problem reading the file {file_path} from {path}: {err}");
        process::exit(1);
    });

    let lines: Vec<String> = file_content.lines().map(|line| line.to_string()).collect();

    let mut dictionary: Vec<String> = vec![];

    for line in lines {
        let line = line.trim().to_string();
        if line.len() == 0 {
            continue;
        }
        dictionary.push(line);
    }

    if dictionary.len() == 0 {
        eprintln!("The dictionary is empty");
        process::exit(1);
    }

    // println!("{} words in {dictionary:?}", dictionary.len());

    let word_idx = rand::thread_rng().gen_range(0..dictionary.len());

    let secret_word = dictionary[word_idx].clone();

    // println!("The secret word is: {secret_word}");

    let word_characters: Vec<char> = secret_word.chars().collect();

    // println!("The secret word is splitted as: {word_characters:?}");

    let mut state: Vec<char> = vec!['_'; word_characters.len()];
    let mut str_state: String = state.iter().collect();

    // println!("The state of the game is: {str_state:?}");

    let mut to_guess = word_characters.len();
    let mut lives = 6;

    loop {

        print!("{CLEAR_SCREEN}");
        
        let str_state: String = state.iter().collect();
        println!("You have {lives} lives left and state of the game is: {str_state}");
        
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap(); // flush the buffer to display the prompt

        let mut letter = String::new();

        io::stdin()
            .read_line(&mut letter)
            .expect("Failed to read line");
        
        // not WAI yet as it does not handle the case where the user types more than one letter
        let letter = letter.trim().to_string().to_lowercase().chars().next().unwrap_or_else(|| {
            eprintln!("Please type a letter.");
            process::exit(1);
        });

        // println!("You guessed: {letter}");

        if state.contains(&letter) {
            println!("\nYou already guessed the letter {letter}");
            continue;
        }
        
        if word_characters.contains(&letter) {
            // println!("Letter found somewhere in the word");
            for (i, _) in word_characters.iter().enumerate() {
                if word_characters[i] == letter {
                    state[i] = letter;
                    to_guess -= 1;
                }
            }
        } else {
            lives -= 1;
            if lives == 0 {
                println!("\nYou lost! The secret word was '{secret_word}'.");
                break;
            } else {
                println!("Letter not found in the word.");
            }   
        }
        
        if to_guess == 0 {
            println!("\nYou win! The secret word was '{secret_word}'.");
            break;
        }

    }

    println!("\n{ALIVE_CELL}  {ALIVE_CELL}");
    println!("  {ALIVE_CELL}  ");
    println!("{ALIVE_CELL}  {ALIVE_CELL}");
}