mod dataentry;
mod guessing;
mod ioutils;

use guessing::Status;
use ioutils::IOError;
use learn_rust_lib::utilities;
use std::{env, process};

fn main() {
    let data_file = retrieve_data_file_path();
    let mut io_manager = ioutils::IOManager::create(&data_file);
    let args: Vec<String> = env::args().collect();

    utilities::clear_screen();

    let data = if args.len() > 1 && args[1] == "--entry" {
        attempt_data_entry_and_reload(&mut io_manager)
    } else {
        load_data_and_attempt_data_entry(&mut io_manager)
    };

    let mut guessing_engine = guessing::GuessingEngine::create(data);
    let mut status = guessing_engine.run();

    loop {
        match *status {
            Status::Paused => {
                utilities::clear_screen();
                println!("Guessing paused. Let's enter some new words!\n");

                let new_words_saved = dataentry::take_input_from_user(&mut io_manager)
                    .unwrap_or_else(|err| {
                        handle_data_entry_error(err);
                        false
                    });

                if !new_words_saved {
                    println!("\nLet's resume guessing!\n");
                    status = guessing_engine.resume();
                } else {
                    let data = load_data(&mut io_manager);
                    println!("\nNew words added. Guessing has been restarted!\n");
                    status = guessing_engine.reset(data);
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
            _ => {
                debug_assert!(false, "Invalid guessing engine status!");
            }
        }
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

fn load_data(io_manager: &mut ioutils::IOManager) -> Vec<String> {
    match io_manager.load() {
        Ok(data) => {
            println!("\nLet's continue with guessing!\n");
            data
        }
        Err(err) => {
            match err {
                IOError::CannotLoadFile => {
                    eprintln!("Error in loading data! File couldn't be opened");
                }
                IOError::ReadEmptyContent => {
                    eprintln!("Error in loading data! File is empty or no valid data is available");
                }
                _ => {
                    handle_unallowed_error(err);
                }
            }

            process::exit(-1);
        }
    }
}

fn load_data_and_attempt_data_entry(io_manager: &mut ioutils::IOManager) -> Vec<String> {
    io_manager.load().unwrap_or_else(|err| match err {
        IOError::CannotLoadFile => {
            eprintln!("Error in loading data! File couldn't be opened");
            process::exit(-1);
        }
        IOError::ReadEmptyContent => {
	    println!("No valid data has been found.\nYou will now be redirected to the data entry menu.\n");
	    attempt_data_entry_and_reload(io_manager)
	}
        _ => {
            handle_unallowed_error(err);
            process::exit(-1)
        }
    })
}

fn attempt_data_entry_and_reload(io_manager: &mut ioutils::IOManager) -> Vec<String> {
    let _ = dataentry::take_input_from_user(io_manager).unwrap_or_else(|err| {
        handle_data_entry_error(err);
        false
    });

    load_data(io_manager)
}

fn handle_data_entry_error(err: IOError) {
    match err {
        IOError::CannotLoadFile => {
            eprintln!("Error in loading data to be updated! File couldn't be opened");
        }
        IOError::CannotSaveFile => {
            eprintln!("Error! Unable to open the file for saving");
        }
        IOError::WriteEmptyContent => {
            eprintln!("Error! Empty content, nothing to save");
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
