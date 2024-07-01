// #![allow(unused)]

use std::env;
use std::fs;

const DEFAULT_DICT_PATH: &str = "data/dictionary.txt";

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