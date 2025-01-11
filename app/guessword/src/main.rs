mod guessing;

use learn_rust_lib::utilities;
use rand::Rng;

/*
TODO:
- include all these words in a file (JSON, simple txt file, CSV ...)
- create functionality for checking their validity when loading them into application (accept only the valid ones within array)
  - this includes allowing only specific characters (e.g. alphanumeric)
- use a vector instead of an array for storing the loaded/validated words
- create functionality for adding new words without manually modifying the file
- create functionality that removes already guessed words from array (vector) so they are not provided multiple times for guessing
- create functionality that prevents duplicate words within array
*/

const AVAILABLE_WORDS: [&str; 14] = [
    "confidential",
    "geographic",
    "availability",
    "ambiguity",
    "scalability",
    "reliability",
    "pedestrian",
    "argumentation",
    "dichotomy",
    "understanding",
    "geometry",
    "responsiveness",
    "accountability",
    "priority",
];

fn main() {
    utilities::clear_screen();

    loop {
        let word_to_guess_index = rand::thread_rng().gen_range(0..=AVAILABLE_WORDS.len() - 1);
        let word_to_guess = AVAILABLE_WORDS[word_to_guess_index].to_string();
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

    println!("Aborted!");
}
