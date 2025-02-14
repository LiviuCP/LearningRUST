use std::{collections::HashSet, io};

pub mod cta;
pub mod random;

pub fn clear_screen() {
    std::process::Command::new("clear").status().unwrap();
}

pub fn read_char(ch: &mut char) -> Result<bool, ()> {
    let mut result = Ok(false);
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap_or_else(|_err| {
        result = Err(());
        0 // set number of read bytes to 0 in case of user input error
    });

    if result == Ok(false) {
        let mut is_buffer_empty = buffer.is_empty();

        if !is_buffer_empty {
            let first_char = buffer.chars().nth(0).unwrap();

            if first_char != '\n' {
                *ch = first_char;
            } else {
                is_buffer_empty = true;
            }
        }

        result = Ok(!is_buffer_empty);
    }

    result
}

pub fn read_line(str: &mut String) -> Result<bool, ()> {
    let mut result = Ok(false);

    io::stdin().read_line(str).unwrap_or_else(|_err| {
        result = Err(());
        0 // set number of read bytes to 0 in case of user input error
    });

    if result == Ok(false) && !str.is_empty() {
        result = Ok(true);
    }

    result
}

pub fn convert_char_to_lowercase(input_char: &mut char) {
    for ch in input_char.to_lowercase() {
        *input_char = ch;
        break;
    }
}

pub fn convert_char_to_uppercase(input_char: &mut char) {
    for ch in input_char.to_uppercase() {
        *input_char = ch;
        break;
    }
}

pub fn replace_chars_in_string(
    replacing_char: char,
    replaced_chars_indexes: &HashSet<usize>,
    string_to_update: &mut String,
) -> usize {
    let mut string_to_update_chars: Vec<char> = (*string_to_update).chars().collect();
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

pub fn push_multiple_times_to_vec<T: Copy>(value: &T, count: usize, arr: &mut Vec<T>) {
    let mut times_to_push = count;

    while times_to_push > 0 {
        arr.push(*value);
        times_to_push -= 1;
    }
}

pub fn get_digits(number: u32) -> Vec<u8> {
    let chars_vec: Vec<char> = number.to_string().chars().collect();
    chars_vec.iter().map(|ch| *ch as u8 - '0' as u8).collect()
}
