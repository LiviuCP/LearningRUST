use std::collections::HashMap;
use crate::utilities;
use phf::phf_map;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M
}

impl RomanDigit {
    pub fn as_char(&self) -> char {
	match *self {
	    RomanDigit::I => 'I',
	    RomanDigit::V => 'V',
	    RomanDigit::X => 'X',
	    RomanDigit::L => 'L',
	    RomanDigit::C => 'C',
	    RomanDigit::D => 'D',
	    RomanDigit::M => 'M'
	}
    }

    pub fn from_char(ch: char) -> Option<Self> {
	match ch {
	    'i' | 'I' => Some(RomanDigit::I),
	    'v' | 'V' => Some(RomanDigit::V),
	    'x' | 'X' => Some(RomanDigit::X),
	    'l' | 'L' => Some(RomanDigit::L),
	    'c' | 'C' => Some(RomanDigit::C),
	    'd' | 'D' => Some(RomanDigit::D),
	    'm' | 'M' => Some(RomanDigit::M),
	    _ => None
	}
    }
}

pub struct RomanNumeral {
    content: Vec::<RomanDigit>
}

impl RomanNumeral {
    const MAX_CHARS_COUNT: usize = 16;

    pub fn is_valid_roman_numeral_string(numeral_str: &str) -> bool {
	let mut is_valid = false;
	let numeral_size = numeral_str.len();

	if numeral_size > 0 && numeral_size <= Self::MAX_CHARS_COUNT {
	    let roman_numeral_regex = Regex::new("^M{0,4}(DC{0,3}|C[DM]|C{1,3}){0,1}(LX{0,3}|X[LC]|X{1,3}){0,1}(VI{0,3}|I[VX]|I{1,3}){0,1}$").unwrap();
	    is_valid = roman_numeral_regex.is_match(&numeral_str.to_uppercase());
	}

	return is_valid;
    }

    pub fn from_string(numeral_str: &str) -> Self {
	let mut result = Self {content: Vec::new()};

	if Self::is_valid_roman_numeral_string(numeral_str) {
	    for ch in numeral_str.chars() {
		if let Some(roman_digit) = RomanDigit::from_char(ch) {
		    result.content.push(roman_digit);
		    continue;
		}

		result.content.clear();
		panic!("Invalid roman digit detected! (should have been filtered out by regex)");
	    }
	}

	result
    }

    pub fn to_string(&self) -> String {
	let mut result = String::new();

	for roman_digit in self.content.iter() {
	    result.push(roman_digit.as_char());
	}

	result
    }

    pub fn get_content(&self) -> &Vec::<RomanDigit> {
	&self.content
    }

    pub fn empty(&self) -> bool {
	self.content.len() == 0
    }
}

pub struct NumberToRomanNumeralConverter {
    remainder: u16,
    result: Vec::<char>
}

impl NumberToRomanNumeralConverter {
    const MIN_NUMBER_TO_CONVERT: u16 = 1;
    const MAX_NUMBER_TO_CONVERT: u16 = 4999;

    pub fn create() -> Self {
	Self{remainder: 0, result: Vec::new()}
    }

    /*
    This method converts a number between 1 and 4999 to a roman numeral by using multiple integer thresholds (along with (modulo) division operations)
    for obtaining the output (roman) chars
     */
    pub fn convert(&mut self, number: u16) -> &Vec::<char> {
	self.reset();

	if Self::is_within_valid_range(number) {
	    self.remainder = number;
	    self.handle_same_appended_char_threshold(1000, 'M');

	    // mutually exclusive: 900 - 999, 500 - 899, 400 - 499
	    let handled = self.handle_different_appended_chars_threshold(900, &mut vec!['C','M']);

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(500, 'D');

		if !handled {
		    self.handle_different_appended_chars_threshold(400, &mut vec!['C','D']);
		}
	    }

	    self.handle_same_appended_char_threshold(100, 'C');

	    // mutually exclusive: 90 - 99, 50 - 89, 40 - 49
	    let handled = self.handle_different_appended_chars_threshold(90, &mut vec!['X','C']);

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(50, 'L');

		if !handled {
		    self.handle_different_appended_chars_threshold(40, &mut vec!['X','L']);
		}
	    }

	    self.handle_same_appended_char_threshold(10, 'X');

	    // mutually exclusive: 9, 5 - 8, 4
	    let handled = self.handle_different_appended_chars_threshold(9, &mut vec!['I','X']);

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(5, 'V');

		if !handled {
		    self.handle_different_appended_chars_threshold(4, &mut vec!['I','V']);
		}
	    }

	    self.handle_same_appended_char_threshold(1, 'I');
	}

	&self.result
    }

    fn handle_same_appended_char_threshold(&mut self, threshold: u16, char_to_append: char) -> bool {
	let can_handle = self.remainder >= threshold;

	if can_handle {
	    utilities::push_multiple_times_to_vec(&char_to_append, (self.remainder / threshold) as usize, &mut self.result);
	    self.remainder = self.remainder % threshold;
	}

	can_handle
    }

    // multiple different chars, appended one time each for the given threshold (if it applies: remainder >= threshold)
    fn handle_different_appended_chars_threshold(&mut self, threshold: u16, chars_to_append: &mut Vec::<char>) -> bool {
	let can_handle = self.remainder >= threshold;

	if can_handle {
	    self.result.append(chars_to_append);
	    self.remainder = self.remainder % threshold;
	}

	can_handle
    }

    fn reset(&mut self) {
	self.remainder = 0;
	self.result.clear();
    }

    fn is_within_valid_range(number: u16) -> bool {
	number >= Self::MIN_NUMBER_TO_CONVERT && number <= Self::MAX_NUMBER_TO_CONVERT
    }
}

