use learn_rust_lib::utilities;
use learn_rust_lib::utilities::random;
use learn_rust_lib::utilities::random::IndexGenerator;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    io, process,
};

#[derive(PartialEq)]
pub enum Status {
    Init,
    Running,
    Paused,
    Resumed,
    Restarted,
    Stopped,
    Aborted,
}

struct GuessingContext {
    word_to_guess: String,
    word_to_display: String, // word displayed as guessing hint (with placeholders that are gradually filled in as the user guesses the chars)
    chars_left_to_guess: HashMap<char, HashSet<usize>>,
    guessed_chars: HashSet<char>,
    word_size_guessed: bool,
}

impl GuessingContext {
    fn create() -> GuessingContext {
        GuessingContext {
            word_to_guess: String::new(),
            word_to_display: String::new(),
            chars_left_to_guess: HashMap::new(),
            guessed_chars: HashSet::new(),
            word_size_guessed: false,
        }
    }

    fn reset(&mut self) {
        self.word_to_guess.clear();
        self.word_to_display.clear();
        self.chars_left_to_guess.clear();
        self.guessed_chars.clear();
        self.word_size_guessed = false;
    }
}

pub struct GuessingEngine {
    data: Vec<String>,
    generator: random::QuickIndexGenerator,
    context: GuessingContext,
    status: Status,
}

impl GuessingEngine {
    pub fn create(data: Vec<String>) -> GuessingEngine {
        GuessingEngine {
            generator: random::QuickIndexGenerator::create(data.len()),
            data,
            context: GuessingContext::create(),
            status: Status::Init,
        }
    }

    pub fn run(&mut self) -> &Status {
        if self.status == Status::Init || self.status == Status::Restarted {
            self.status = Status::Running;
        } else {
            debug_assert!(self.status == Status::Resumed);
        }

        loop {
            if self.status == Status::Running {
                self.build_context();
            }

            if !self.context.word_size_guessed {
                self.guess_word_size();

                if self.status == Status::Paused || self.status == Status::Aborted {
                    break;
                }

                self.display_word_size_guessed_message();
            }

            self.guess_word();

            if self.status == Status::Paused || self.status == Status::Aborted {
                break;
            }

            self.display_word_guessed_message();

            if self.status == Status::Resumed {
                self.status = Status::Running;
            }

            self.context.reset();

            if self.generator.is_empty() {
                self.status = Status::Stopped;
                break;
            }

            Self::display_continue_guessing_message();
        }

        &self.status
    }

    pub fn resume(&mut self) -> &Status {
        if self.status == Status::Paused {
            self.status = Status::Resumed;
            self.run();
        } else {
            debug_assert!(self.status == Status::Paused);
        }

        &self.status
    }

    pub fn reset(&mut self, data: Vec<String>) -> &Status {
        self.generator = random::QuickIndexGenerator::create(data.len());
        self.data = data;
        self.context.reset();
        self.status = Status::Restarted;

        self.run()
    }

    fn guess_word_size(&mut self) {
        let word_size_to_guess = self.context.word_to_guess.chars().count();

        loop {
            let result = self.request_guessing_word_size();

            if result.is_empty() {
                self.status = Status::Aborted;
                break;
            } else if result == ":".to_string() {
                self.status = Status::Paused;
                break;
            }

            let guessed_word_size: usize = match result.parse() {
                Ok(guessed_nr) => guessed_nr,
                Err(_) => {
                    Self::display_invalid_input_message();
                    continue;
                }
            };

            match guessed_word_size.cmp(&word_size_to_guess) {
                Ordering::Less => Self::display_not_matching_size_message(false),
                Ordering::Greater => Self::display_not_matching_size_message(true),
                Ordering::Equal => {
                    self.context.word_size_guessed = true;
                    break;
                }
            }
        }
    }

