mod dataentry;
mod guessing;
mod ioutils;

use learn_rust_lib::utilities;
use learn_rust_lib::utilities::random;
use learn_rust_lib::utilities::random::IndexGenerator;
use std::{env, process};

fn main() {
    let data_file = ioutils::build_data_file_path().unwrap_or_else(|err| {
        eprintln!("Error! {err}");
        process::exit(-1);
    });

    let mut io_manager = ioutils::IOManager::create(&data_file);
    let args: Vec<String> = env::args().collect();

    utilities::clear_screen();

    if args.len() > 1 && args[1] == "--entry" {
        dataentry::take_input_from_user(&mut io_manager);
        println!("\nLet's continue with guessing!\n");
    }

    let data = io_manager.load().unwrap_or_else(|err| {
        eprintln!("Error in loading data! {err}");
        process::exit(-1);
    });

    let mut generator = random::QuickIndexGenerator::create(data.len());

    loop {
        let word_to_guess_index = generator
            .generate()
            .expect("There should have been at least one word left to guess!");

        let word_to_guess = data[word_to_guess_index].to_string();
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
