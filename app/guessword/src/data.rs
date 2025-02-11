use crate::ioutils;
use ioutils::IOError;
use learn_rust_lib::utilities;
use std::{collections::HashSet, io};

struct ConsolidationStatistics {
    initial_words_count: usize,
    added_words_count: usize,
    duplicate_words_count: usize,
}

type Data = Vec<String>;

pub struct DataManager {
    data: Data,
    io_manager: ioutils::IOManager,
    is_initialized: bool,
}

impl DataManager {
    pub fn create(io_manager: ioutils::IOManager) -> DataManager {
        DataManager {
            data: Data::new(),
            io_manager,
            is_initialized: false,
        }
    }

    pub fn init(&mut self) -> Result<(), IOError> {
        self.is_initialized = true;
        let result = self.io_manager.load();

        match result {
            Ok(data) => {
                self.data = data;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn data(&self) -> &Data {
        &self.data
    }

    pub fn take_input_from_user(&mut self) -> Result<bool, IOError> {
        debug_assert!(self.is_initialized);

        let mut result = Ok(false);
        let mut provided_words = Vec::new();

        while result == Ok(false) {
            if let Err(err) = Self::request_user_input(&mut provided_words) {
                result = Err(err);
                break;
            }

            if provided_words.is_empty() {
                utilities::clear_screen();
                Self::display_no_data_provided_message();
                break;
            }

            match Self::prompt_for_save() {
                Ok(Some(true)) => {
                    let statistics = self.consolidate_data(provided_words);

                    utilities::clear_screen();

                    if statistics.added_words_count > 0 {
                        match self.io_manager.save(self.data.clone()) {
                            Ok(_) => {
                                result = Ok(true);

                                Self::display_data_saved_message(
                                    &statistics,
                                    self.io_manager.get_data_file(),
                                );
                            }
                            Err(error) => {
                                result = Err(error);
                            }
                        }
                    } else {
                        Self::display_provided_data_is_duplicate_message();
                    }
                }
                Ok(Some(false)) => {
                    utilities::clear_screen();
                    Self::display_saving_aborted_message();
                }
                Ok(None) => {
                    utilities::clear_screen();
                    continue;
                }
                Err(_) => {
                    result = Err(IOError::UserInputError);
                }
            }

            break;
        }

        result
    }

    fn request_user_input(user_input: &mut Data) -> Result<(), IOError> {
        let mut result = Ok(());

        loop {
            println!("Enter a new word (press ENTER to abort): ");

            let mut new_word = String::new();

            if let Err(_) = io::stdin().read_line(&mut new_word) {
                result = Err(IOError::UserInputError);
                break;
            }

            new_word = new_word.trim().to_string();

            if new_word.is_empty() {
                break;
            }

            utilities::clear_screen();

            if !new_word.chars().all(|ch| ch.is_ascii_alphabetic())
                || new_word.len() < ioutils::MIN_WORD_SIZE
            {
                println!("The word \"{new_word}\" contains invalid characters and/or doesn't have the required minimum chars count ({})!", ioutils::MIN_WORD_SIZE);
                println!("Please try again\n");
                continue;
            }

            user_input.push(
                new_word
                    .chars()
                    .map(|ch| ch.to_ascii_lowercase())
                    .collect::<String>(),
            );

            println!("Word added!\n");
        }

        result
    }

    fn prompt_for_save() -> Result<Option<bool>, ()> {
        utilities::cta::execute_yes_no_cancel_cta("Do you want to save the changes?")
    }

    fn consolidate_data(&mut self, input_data: Data) -> ConsolidationStatistics {
        let mut statistics = ConsolidationStatistics {
            initial_words_count: self.data.len(),
            added_words_count: 0,
            duplicate_words_count: 0,
        };

        let input_entries_count = input_data.len();
        let mut consolidation_hash: HashSet<String> =
            self.data.iter().map(|item| item.to_string()).collect();

        for word in input_data.into_iter() {
            consolidation_hash.insert(word);
        }

        statistics.added_words_count = consolidation_hash.len() - statistics.initial_words_count;
        statistics.duplicate_words_count = input_entries_count - statistics.added_words_count;

        if statistics.added_words_count > 0 {
            self.data = consolidation_hash.into_iter().collect();
        }

        statistics
    }

    fn display_no_data_provided_message() {
        println!("No new words provided!");
    }

    fn display_data_saved_message(statistics: &ConsolidationStatistics, data_file: &str) {
        println!("Initial words count: {}", statistics.initial_words_count);
        println!(
            "\n{} new words accepted, {} are duplicates",
            statistics.added_words_count, statistics.duplicate_words_count
        );
        println!(
            "{} words added to {}\n",
            statistics.added_words_count, data_file
        );
        println!(
            "Total words count is now: {}",
            statistics.initial_words_count + statistics.added_words_count
        );
    }

    fn display_provided_data_is_duplicate_message() {
        println!("All provided words are already contained within file!");
    }

    fn display_saving_aborted_message() {
        println!("Saving aborted!");
    }
}
