use std::env;
use std::fs;
use std::io::{self, Write};
use rand::Rng;

const DEFAULT_DICT_PATH: &str = "data/dictionary.txt";
const CLEAR_CONSOLE: &str = "\x1B[2J\x1B[1;1H";

pub struct Config {
    pub dict_path: String,
    pub dict: Vec<String>,
    pub ignore_case: bool,
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, String> {

        let dict_path: String = if args.len() < 2 {
            DEFAULT_DICT_PATH.to_string()
        } else {
            args[1].clone()
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        let dict_content = fs::read_to_string(&dict_path).map_err(|err| {
            let binding = env::current_dir().unwrap();
            let path = binding.display();
            format!("unable to read the dictionary at {dict_path} from {path} -> {err}")
        })?;
    
        let lines: Vec<String> = dict_content.lines().map(|line| line.to_string()).collect();
    
        let mut dict: Vec<String> = vec![];
    
        for line in lines {
            let mut line = line.trim().to_string();
            if line.len() == 0 {
                continue;
            }
            if ignore_case {
                line = line.to_lowercase();
            }
            dict.push(line);
        }
    
        if dict.is_empty() {
            return Err(format!("the dictionary in {dict_path} is empty"));
        }

        Ok(Config { dict_path, dict, ignore_case })
    }

}

/// Initialize the game by sampling a word from the dictionary and splitting it into a vector of characters.
fn init_game(config: &Config) -> (String, Vec<char>) {
    let word_idx = rand::thread_rng().gen_range(0..config.dict.len());
    let secret_word = config.dict[word_idx].clone();
    let word_characters: Vec<char> = secret_word.chars().collect();
    (secret_word, word_characters)
}

/// Game logic
pub fn game(config: &Config) -> Result<(), String>{

    // sample a word according to the config
    let (secret_word, word_characters) = init_game(config);

    // initialize state of the game variables
    let mut state: Vec<char> = vec!['_'; word_characters.len()];
    let mut to_guess = word_characters.len(); // remaining number of characters to guess
    let mut lives = 6;
    let mut tried_letters: Vec<char> = vec![]; // unique set of letters that were already tried

    // main game loop
    loop {

        print!("{CLEAR_CONSOLE}");

        // display the state of the game
        let str_state: String = state.iter().collect();
        println!("You have {lives} lives left and state of the game is: {str_state}");
        let str_tried_letters: String = 
            tried_letters.iter().map(|c| c.to_string().to_uppercase()).collect::<Vec<String>>().join("");
        println!("\nLetters already tried: {str_tried_letters}");
        
        // prompt the user for a guess
        print!("\nPlease input your guess: ");
        io::stdout().flush().unwrap(); // flush the buffer to display the prompt

        // read the input from the user
        let mut str_input = String::new();
        io::stdin().read_line(&mut str_input).expect("Failed to read line");
        let input = str_input.trim().to_string().to_lowercase().chars().collect::<Vec<char>>();
        let letter: char;
        if input.len() == 1 {
            letter = char::from(input[0]);
        } else {
            continue;
        }
        
        // check if the letter was already tried
        if tried_letters.contains(&letter) {
            continue;
        } else {
            tried_letters.push(letter);
        }
        
        // update the state of the game
        if word_characters.contains(&letter) {
            for (i, _) in word_characters.iter().enumerate() {
                if word_characters[i] == letter {
                    state[i] = letter;
                    to_guess -= 1;
                }
            }
        } else {
            lives -= 1;
            // check for defeat
            if lives == 0 {
                println!("\nYou lost! The secret word was '{secret_word}'.");
                return Ok(());
            } else {
                println!("Letter not found in the word.");
            }   
        }
        
        // check for victory
        if to_guess == 0 {
            println!("\nYou win! The secret word was '{secret_word}'.");
            return Ok(());
        }

    }
}