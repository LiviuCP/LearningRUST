mod guessing;

use homedir::my_home;
use learn_rust_lib::utilities;
use rand::Rng;
use std::fs;

/*
TODO:
- create functionality for checking their validity when loading them into application (accept only the valid ones within array)
  - this includes allowing only specific characters (e.g. alphanumeric)
- create functionality for adding new words without manually modifying the file
- create functionality that removes already guessed words from array (vector) so they are not provided multiple times for guessing
- create functionality that prevents duplicate words within array
*/

const MIN_WORD_SIZE: usize = 10;

fn build_input_file_path() -> Result<String, &'static str> {
    static DOCUMENTS_DIR_NAME: &str = "Documents";
    static INPUT_FILE_NAME: &str = "input.txt";

    let mut result = Err("Unable to retrieve the input file path!");

    if let Ok(Some(mut path)) = my_home() {
        path.push(DOCUMENTS_DIR_NAME);
        path.push(INPUT_FILE_NAME);

        if let Some(path) = path.to_str() {
            result = Ok(path.to_string());
        }
    }

    result
}

fn read_input(file_path: &str, input: &mut Vec<String>) {
    let input_str = fs::read_to_string(file_path).expect("File could not be opened!");

    for word in input_str.split('\n') {
        if word.len() > MIN_WORD_SIZE {
            input.push(word.to_string());
        }
    }
}

fn main() {
    utilities::clear_screen();
    let mut input = Vec::new();

    if let Ok(input_file) = build_input_file_path() {
        read_input(&input_file, &mut input);
    }

    loop {
        if input.is_empty() {
            println!("No words to guess!");
            break;
        }

        let word_to_guess_index = rand::thread_rng().gen_range(0..=input.len() - 1);
        let word_to_guess = input[word_to_guess_index].to_string();
        let word_size_successfully_guessed = guessing::guess_word_size(&word_to_guess);

        if word_size_successfully_guessed {
            let word_to_guess_size = word_to_guess.len();
            println!(
                "Congrats! You guessed the word size: {} characters",
                word_to_guess_size
            );
            println!("Now it's time to guess the word.");
            let word_successfully_guessed = guessing::guess_word(&word_to_guess);

            if word_successfully_guessed {
                match word_to_guess_size {
                    0 => println!("\nThe word contains no characters!"),
                    1 | 2 => println!(
                        "\nNo characters to guess, the word is: \"{}\"",
                        word_to_guess
                    ),
                    _ => println!("\nCongrats! You guessed the word: \"{}\".", word_to_guess),
                }

                println!("Now it's time for another word.\n");
                continue;
            }
        }

        break;
    }

    println!("Aborted");
}
