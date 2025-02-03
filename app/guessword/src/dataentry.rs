use crate::ioutils;
use learn_rust_lib::utilities;
use std::{collections::HashSet, io, process};

struct ConsolidatedData {
    data: Vec<String>,
    initial_count: usize,
    added_count: usize,
    duplicates_count: usize,
}

pub fn take_input_from_user(io_manager: &mut ioutils::IOManager) -> bool {
    let mut saved = false;
    let data_file = io_manager.get_data_file();

    let data = io_manager.load().unwrap_or_else(|err| {
        eprintln!("Error in loading data to be updated! {err}");
        process::exit(-1);
    });

    let mut provided_words = Vec::new();

    loop {
        request_user_input(&mut provided_words);

        if provided_words.is_empty() {
            utilities::clear_screen();
            println!("No new words provided!");
            break;
        }

        match prompt_for_save() {
            Some(true) => {
                let result = consolidate_data(data, provided_words);

                utilities::clear_screen();

                println!("Initial words count: {}", result.initial_count);
                println!(
                    "\n{} new words accepted, {} are duplicates",
                    result.added_count, result.duplicates_count
                );

                if result.added_count > 0 {
                    let total_words_count = io_manager.save(result.data).unwrap_or_else(|err| {
                        eprintln!("Error! {err}");
                        process::exit(-1);
                    });

                    saved = true;

                    println!("{} words added to {data_file}\n", result.added_count);
                    println!("Total words count is now: {total_words_count}");
                } else {
                    println!("No new words added to file!");
                }
            }
            Some(false) => {
                utilities::clear_screen();
                println!("Saving aborted!");
            }
            _ => {
                utilities::clear_screen();
                continue;
            }
        }

        break;
    }

    saved
}

fn request_user_input(user_input: &mut Vec<String>) {
    loop {
        println!("Enter a new word (press ENTER to abort): ");

        let mut new_word = String::new();

        io::stdin().read_line(&mut new_word).unwrap_or_else(|err| {
            eprintln!("Failed reading the new word: {err}");
            process::exit(-1)
        });

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
}

fn prompt_for_save() -> Option<bool> {
    let mut result = None;

    println!("Do you want to save the changes (y - yes, n - no, c - cancel)?");

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .unwrap_or_else(|err| {
                eprintln!("Failed reading the user input: {err}");
                process::exit(-1)
            });

        match user_input.trim() {
            "y" | "Y" => {
                result = Some(true);
            }
            "n" | "N" => {
                result = Some(false);
            }
            "c" | "C" => {}
            _ => {
                println!("Invalid choice! Please try again");
                continue;
            }
        }

        break;
    }

    result
}

fn consolidate_data(initial_data: Vec<String>, input_data: Vec<String>) -> ConsolidatedData {
    let mut result = ConsolidatedData {
        data: Vec::new(),
        initial_count: initial_data.len(),
        added_count: 0,
        duplicates_count: 0,
    };

    let input_entries_count = input_data.len();
    let mut consolidation_hash: HashSet<String> = initial_data.into_iter().collect();

    for word in input_data.into_iter() {
        consolidation_hash.insert(word);
    }

    result.added_count = consolidation_hash.len() - result.initial_count;
    result.duplicates_count = input_entries_count - result.added_count;

    result.data = consolidation_hash.into_iter().collect();

    result
}
