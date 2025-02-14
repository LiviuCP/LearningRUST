mod data;
mod guessing;
mod ioutils;

use data::DataManager;
use guessing::Status;
use ioutils::IOError;
use learn_rust_lib::utilities;
use std::{env, process};

fn main() {
    let data_file = retrieve_data_file_path();
    let mut data_manager = DataManager::create(ioutils::IOManager::create(&data_file));
    let args: Vec<String> = env::args().collect();

    utilities::clear_screen();
    check_cli_args(&args);

    let is_additional_data_requested = args.len() > 1 && args[1] == "--entry";

    data_manager.init().unwrap_or_else(|err| match err {
        IOError::CannotLoadFile => {
            eprintln!("Error in loading data! File couldn't be opened");
            process::exit(-1);
        }
        _ => {
            handle_unallowed_error(err);
            process::exit(-1);
        }
    });

    let is_data_available = !data_manager.data().is_empty();

    if !is_data_available && !is_additional_data_requested {
        println!(
            "No valid data has been found.\nYou will now be redirected to the data entry menu.\n"
        );
    }

    if !is_data_available || is_additional_data_requested {
        let _ = data_manager.take_input_from_user().unwrap_or_else(|err| {
            handle_data_entry_error(err);
            false
        });

        if data_manager.data().is_empty() {
            eprintln!("No words are available for guessing!");
            process::exit(-1);
        }

        println!("\nLet's continue with guessing!\n");
    }

    let mut guessing_engine = guessing::GuessingEngine::create(data_manager.data());
    let mut status = guessing_engine.run();

    loop {
        match *status {
            Status::Paused => {
                utilities::clear_screen();
                println!("Guessing paused. Let's enter some new words!\n");

                let new_words_saved = data_manager.take_input_from_user().unwrap_or_else(|err| {
                    handle_data_entry_error(err);
                    false
                });

                if !new_words_saved {
                    println!("\nLet's resume guessing!\n");
                    status = guessing_engine.resume();
                } else {
                    println!("\nNew words added. Guessing has been restarted!\n");
                    status = guessing_engine.reset(data_manager.data());
                }
            }
            Status::Stopped => {
                println!("There are no more words left to guess!");
                break;
            }
            Status::Aborted => {
                utilities::clear_screen();
                println!("Aborted");
                break;
            }
            Status::Error => {
                println!("Aborted");
                break;
            }
            _ => {
                debug_assert!(false, "Invalid guessing engine status!");
            }
        }
    }
}

fn check_cli_args(args: &Vec<String>) {
    let args_count = args.len();

    if args_count > 2 || (args_count == 2 && args[1] != "--entry") {
        eprintln!("Invalid argument(s). The only valid option is currently \"--entry\" (enter new words).");
        eprintln!("Please try again.");

        process::exit(-1);
    }
}

fn retrieve_data_file_path() -> String {
    ioutils::build_data_file_path().unwrap_or_else(|err| {
        if err == IOError::CannotRetrieveFilePath {
            eprintln!("Error! Cannot retrieve data file path");
        } else {
            handle_unallowed_error(err);
        }

        process::exit(-1);
    })
}

fn handle_data_entry_error(err: IOError) {
    match err {
        IOError::CannotLoadFile => {
            eprintln!("Error in loading data to be updated! File couldn't be opened");
        }
        IOError::CannotSaveFile => {
            eprintln!("Error! Unable to open the file for saving");
        }
        IOError::OtherSavingError => {
            eprintln!("An error occurred while saving the words!");
        }
        IOError::UserInputError => {
            eprintln!("An error occurred while processing the user input!");
        }
        _ => {
            handle_unallowed_error(err);
        }
    }

    process::exit(-1);
}

fn handle_unallowed_error(err: IOError) {
    eprintln!("This error should not occur: {:?}", err);
    debug_assert!(false);
}
