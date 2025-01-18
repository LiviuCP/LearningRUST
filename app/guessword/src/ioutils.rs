use homedir::my_home;
use std::{collections::HashSet, fs};

static DIR_NAME: &str = "Documents";
static INPUT_FILE_NAME: &str = "input.txt";

const MIN_WORD_SIZE: usize = 10;

pub fn build_input_file_path() -> Result<String, &'static str> {
    let mut result = Err("Unable to retrieve the input file path");

    if let Ok(Some(mut path)) = my_home() {
        path.push(DIR_NAME);
        path.push(INPUT_FILE_NAME);

        if let Some(path) = path.to_str() {
            result = Ok(path.to_string());
        }
    }

    result
}

pub fn read_input(file_path: &str) -> Result<Vec<String>, &'static str> {
    let mut result = Err("File could not be opened");

    if let Ok(input_str) = fs::read_to_string(file_path) {
        // use HashSet to remove duplicate values, the collect the (valid) input into a vector for enabling randomization
        let input: HashSet<String> = input_str
            .split('\n')
            .filter(|word| {
                // although len() returns number of bytes (not chars) if we constrain the chars to be ASCII then it should represent the number of chars (otherwise if not ASCII the filter will fail anyway)
                word.chars().all(|ch| ch.is_ascii_alphabetic()) && word.len() >= MIN_WORD_SIZE
            })
            .map(|val| {
                val.chars()
                    .map(|ch| ch.to_ascii_lowercase())
                    .collect::<String>()
            })
            .collect();

        result = if !input.is_empty() {
            Ok(input.iter().map(|val| val.to_string()).collect())
        } else {
            Err("File is empty or contains no valid data")
        };
    }

    result
}
