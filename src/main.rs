#![allow(unused)]

use std::path::Path;
use std::process;
use std::fs;
use std::env;
use std::io::{self, Write};
use rand::Rng;

use hangman_rust::Config;

const CLEAR_SCREEN: &str = "\x1B[2J\x1B[1;1H";
const ALIVE_CELL: &str = "\u{2588}\u{2588}";
const DEAD_CELL: &str = "  ";

// println!("\n{ALIVE_CELL}  {ALIVE_CELL}");
// println!("  {ALIVE_CELL}  ");
// println!("{ALIVE_CELL}  {ALIVE_CELL}");

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("ERROR when building the config: {err}");
        process::exit(1);
    });

    let word_idx = rand::thread_rng().gen_range(0..config.dict.len());

    let secret_word = config.dict[word_idx].clone();

    let word_characters: Vec<char> = secret_word.chars().collect();

    let mut state: Vec<char> = vec!['_'; word_characters.len()];
    let mut str_state: String = state.iter().collect();

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

}

// TODO
// * display the letters that were already tried
// * don't take a live it it was already tried
// * ask for a new output if the user types more than one character