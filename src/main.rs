use std::process;
use std::env;

use hangman_rust::{Config, game};

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("ERROR when building the config: {err}");
        process::exit(1);
    });

    let _ = game(&config).unwrap_or_else(|err| {
        eprintln!("ERROR when playing the game: {err}");
        process::exit(1);
    });

}