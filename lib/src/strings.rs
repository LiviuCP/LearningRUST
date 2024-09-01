use std::collections::HashSet;
use super::utilities;

/* Converts the string as follows:
- if the string starts with a vowel it appends "-hay" to the end, e.g. "orange" becomes "orange-hay"
- if the string starts with a consonant it takes the consonant and appends following substring to the end: "-[consonant]ay", e.g. "story" becomes "tory-say"

Note: strings containing invalid characters (other than alpha-numeric) are discarded and an empty result is returned. Same is valid for empty strings or 1-char strings where the character is a consonant.
*/
pub fn convert_to_pig_latin(input_string: &String) -> String {
    let input_string_chars: Vec::<char> = input_string.chars().collect();
    let input_string_size = input_string_chars.len();
    let mut is_input_valid = false;
    let mut is_starting_char_vowel = false;
    let mut output_string_chars = Vec::<char>::new();

    if input_string_size > 0 {
	// for simplicity we only consider the "standard" latin vowels
	is_starting_char_vowel = HashSet::from(['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']).contains(&input_string_chars[0]);
	is_input_valid = is_starting_char_vowel || input_string_size > 1;

	// if the string size is exactly one character then this one should be a vowel
	if is_input_valid {
	    for ch in input_string_chars.iter() {
		if !ch.is_alphabetic() {
		    is_input_valid = false;
		    break;
		}
	    }
	}
    }

    if is_input_valid {
	// suffix has 4 characters ("-hay") but for consonants the second suffix char is the starting input string character so less space should be reserved
	let suffix_space_to_reserve = if is_starting_char_vowel {4} else {3};
	output_string_chars.reserve(input_string_size + suffix_space_to_reserve);

	let copy_starting_index = if is_starting_char_vowel {0} else {1};

	for ch in (&input_string_chars[copy_starting_index..]).iter() {
	    let mut current_ch = *ch;
	    utilities::convert_char_to_lowercase(&mut current_ch);
	    output_string_chars.push(current_ch);
	}

	let mut second_suffix_char = 'h';

	if !is_starting_char_vowel {
	    second_suffix_char = input_string_chars[0];
	    utilities::convert_char_to_lowercase(&mut second_suffix_char);
	}

	output_string_chars.push('-');
	output_string_chars.push(second_suffix_char);
	output_string_chars.push('a');
	output_string_chars.push('y');
    }

    output_string_chars.iter().collect::<String>()
}
