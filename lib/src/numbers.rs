use std::collections::HashMap;
use crate::utilities;

pub fn reverse_int(input:(u64, u8)) -> (u64, u8) {
    fn update_input_and_output_numbers(input_number: &mut u64, output_number: &mut u64) -> u8 {
	let found_digit = *input_number % 10;
	*output_number = *output_number * 10 + found_digit;
	*input_number = *input_number / 10;

	found_digit as u8
    }

    let mut input_number = input.0;
    let mut output_number = 0;
    let mut output_preceding_zeroes_count:u8 = 0;

    // find all trailing zeroes and convert them to preceding zeroes
    while input_number > 0 {
	let found_digit = update_input_and_output_numbers(&mut input_number, &mut output_number);

	if found_digit != 0 {
            break;
	}

	output_preceding_zeroes_count += 1;
    }

    // handle remaining digits
    while input_number > 0 {
	update_input_and_output_numbers(&mut input_number, &mut output_number);
    }

    if output_number == 0 {
	output_preceding_zeroes_count = input.1;
    }
    else {
	output_number *= u64::pow(10, input.1 as u32);
    }

    (output_number, output_preceding_zeroes_count)
}

pub fn compute_median(numbers_list: &Vec::<i32>) -> Option<i32> {
    let mut result: Option::<i32> = None;
    let list_size = numbers_list.len();

    if list_size > 0 {
	let mut sorted_numbers_list = numbers_list.clone(); // the original list should not be modified, we work on a copy
	sorted_numbers_list.sort();

	let median_index = list_size / 2;
	result = Some(sorted_numbers_list[median_index]);

	if list_size % 2 == 0 {
	    // actually it should be the exact floating point average, i.e. 2 + 3 = 2,5 (not 2), however for simplicity we will do a "floor" average
	    result = Some((sorted_numbers_list[median_index-1] + result.unwrap()) / 2);
	}
    }

    result
}

// computes the value(s) that occur(s) most often (mode = number of occurrences of this/these value(s))
pub fn compute_mode(numbers_list: &Vec::<i32>) -> (usize, Vec::<i32>) {
    let mut occurrences_map = HashMap::new();

    // count the occurrences of each value from input vector
    for value in numbers_list.iter() {
	let value_occurrences = occurrences_map.entry(*value).or_insert(0);
	*value_occurrences += 1;
    }

    let mut values_with_mode = Vec::<i32>::new(); // value(s) with max number of occurrences
    let mut mode:usize = 0; // max number of occurrences

    for (value, occurrences) in &occurrences_map {
	if *occurrences > mode {
	    mode = *occurrences;
	    values_with_mode.clear();
	    values_with_mode.push(*value);
	}
	else if *occurrences == mode {
	    values_with_mode.push(*value);
	}
    }

    // sort the values with max occurrences to get a consistent result (not mandatory but useful for testing)
    values_with_mode.sort();

    (mode, values_with_mode)
}

/*
This method converts a number between 1 and 4999 to a roman numeral by using multiple integer thresholds (along with (modulo) division operations)
for obtaining the output (roman) chars
*/
pub fn convert_number_to_roman_numeral(number: u16) -> Vec::<char> {
    // single char, appended one or multiple times (e.g. 'D' for 500, 'MMM' for 3000) for the given threshold (if it applies: remainder >= threshold)
    fn handle_same_appended_char_threshold(threshold: u16, remainder: &mut u16, char_to_append: char, result: &mut Vec::<char>) -> bool {
	let can_handle = *remainder >= threshold;

	if can_handle {
	    utilities::push_multiple_times_to_vec(&char_to_append, (*remainder / threshold) as usize, result);
	    *remainder = *remainder % threshold;
	}

	can_handle
    }

    // multiple different chars, appended one time each for the given threshold (if it applies: remainder >= threshold)
    fn handle_different_appended_chars_threshold(threshold: u16, remainder: &mut u16, chars_to_append: &mut Vec::<char>, result: &mut Vec::<char>) -> bool {
	let can_handle = *remainder >= threshold;

	if can_handle {
	    (*result).append(chars_to_append);
	    *remainder = *remainder % threshold;
	}

	can_handle
    }

    let mut result = Vec::<char>::new();
    let mut remainder = number;

    if remainder > 0 && remainder < 5000 {
	handle_same_appended_char_threshold(1000, &mut remainder, 'M', &mut result);

	// mutually exclusive: 900 - 999, 500 - 899, 400 - 499
	let handled = handle_different_appended_chars_threshold(900, &mut remainder, &mut vec!['C','M'], &mut result);

	if !handled {
	    let handled = handle_same_appended_char_threshold(500, &mut remainder, 'D', &mut result);

	    if !handled {
		handle_different_appended_chars_threshold(400, &mut remainder, &mut vec!['C','D'], &mut result);
	    }
	}

	handle_same_appended_char_threshold(100, &mut remainder, 'C', &mut result);

	// mutually exclusive: 90 - 99, 50 - 89, 40 - 49
	let handled = handle_different_appended_chars_threshold(90, &mut remainder, &mut vec!['X','C'], &mut result);

	if !handled {
	    let handled = handle_same_appended_char_threshold(50, &mut remainder, 'L', &mut result);

	    if !handled {
		handle_different_appended_chars_threshold(40, &mut remainder, &mut vec!['X','L'], &mut result);
	    }
	}

	handle_same_appended_char_threshold(10, &mut remainder, 'X', &mut result);

	// mutually exclusive: 9, 5 - 8, 4
	let handled = handle_different_appended_chars_threshold(9, &mut remainder, &mut vec!['I','X'], &mut result);

	if !handled {
	    let handled = handle_same_appended_char_threshold(5, &mut remainder, 'V', &mut result);

	    if !handled {
		handle_different_appended_chars_threshold(4, &mut remainder, &mut vec!['I','V'], &mut result);
	    }
	}

	handle_same_appended_char_threshold(1, &mut remainder, 'I', &mut result);
    }

    result
}