/*
Key is an array consisting of two elements:
- digit power-of-ten index (e.g. for hundreds digit it is 2, i.e. 10^2 = 100)
- digit value

A tuple would have been more appropriate, yet this is not supported as key by the phf compile-time map.
*/
static ROMAN_NUMERALS_CONVERSION_MAP: phf::Map<[u8;2], &'static str> = phf_map! {
    [0, 1] => "I",
    [0, 2] => "II",
    [0, 3] => "III",
    [0, 4] => "IV",
    [0, 5] => "V",
    [0, 6] => "VI",
    [0, 7] => "VII",
    [0, 8] => "VIII",
    [0, 9] => "IX",
    [1, 1] => "X",
    [1, 2] => "XX",
    [1, 3] => "XXX",
    [1, 4] => "XL",
    [1, 5] => "L",
    [1, 6] => "LX",
    [1, 7] => "LXX",
    [1, 8] => "LXXX",
    [1, 9] => "XC",
    [2, 1] => "C",
    [2, 2] => "CC",
    [2, 3] => "CCC",
    [2, 4] => "CD",
    [2, 5] => "D",
    [2, 6] => "DC",
    [2, 7] => "DCC",
    [2, 8] => "DCCC",
    [2, 9] => "CM",
    [3, 1] => "M",
    [3, 2] => "MM",
    [3, 3] => "MMM",
    [3, 4] => "MMMM"
};

/* Alternative method for converting integer to roman numeral, same constraints */
pub fn convert_number_to_roman_numeral_using_hash(number: u16) -> Vec::<char> {
    let mut result = Vec::new();

    if number > 0 && number < 5000 {
	let digits = utilities::get_digits(number as u32);

	// no need to check the digits_count is > 0 for getting power_of_ten_index (see below), an integer should have min. 1 digit
	let digits_count = digits.len();

	for (index, digit) in digits.iter().enumerate() {
	    let power_of_ten_index = (digits_count - 1 - index) as u8; // place of the digit within the number, e.g. for thousands it is 3 corresponding to 10^3
	    let chars_to_append: &str = ROMAN_NUMERALS_CONVERSION_MAP.get(&[power_of_ten_index, *digit]).unwrap_or(&"");
	    let mut chars_to_append: Vec::<char> = (*chars_to_append).chars().collect();
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

pub struct RomanNumeralToNumberConverter {
    resulting_digit_values: [u16;Self::MAX_DIGITS_COUNT],
    conversion_map: HashMap::<(DecimalDigit, char), DecimalDigit>
}

impl RomanNumeralToNumberConverter {
    /*
    This is the maximum number of digits that the resulting number (converted from roman numeral) can have.
    If the number has less digits the missing ones are considered to have value 0.
     */
    const MAX_DIGITS_COUNT: usize = 4;

    pub fn create() -> Self {
	Self{resulting_digit_values: [0;Self::MAX_DIGITS_COUNT], conversion_map: Self::build_numbers_conversion_map()}
    }

    /*
    This function converts a roman numeral into the concrete numeric value.
    Only roman numerals mapping to integer values between 0 and 5000 (ends excluded) are accepted.
     */
    pub fn convert(&mut self, numeral: &Vec::<char>) -> u16 {
	self.reset();
	let mut result = 0;

	if numeral.len() > 0 {
	    let mut current_digit = DecimalDigit::Thousands(0);
	    let mut is_valid_numeral = true;

	    for ch in numeral.iter() {
		let mut current_char = *ch;
		utilities::convert_char_to_uppercase(&mut current_char);
		let key = (current_digit.clone(), current_char);

		match self.conversion_map.get(&key) {
		    Some(digit) => {
			current_digit = digit.clone();
			self.update_digit(&current_digit);
		    },
		    None => {
			is_valid_numeral = false;
			break;
		    }
		}
	    }

	    if is_valid_numeral {
		for index in 0..Self::MAX_DIGITS_COUNT {
		    result += u16::pow(10, (Self::MAX_DIGITS_COUNT - 1 - index) as u32) * self.resulting_digit_values[index];
		}
	    }
	    else {
		self.reset();
	    }
	}

	return result;
    }

    fn update_digit(&mut self, digit: &DecimalDigit) {
	match digit {
	    DecimalDigit::Thousands(thousands_value) => self.resulting_digit_values[0] = *thousands_value,
	    DecimalDigit::Hundreds(hundreds_value) => self.resulting_digit_values[1] = *hundreds_value,
	    DecimalDigit::Tens(tens_value) => self.resulting_digit_values[2] = *tens_value,
	    DecimalDigit::Units(units_value) => self.resulting_digit_values[3] = *units_value
	}
    }

    fn reset(&mut self) {
	self.resulting_digit_values = [0;Self::MAX_DIGITS_COUNT];
    }

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
}
