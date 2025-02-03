mod dataentry;
mod guessing;
mod ioutils;

use guessing::Status;
use learn_rust_lib::utilities;
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

    let mut guessing_engine = guessing::GuessingEngine::create(data);
    let mut status = guessing_engine.run();

    loop {
        match *status {
            Status::Paused => {
                utilities::clear_screen();
                println!("Guessing paused. Let's enter some new words!\n");
                let new_words_saved = dataentry::take_input_from_user(&mut io_manager);

                if !new_words_saved {
                    println!("\nLet's resume guessing!\n");
                    status = guessing_engine.resume();
                } else {
                    let data = io_manager.load().unwrap_or_else(|err| {
                        eprintln!("Error in loading data! {err}");
                        process::exit(-1);
                    });

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
