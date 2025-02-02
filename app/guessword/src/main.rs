mod dataentry;
mod guessing;
mod ioutils;

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
    guessing_engine.run();
}