// TODO: create static map (probably by using crates.io)
fn build_roman_numerals_conversion_map() -> HashMap::<(usize, char), Vec::<char>> {
    let result = HashMap::from([
	((0 as usize, '1'), vec!('I')),
	((0 as usize, '2'), vec!('I', 'I')),
	((0 as usize, '3'), vec!('I', 'I', 'I')),
	((0 as usize, '4'), vec!('I', 'V')),
	((0 as usize, '5'), vec!('V')),
	((0 as usize, '6'), vec!('V', 'I')),
	((0 as usize, '7'), vec!('V', 'I', 'I')),
	((0 as usize, '8'), vec!('V', 'I', 'I', 'I')),
	((0 as usize, '9'), vec!('I', 'X')),
	((1 as usize, '1'), vec!('X')),
	((1 as usize, '2'), vec!('X', 'X')),
	((1 as usize, '3'), vec!('X', 'X', 'X')),
	((1 as usize, '4'), vec!('X', 'L')),
	((1 as usize, '5'), vec!('L')),
	((1 as usize, '6'), vec!('L', 'X')),
	((1 as usize, '7'), vec!('L', 'X', 'X')),
	((1 as usize, '8'), vec!('L', 'X', 'X', 'X')),
	((1 as usize, '9'), vec!('X', 'C')),
	((2 as usize, '1'), vec!('C')),
	((2 as usize, '2'), vec!('C', 'C')),
	((2 as usize, '3'), vec!('C', 'C', 'C')),
	((2 as usize, '4'), vec!('C', 'D')),
	((2 as usize, '5'), vec!('D')),
	((2 as usize, '6'), vec!('D', 'C')),
	((2 as usize, '7'), vec!('D', 'C', 'C')),
	((2 as usize, '8'), vec!('D', 'C', 'C', 'C')),
	((2 as usize, '9'), vec!('C', 'M')),
	((3 as usize, '1'), vec!('M')),
	((3 as usize, '2'), vec!('M', 'M')),
	((3 as usize, '3'), vec!('M', 'M', 'M')),
	((3 as usize, '4'), vec!('M', 'M', 'M', 'M'))
    ]);

    result
}

pub fn convert_number_to_roman_numeral_using_hash(number: u16) -> Vec::<char> {
    let mut result = Vec::<char>::new();

    if number > 0 && number < 5000 {
	let conversion_map = build_roman_numerals_conversion_map();
	let digits: Vec::<char> = number.to_string().chars().collect();

	// no need to check the digits_count is > 0 for getting power_of_ten_index (see below), an integer should have min. 1 digit
	let digits_count = digits.len();

	for (index, digit_as_char) in digits.iter().enumerate() {
	    let power_of_ten_index = digits_count - 1 - index; // place of the digit within the number, e.g. for thousands it is 3 corresponding to 10^3
	    let mut chars_to_append: Vec::<char> = conversion_map.get(&(power_of_ten_index, *digit_as_char)).unwrap_or(&Vec::new()).to_vec();
	    result.append(&mut chars_to_append);
	}
    }

    result
}
