use std::{io, cmp::Ordering, collections::{HashSet, HashMap}};
use super::utilities;

pub fn guess_word_size(word_to_guess: &String) -> bool {
    let word_size_to_guess = word_to_guess.len();
    let mut word_size_successfully_guessed = false;

    loop {
	println!("Please guess the word size (press ENTER to abort):");
	let mut guessed_word_size = String::new();

	io::stdin()
            .read_line(&mut guessed_word_size)
            .expect("Failed reading the number of digits!");

	utilities::clear_screen();

	guessed_word_size = guessed_word_size.trim().to_string();

	if guessed_word_size.len() == 0 {
            break;
	}

	let guessed_word_size:usize = match guessed_word_size.parse() {
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
		word_size_successfully_guessed = true;
		break;
            }
	}
    }

    word_size_successfully_guessed
}

pub fn guess_word(word_to_guess: &String) -> bool {
    let word_to_guess_size = word_to_guess.len();
    let mut nr_of_chars_to_guess = if word_to_guess_size < 2 {0} else {word_to_guess_size - 2}; // exclude beginning and ending char (should be displayed)
    let mut word_to_display = String::new(); // word displayed as guessing hint (with placeholders that are gradually filled in as the user guesses the chars)
    let mut guessed_chars = HashSet::new();
    let mut chars_left_to_guess = build_chars_left_to_guess_map(word_to_guess);
    let mut word_successfully_guessed = false;

    if nr_of_chars_to_guess > 0 {
	word_to_display = compute_word_to_display_initial_value(&word_to_guess);
    }
    else
    {
	word_successfully_guessed = true; // nothing to guess here as the first and last characters are displayed anyway
    }

    while !word_successfully_guessed {
	println!("\nPlease guess: \"{}\"", word_to_display);
	println!("({} char(s) to guess)", nr_of_chars_to_guess);
	println!("\nEnter a character:");

	let mut input_char: char = '\0';
	let char_successfully_read = utilities::read_char(&mut input_char);

	utilities::clear_screen();

	if !char_successfully_read {
	    break;
	}

	utilities::convert_char_to_lowercase(&mut input_char);

	if guessed_chars.contains(&input_char) {
	    println!("Char \'{}\' already guessed!", input_char);
	    println!("Please try again with another char.");
            continue;
	}

	if let Some(occurrence_indexes) = chars_left_to_guess.get(&input_char) {
	    let found_occurrences_count = occurrence_indexes.len();
	    nr_of_chars_to_guess -= found_occurrences_count;

	    // for each guessed character all occurrences are filled-in into the guessing hint
	    for index in occurrence_indexes {
		word_to_display.replace_range(*index..*index+1, &word_to_guess[*index..*index+1]);
	    }

	    chars_left_to_guess.remove(&input_char);
	    guessed_chars.insert(input_char);

	    println!("Found {} occurrences of char: \'{}\'", found_occurrences_count, input_char);

	}
	else {
            println!("The word doesn't contain char: \'{}\'. Please try again.", input_char);
	}

	word_successfully_guessed = chars_left_to_guess.len() == 0;
    }

    word_successfully_guessed
}

fn compute_word_to_display_initial_value(word_to_guess: &String) -> String {
    let mut word_to_display = String::new();
    let word_to_guess_size = word_to_guess.len();

    if word_to_guess_size > 2 {
	// the guessing hint should initially contain underscore placeholders for all characters except the first and last
	word_to_display.push(word_to_guess.chars().nth(0).unwrap());

	let mut underscores_to_add = word_to_guess_size - 2;

	while underscores_to_add > 0 {
	    word_to_display.push('_');
	    underscores_to_add -= 1;
	}

	word_to_display.push(word_to_guess.chars().nth(word_to_guess.len() - 1).unwrap());
    }
    else
    {
	word_to_display.push_str(word_to_guess.as_str());
    }

    word_to_display
}

fn build_chars_left_to_guess_map(word_to_guess: &String) -> HashMap::<char, Vec::<usize>> {
    let mut chars_left_to_guess = HashMap::<char, Vec::<usize>>::new();
    let word_to_guess_size = word_to_guess.len();

    if word_to_guess_size > 2 {
	let chars_to_guess = &word_to_guess[1..word_to_guess_size-1];

	for (index, ch) in chars_to_guess.chars().enumerate() {
	    if let Some(occurrence_indexes) = chars_left_to_guess.get_mut(&ch) {
		// the index from slice chars_to_guess should be mapped to word_to_guess string index (the slice excludes starting/ending word to guess characters)
		occurrence_indexes.push(index+1);
	    }
	    else {
		let occurrence_indexes = vec![index+1];
		chars_left_to_guess.insert(ch, occurrence_indexes);
	    }
	}
    }

    chars_left_to_guess
}
