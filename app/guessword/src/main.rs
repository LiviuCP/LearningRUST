mod guessing;
mod ioutils;

use learn_rust_lib::utilities;
use learn_rust_lib::utilities::random;
use std::process;

fn main() {
    utilities::clear_screen();

    let input_file = ioutils::build_input_file_path().unwrap_or_else(|err| {
        eprintln!("Error! {err}");
        process::exit(-1);
    });

    let input = ioutils::read_input(&input_file).unwrap_or_else(|err| {
        eprintln!("Error in processing input file! {err}");
        process::exit(-1);
    });

    let mut generator = random::IndexGenerator::create(input.len());

    loop {
        let word_to_guess_index = generator
            .generate()
            .expect("There should have been at least one word left to guess!");

        let word_to_guess = input[word_to_guess_index].to_string();
        let word_size_successfully_guessed = guessing::guess_word_size(&word_to_guess);

        if !word_size_successfully_guessed {
            println!("Aborted");
            break;
        }

        println!(
            "Congrats! You guessed the word size: {} characters",
            word_to_guess.len()
        );

        println!("Now it's time to guess the word.");

        let word_successfully_guessed = guessing::guess_word(&word_to_guess);

        if !word_successfully_guessed {
            println!("Aborted");
            break;
        }

        println!("\nCongrats! You guessed the word: \"{}\".", word_to_guess);

        if generator.is_empty() {
            println!("There are no more words left to guess!");
            break;
        }

        println!("Now it's time for another word.\n");
    }
}
