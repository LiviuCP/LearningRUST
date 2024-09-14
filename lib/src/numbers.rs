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

/* Alternative method for converting integer to roman numeral, same constraints */
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

// the inner value is the value of the digit
#[derive(Eq, PartialEq, Hash, Clone)]
enum DecimalDigit {
    Thousands(u16),
    Hundreds(u16),
    Tens(u16),
    Units(u16)
}

// TODO: implement static map (probably by using crates.io)
fn build_numbers_conversion_map() -> HashMap::<(DecimalDigit, char), DecimalDigit> {
    let result = HashMap::from([
	((DecimalDigit::Thousands(0), 'M'), DecimalDigit::Thousands(1)),
	((DecimalDigit::Thousands(1), 'M'), DecimalDigit::Thousands(2)),
	((DecimalDigit::Thousands(2), 'M'), DecimalDigit::Thousands(3)),
	((DecimalDigit::Thousands(3), 'M'), DecimalDigit::Thousands(4)),
	((DecimalDigit::Thousands(0), 'D'), DecimalDigit::Hundreds(5)),
	((DecimalDigit::Thousands(1), 'D'), DecimalDigit::Hundreds(5)),
	((DecimalDigit::Thousands(2), 'D'), DecimalDigit::Hundreds(5)),
	((DecimalDigit::Thousands(3), 'D'), DecimalDigit::Hundreds(5)),
	((DecimalDigit::Thousands(4), 'D'), DecimalDigit::Hundreds(5)),
	((DecimalDigit::Thousands(0), 'C'), DecimalDigit::Hundreds(1)),
	((DecimalDigit::Thousands(1), 'C'), DecimalDigit::Hundreds(1)),
	((DecimalDigit::Thousands(2), 'C'), DecimalDigit::Hundreds(1)),
	((DecimalDigit::Thousands(3), 'C'), DecimalDigit::Hundreds(1)),
	((DecimalDigit::Thousands(4), 'C'), DecimalDigit::Hundreds(1)),
	((DecimalDigit::Thousands(0), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Thousands(1), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Thousands(2), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Thousands(3), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Thousands(4), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Thousands(0), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Thousands(1), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Thousands(2), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Thousands(3), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Thousands(4), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Thousands(0), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Thousands(1), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Thousands(2), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Thousands(3), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Thousands(4), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Thousands(0), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Thousands(1), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Thousands(2), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Thousands(3), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Thousands(4), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(1), 'C'), DecimalDigit::Hundreds(2)),
	((DecimalDigit::Hundreds(2), 'C'), DecimalDigit::Hundreds(3)),
	((DecimalDigit::Hundreds(1), 'D'), DecimalDigit::Hundreds(4)),
	((DecimalDigit::Hundreds(5), 'C'), DecimalDigit::Hundreds(6)),
	((DecimalDigit::Hundreds(6), 'C'), DecimalDigit::Hundreds(7)),
	((DecimalDigit::Hundreds(7), 'C'), DecimalDigit::Hundreds(8)),
	((DecimalDigit::Hundreds(1), 'M'), DecimalDigit::Hundreds(9)),
	((DecimalDigit::Hundreds(1), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(2), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(3), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(4), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(5), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(6), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(7), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(8), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(9), 'L'), DecimalDigit::Tens(5)),
	((DecimalDigit::Hundreds(1), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(2), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(3), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(4), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(5), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(6), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(7), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(8), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(9), 'X'), DecimalDigit::Tens(1)),
	((DecimalDigit::Hundreds(1), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(2), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(3), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(4), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(5), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(6), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(7), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(8), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(9), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Hundreds(1), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(2), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(3), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(4), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(5), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(6), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(7), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(8), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Hundreds(9), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(1), 'X'), DecimalDigit::Tens(2)),
	((DecimalDigit::Tens(2), 'X'), DecimalDigit::Tens(3)),
	((DecimalDigit::Tens(1), 'L'), DecimalDigit::Tens(4)),
	((DecimalDigit::Tens(5), 'X'), DecimalDigit::Tens(6)),
	((DecimalDigit::Tens(6), 'X'), DecimalDigit::Tens(7)),
	((DecimalDigit::Tens(7), 'X'), DecimalDigit::Tens(8)),
	((DecimalDigit::Tens(1), 'C'), DecimalDigit::Tens(9)),
	((DecimalDigit::Tens(1), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(2), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(3), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(4), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(5), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(6), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(7), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(8), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(9), 'V'), DecimalDigit::Units(5)),
	((DecimalDigit::Tens(1), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(2), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(3), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(4), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(5), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(6), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(7), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(8), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Tens(9), 'I'), DecimalDigit::Units(1)),
	((DecimalDigit::Units(1), 'I'), DecimalDigit::Units(2)),
	((DecimalDigit::Units(2), 'I'), DecimalDigit::Units(3)),
	((DecimalDigit::Units(1), 'V'), DecimalDigit::Units(4)),
	((DecimalDigit::Units(5), 'I'), DecimalDigit::Units(6)),
	((DecimalDigit::Units(6), 'I'), DecimalDigit::Units(7)),
	((DecimalDigit::Units(7), 'I'), DecimalDigit::Units(8)),
	((DecimalDigit::Units(1), 'X'), DecimalDigit::Units(9))
    ]);

    result

}

/*
This is the maximum number of digits that the resulting number (converted from roman numeral) can have.
If the number has less digits the missing ones are considered to have value 0.
*/
const MAX_DIGITS_COUNT: usize = 4;

/*
This function converts a roman numeral into the concrete numeric value.
Only roman numerals mapping to integer values between 0 and 5000 (ends excluded) are accepted.
*/
pub fn convert_roman_numeral_to_number(numeral: &Vec::<char>) -> u16 {
    fn update_digit(digit: &DecimalDigit, resulting_digit_values: &mut [u16;4]) {
	match digit {
	    DecimalDigit::Thousands(thousands_value) => resulting_digit_values[0] = *thousands_value,
	    DecimalDigit::Hundreds(hundreds_value) => resulting_digit_values[1] = *hundreds_value,
	    DecimalDigit::Tens(tens_value) => resulting_digit_values[2] = *tens_value,
	    DecimalDigit::Units(units_value) => resulting_digit_values[3] = *units_value
	}
    }

    let mut result = 0;

    if numeral.len() > 0 {
	let numbers_conversion_map = build_numbers_conversion_map();
	let mut resulting_digit_values = [0;MAX_DIGITS_COUNT];
	let mut current_digit = DecimalDigit::Thousands(0);
	let mut is_valid_numeral = true;

	for ch in numeral.iter() {
	    let mut current_char = *ch;
	    utilities::convert_char_to_uppercase(&mut current_char);
	    let key = (current_digit.clone(), current_char);

	    match numbers_conversion_map.get(&key) {
		Some(digit) => {
		    current_digit = digit.clone();
		    update_digit(&current_digit, &mut resulting_digit_values);
		},
		None => {
		    is_valid_numeral = false;
		    break;
		}
	    }
	}

	if is_valid_numeral {
	    for index in 0..MAX_DIGITS_COUNT {
		result += u16::pow(10, (MAX_DIGITS_COUNT - 1 - index) as u32) * resulting_digit_values[index];
	    }
	}
    }

    return result;
}
