use std::{io, collections::HashSet};

pub fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn read_char(ch: &mut char) -> bool {
    let mut buffer = String::new();

    io::stdin()
	.read_line(&mut buffer)
	.expect("Failed reading the line!");

    let mut is_buffer_empty = buffer.len() == 0;

    if !is_buffer_empty {
	let first_char = buffer.chars().nth(0).unwrap();

	if first_char != '\n' {
            *ch = first_char;
	}
	else {
            is_buffer_empty = true;
	}
    }

    !is_buffer_empty
}

pub fn convert_char_to_lowercase(input_char: &mut char) {
    for ch in input_char.to_lowercase() {
	*input_char = ch;
	break;
    }
}

pub fn replace_chars_in_string(replacing_char: char, replaced_chars_indexes: &HashSet::<usize>, string_to_update: &mut String) -> usize {
    let mut string_to_update_chars:Vec<char> = (*string_to_update).chars().collect();
    let string_to_update_size = string_to_update_chars.len();
    let mut replaced_chars_count = 0;

    for index in replaced_chars_indexes {
	if *index < string_to_update_size {
	    string_to_update_chars[*index] = replacing_char;
	    replaced_chars_count += 1;
	}
    }

    *string_to_update = string_to_update_chars.iter().collect::<String>();
    replaced_chars_count
}