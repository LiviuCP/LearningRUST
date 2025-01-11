use super::utilities;
use std::collections::HashSet;

fn is_starting_char_vowel(input_string_chars: &Vec<char>) -> bool {
    let mut is_vowel = false;

    if input_string_chars.len() > 0 {
        // for simplicity we only consider the "standard" latin vowels
        is_vowel = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'])
            .contains(&input_string_chars[0])
    }

    is_vowel
}

fn is_valid_pig_latin_input(input_string_chars: &Vec<char>) -> bool {
    let mut is_input_valid = false;
    let input_string_size = input_string_chars.len();

    if input_string_size > 0 {
        // if the string size is exactly one character then this one should be a vowel
        is_input_valid = input_string_size > 1 || is_starting_char_vowel(&input_string_chars);

        if is_input_valid {
            for ch in input_string_chars.iter() {
                if !ch.is_alphabetic() {
                    is_input_valid = false;
                    break;
                }
            }
        }
    }

    is_input_valid
}

/*
Converts the string as follows:
- if the string starts with a vowel it appends "-hay" to the end, e.g. "orange" becomes "orange-hay"
- if the string starts with a consonant it takes the consonant and appends following substring to the end: "-[consonant]ay", e.g. "story" becomes "tory-say"

Note: strings containing invalid characters (other than alpha-numeric) are discarded and an empty result is returned. Same is valid for empty strings or 1-char strings where the character is a consonant.
*/
pub fn convert_to_pig_latin(input_string: &String) -> String {
    let mut output_string = String::new();
    let input_string_chars: Vec<char> = input_string.chars().collect();

    if is_valid_pig_latin_input(&input_string_chars) {
        let is_first_input_char_vowel = is_starting_char_vowel(&input_string_chars);

        // for consonants first char is copied when appending the suffix
        let copy_starting_index = if is_first_input_char_vowel { 0 } else { 1 };

        output_string = (&input_string_chars[copy_starting_index..])
            .iter()
            .map(|ch| {
                let mut current_ch = *ch;
                utilities::convert_char_to_lowercase(&mut current_ch);
                current_ch
            })
            .collect();

        let mut second_suffix_char = 'h';

        if !is_first_input_char_vowel {
            second_suffix_char = input_string_chars[0];
            utilities::convert_char_to_lowercase(&mut second_suffix_char);
        }

        output_string.push('-');
        output_string.push(second_suffix_char);
        output_string.push('a');
        output_string.push('y');
    }

    output_string
}