    fn guess_word(&mut self) {
        let word_to_guess_size = self.context.word_to_guess.chars().count();
        let mut nr_of_chars_to_guess = if word_to_guess_size < 2 {
            0
        } else {
            word_to_guess_size - 2
        }; // exclude beginning and ending char (should be displayed)

        debug_assert!(nr_of_chars_to_guess > 0);

        while !self.context.chars_left_to_guess.is_empty() {
            let input_char;
            let result = self.request_guessing_a_char(nr_of_chars_to_guess);

            match result {
                Some(':') => {
                    self.status = Status::Paused;
                    break;
                }
                Some(ch) => {
                    input_char = ch;
                }
                None => {
                    self.status = Status::Aborted;
                    break;
                }
            }

            if self.context.guessed_chars.contains(&input_char) {
                Self::display_char_not_guessed_message(input_char, true);
                continue;
            }

            if let Some(occurrence_indexes) = self.context.chars_left_to_guess.get(&input_char) {
                let found_occurrences_count = occurrence_indexes.len();

                debug_assert!(found_occurrences_count <= nr_of_chars_to_guess);

                nr_of_chars_to_guess -= found_occurrences_count;
                let replaced_occurrences_count = utilities::replace_chars_in_string(
                    input_char,
                    &occurrence_indexes,
                    &mut self.context.word_to_display,
                );

                debug_assert!(replaced_occurrences_count == found_occurrences_count);

                self.context.chars_left_to_guess.remove(&input_char);
                self.context.guessed_chars.insert(input_char);

                Self::display_guessed_char_message(input_char, found_occurrences_count);
            } else {
                Self::display_char_not_guessed_message(input_char, false);
            }
        }
    }

    fn build_context(&mut self) {
        debug_assert!(self.status == Status::Running);

        let word_to_guess_index = self
            .generator
            .generate()
            .expect("There should have been at least one word left to guess!");

        let word_to_guess = self.data[word_to_guess_index].to_string();

        if word_to_guess.len() > 2 {
            self.context.word_to_guess = word_to_guess;
            self.build_chars_left_to_guess_map();
            self.compute_word_to_display_initial_value();
        } else {
            debug_assert!(
                false,
                "There is nothing to guess here, the word is too small!"
            );
        }
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

    fn request_guessing_word_size(&mut self) -> String {
        println!("Please guess the word size (press ENTER to abort, enter \':\' for new words entry menu):");
        let mut guessed_word_size = String::new();

        io::stdin()
            .read_line(&mut guessed_word_size)
            .unwrap_or_else(|err| {
                eprintln!("Failed reading the number of digits: {err}");
                process::exit(-1)
            });

        guessed_word_size = guessed_word_size.trim().to_string();
        guessed_word_size
    }

    fn display_word_size_guessed_message(&self) {
        utilities::clear_screen();

        println!(
            "Congrats! You guessed the word size: {} characters",
            self.context.word_to_guess.len()
        );

        println!("Now it's time to guess the word.\n");
    }

    fn display_not_matching_size_message(is_higher: bool) {
        utilities::clear_screen();

        if is_higher {
            println!("Please enter a lower number!");
        } else {
            println!("Please enter a higher number!");
        }
    }

    fn request_guessing_a_char(&mut self, nr_of_chars_to_guess: usize) -> Option<char> {
        println!("Please guess: \"{}\"", self.context.word_to_display);
        println!("({} char(s) to guess)", nr_of_chars_to_guess);
        println!(
            "\nEnter a character (press ENTER to abort, enter \':\' for new words entry menu):"
        );

        let mut result = None;
        let mut input_char: char = '\0';
        let char_successfully_read = utilities::read_char(&mut input_char);

        utilities::clear_screen();

        if char_successfully_read {
            utilities::convert_char_to_lowercase(&mut input_char);
            result = Some(input_char)
        }

        result
    }

    fn display_guessed_char_message(ch: char, found_occurrences_count: usize) {
        println!("Found {found_occurrences_count} occurrences of char: \'{ch}\'\n");
    }

    fn display_char_not_guessed_message(ch: char, already_guessed: bool) {
        if already_guessed {
            println!("Char \'{ch}\' already guessed! Please try again with another char.\n");
        } else {
            println!("None of the blanks matches char: \'{ch}\'. Please try again.\n");
        }
    }

    fn display_word_guessed_message(&self) {
        println!(
            "Congrats! You guessed the word: \"{}\".",
            self.context.word_to_guess
        );
    }

    fn display_continue_guessing_message() {
        println!("Now it's time for another word.\n");
    }

    fn display_invalid_input_message() {
        utilities::clear_screen();
        println!("Invalid input! Please try again.");
    }
}
