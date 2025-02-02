use learn_rust_lib::utilities;
use learn_rust_lib::utilities::random;
use learn_rust_lib::utilities::random::IndexGenerator;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io, process,
};

struct GuessingContext {
    word_to_guess: String,
    word_to_display: String, // word displayed as guessing hint (with placeholders that are gradually filled in as the user guesses the chars)
    chars_left_to_guess: HashMap<char, HashSet<usize>>,
    guessed_chars: HashSet<char>,
    is_size_guessed: bool,
}

impl GuessingContext {
    fn create() -> GuessingContext {
        GuessingContext {
            word_to_guess: String::new(),
            word_to_display: String::new(),
            chars_left_to_guess: HashMap::new(),
            guessed_chars: HashSet::new(),
            is_size_guessed: false,
        }
    }

    fn reset(&mut self) {
        self.word_to_guess.clear();
        self.word_to_display.clear();
        self.chars_left_to_guess.clear();
        self.guessed_chars.clear();
        self.is_size_guessed = false;
    }
}

pub struct GuessingEngine {
    data: Vec<String>,
    generator: random::QuickIndexGenerator,
    context: GuessingContext,
}

impl GuessingEngine {
    pub fn create(data: Vec<String>) -> GuessingEngine {
        GuessingEngine {
            generator: random::QuickIndexGenerator::create(data.len()),
            data,
            context: GuessingContext::create(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let word_to_guess_index = self
                .generator
                .generate()
                .expect("There should have been at least one word left to guess!");

            self.context.word_to_guess = self.data[word_to_guess_index].to_string();
            self.guess_word_size();

            if !self.context.is_size_guessed {
                println!("Aborted");
                break;
            }

            println!(
                "Congrats! You guessed the word size: {} characters",
                self.context.word_to_guess.len()
            );

            println!("Now it's time to guess the word.");

            let word_successfully_guessed = self.guess_word();

            if !word_successfully_guessed {
                println!("Aborted");
                break;
            }

            println!(
                "\nCongrats! You guessed the word: \"{}\".",
                self.context.word_to_guess
            );

            self.context.reset();

            if self.generator.is_empty() {
                println!("There are no more words left to guess!");
                break;
            }

            println!("Now it's time for another word.\n");
        }
    }

    fn guess_word_size(&mut self) {
        let word_size_to_guess = self.context.word_to_guess.chars().count();

        loop {
            println!("Please guess the word size (press ENTER to abort):");
            let mut guessed_word_size = String::new();

            io::stdin()
                .read_line(&mut guessed_word_size)
                .unwrap_or_else(|err| {
                    eprintln!("Failed reading the number of digits: {err}");
                    process::exit(-1)
                });

            utilities::clear_screen();

            guessed_word_size = guessed_word_size.trim().to_string();

            if guessed_word_size.len() == 0 {
                break;
            }

            let guessed_word_size: usize = match guessed_word_size.parse() {
                Ok(guessed_nr) => guessed_nr,
                Err(_) => {
                    println!("Invalid input! Please try again.");
                    continue;
                }
            };

            match guessed_word_size.cmp(&word_size_to_guess) {
                Ordering::Less => println!("Please enter a higher number!"),
                Ordering::Greater => println!("Please enter a smaller number!"),
                Ordering::Equal => {
                    self.context.is_size_guessed = true;
                    break;
                }
            }
        }
    }

    fn guess_word(&mut self) -> bool {
        let word_to_guess_size = self.context.word_to_guess.chars().count();
        let mut nr_of_chars_to_guess: usize = if word_to_guess_size < 2 {
            0
        } else {
            word_to_guess_size - 2
        }; // exclude beginning and ending char (should be displayed)

        self.build_chars_left_to_guess_map();
        let mut word_successfully_guessed = false;

        if nr_of_chars_to_guess > 0 {
            self.compute_word_to_display_initial_value();
        } else {
            word_successfully_guessed = true; // nothing to guess here as the first and last characters are displayed anyway
        }

        while !word_successfully_guessed {
            println!("\nPlease guess: \"{}\"", self.context.word_to_display);
            println!("({} char(s) to guess)", nr_of_chars_to_guess);
            println!("\nEnter a character:");

            let mut input_char: char = '\0';
            let char_successfully_read = utilities::read_char(&mut input_char);

            utilities::clear_screen();

            if !char_successfully_read {
                break;
            }

            utilities::convert_char_to_lowercase(&mut input_char);

            if self.context.guessed_chars.contains(&input_char) {
                println!("Char \'{}\' already guessed!", input_char);
                println!("Please try again with another char.");
                continue;
            }

            if let Some(occurrence_indexes) = self.context.chars_left_to_guess.get(&input_char) {
                let found_occurrences_count = occurrence_indexes.len();

                assert!(found_occurrences_count <= nr_of_chars_to_guess);

                nr_of_chars_to_guess -= found_occurrences_count;
                let replaced_occurrences_count = utilities::replace_chars_in_string(
                    input_char,
                    &occurrence_indexes,
                    &mut self.context.word_to_display,
                );

                assert!(replaced_occurrences_count == found_occurrences_count);

                self.context.chars_left_to_guess.remove(&input_char);
                self.context.guessed_chars.insert(input_char);

                println!(
                    "Found {} occurrences of char: \'{}\'",
                    found_occurrences_count, input_char
                );
            } else {
                println!(
                    "The word doesn't contain char: \'{}\'. Please try again.",
                    input_char
                );
            }

            word_successfully_guessed = self.context.chars_left_to_guess.len() == 0;
        }

        word_successfully_guessed
    }

    fn compute_word_to_display_initial_value(&mut self) {
        let word_to_guess_chars: Vec<char> = self.context.word_to_guess.chars().collect();
        let word_to_guess_size = word_to_guess_chars.len();

        if word_to_guess_size > 2 {
            // the guessing hint should initially contain underscore placeholders for all characters except the first and last
            self.context.word_to_display.push(word_to_guess_chars[0]);
            let mut nr_of_underscores_to_add = word_to_guess_size - 2;

            while nr_of_underscores_to_add > 0 {
                self.context.word_to_display.push('-');
                nr_of_underscores_to_add -= 1;
            }

            self.context
                .word_to_display
                .push(word_to_guess_chars[word_to_guess_size - 1]);
        } else {
            self.context.word_to_display = self.context.word_to_guess.clone();
        }
    }

    fn build_chars_left_to_guess_map(&mut self) {
        let word_to_guess_chars: Vec<char> = self.context.word_to_guess.chars().collect();
        let word_to_guess_size = word_to_guess_chars.len();

        if word_to_guess_size > 2 {
            let chars_to_guess = &word_to_guess_chars[1..word_to_guess_size - 1];

            for (index, ch) in chars_to_guess.iter().enumerate() {
                let occurrence_indexes = self
                    .context
                    .chars_left_to_guess
                    .entry(*ch)
                    .or_insert(HashSet::<usize>::new());
                (*occurrence_indexes).insert(index + 1); // index should be mapped to actual char index in word_to_guess (take first char into account)
            }
        }
    }
}
