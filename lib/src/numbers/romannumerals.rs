use std::{fmt, str::FromStr, convert::{TryFrom, Into}, collections::HashMap};
use crate::utilities;
use phf::phf_map;
use regex::Regex;

/* A more general error, might occur when:
- the roman numeral string has invalid characters
- the roman numeral build rules are not followed (roman numeral format is not valid)
*/
#[derive(PartialEq, Debug)]
pub struct ParseRomanNumeralStringError;

/* A more specific error, occurs when the roman numeral build rules are not followed (e.g. numeral "IVI" is not valid) */
#[derive(PartialEq, Debug)]
pub struct ParseRomanNumeralDigitsError;

/* A more specific error, occurs when the integer to be converted into roman numeral exceeds the maximum allowed value
   (for this library this is currently 4999)
*/
#[derive(PartialEq, Debug)]
pub struct MaxNumberExceededError;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
pub struct RomanNumeral {
    content: Vec::<RomanDigit>
}

impl FromStr for RomanNumeral {
    type Err = ParseRomanNumeralStringError;

    fn from_str(numeral_str: &str) -> Result<Self, Self::Err> {
	let mut result = Err(ParseRomanNumeralStringError);

	if Self::is_valid_roman_numeral_string(numeral_str) {
	    let mut numeral = Self::create();

	    for ch in numeral_str.chars() {
		if let Some(roman_digit) = RomanDigit::from_char(ch) {
		    numeral.content.push(roman_digit);
		    continue;
		}

		numeral.content.clear();
		panic!("Invalid roman digit detected! (should have been filtered out by regex)");
	    }

	    result = Ok(numeral);
	}
	else if numeral_str.is_empty() {
	    result = Ok(RomanNumeral::create());
	}

	result
    }
}

// provides display AND ToStr trait (for ToStr: see tests)
impl fmt::Display for RomanNumeral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	let numeral_str = self.content.iter().map(|roman_digit| roman_digit.as_char()).collect::<String>();
	write!(f, "{}", numeral_str)
    }
}

// provides TryInto as well (see tests)
impl TryFrom<Vec<RomanDigit>> for RomanNumeral {
    type Error = ParseRomanNumeralDigitsError;

    fn try_from(digits: Vec<RomanDigit>) -> Result<Self, Self::Error> {
	let mut result = Ok(RomanNumeral::create());

	if !digits.is_empty() {
	    let numeral_str: String = digits.iter().map(|roman_digit| roman_digit.as_char()).collect();
	    let numeral = Self::from_string(&numeral_str);
	    result = if !numeral.empty() {Ok(numeral)} else {Err(ParseRomanNumeralDigitsError)};
	}

	result
    }
}

impl Into<Vec<RomanDigit>> for RomanNumeral {
    fn into(self) -> Vec<RomanDigit> {
	return self.content
    }
}

impl RomanNumeral {
    const MAX_CHARS_COUNT: usize = 16;

    pub fn create() -> Self {
	RomanNumeral{content: Vec::new()}
    }

    pub fn is_valid_roman_numeral_string(numeral_str: &str) -> bool {
	let mut is_valid = false;
	let numeral_size = numeral_str.len();

	if numeral_size > 0 && numeral_size <= Self::MAX_CHARS_COUNT {
	    let roman_numeral_regex = Regex::new("^M{0,4}(DC{0,3}|C[DM]|C{1,3}){0,1}(LX{0,3}|X[LC]|X{1,3}){0,1}(VI{0,3}|I[VX]|I{1,3}){0,1}$").unwrap();
	    is_valid = roman_numeral_regex.is_match(&numeral_str.to_uppercase());
	}

	return is_valid;
    }

    pub fn get_content(&self) -> &Vec::<RomanDigit> {
	&self.content
    }

    pub fn clear(&mut self) {
	self.content.clear();
    }

    pub fn empty(&self) -> bool {
	self.content.len() == 0
    }

    /* error-free version of the from_str() method
    - the string parsing error is turned into empty numeral
    - the caller should interpret the empty result accordingly in relation to string input / should capture specific errors in advance (before calling this method)
    - in other words this method should be called only "when the caller is sure" that the passed input is the "ok" one from requirements point of view
    */
    fn from_string(numeral_str: &str) -> Self {
	match Self::from_str(&numeral_str) {
	    Ok(numeral) => numeral,
	    Err(_) => Self::create()
	}
    }
}

pub struct NumberToRomanNumeralConverter {
    remainder: u16,
    result_str: String
}

impl NumberToRomanNumeralConverter {
    const MIN_NUMBER_TO_CONVERT: u16 = 1;
    const MAX_NUMBER_TO_CONVERT: u16 = 4999;

    pub fn create() -> Self {
	Self{remainder: 0, result_str: String::new()}
    }

    /*
    This method converts a number between 1 and 4999 to a roman numeral by using multiple integer thresholds (along with (modulo) division operations) for obtaining the output
    Special case: 0 (converted into empty numeral)
     */
    pub fn convert(&mut self, number: u16) -> Result<RomanNumeral, MaxNumberExceededError> {
	self.reset();

	let mut max_nr_exceeded = false;

	if Self::is_within_valid_range(number) {
	    self.remainder = number;
	    self.handle_same_appended_char_threshold(1000, 'M');

	    // mutually exclusive: 900 - 999, 500 - 899, 400 - 499
	    let handled = self.handle_different_appended_chars_threshold(900, "CM");

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(500, 'D');

		if !handled {
		    self.handle_different_appended_chars_threshold(400, "CD");
		}
	    }

	    self.handle_same_appended_char_threshold(100, 'C');

	    // mutually exclusive: 90 - 99, 50 - 89, 40 - 49
	    let handled = self.handle_different_appended_chars_threshold(90, "XC");

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(50, 'L');

		if !handled {
		    self.handle_different_appended_chars_threshold(40, "XL");
		}
	    }

	    self.handle_same_appended_char_threshold(10, 'X');

	    // mutually exclusive: 9, 5 - 8, 4
	    let handled = self.handle_different_appended_chars_threshold(9, "IX");

	    if !handled {
		let handled = self.handle_same_appended_char_threshold(5, 'V');

		if !handled {
		    self.handle_different_appended_chars_threshold(4, "IV");
		}
	    }

	    self.handle_same_appended_char_threshold(1, 'I');
	}
	else if number != 0 {
	    max_nr_exceeded = true;
	}

	// out-of-range is treated in different ways: 0 results in empty numeral while exceeding threshold is considered an error
	let result = if !max_nr_exceeded {Ok(RomanNumeral::from_string(&self.result_str))} else {Err(MaxNumberExceededError)};
	result
    }

    fn handle_same_appended_char_threshold(&mut self, threshold: u16, char_to_append: char) -> bool {
	let can_handle = self.remainder >= threshold;

	if can_handle {
	    let mut chars_to_append = Vec::<char>::new();
	    utilities::push_multiple_times_to_vec(&char_to_append, (self.remainder / threshold) as usize, &mut chars_to_append);
	    self.result_str.push_str(&chars_to_append.iter().collect::<String>());
	    self.remainder = self.remainder % threshold;
	}

	can_handle
    }

    // multiple different chars, appended one time each for the given threshold (if it applies: remainder >= threshold)
    fn handle_different_appended_chars_threshold(&mut self, threshold: u16, chars_to_append: &str) -> bool {
	let can_handle = self.remainder >= threshold;

	if can_handle {
	    self.result_str.push_str(chars_to_append);
	    self.remainder = self.remainder % threshold;
	}

	can_handle
    }

    fn reset(&mut self) {
	self.remainder = 0;
	self.result_str.clear();
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

/* Alternative method for converting integer to roman numeral, same constraints, same special case: 0 */
pub fn convert_number_to_roman_numeral_using_hash(number: u16) -> Result<RomanNumeral, MaxNumberExceededError> {
    let mut result_str = String::new();
    let mut max_nr_exceeded = false;

    if number > 0 && number < 5000 {
	let digits = utilities::get_digits(number as u32);

	// no need to check the digits_count is > 0 for getting power_of_ten_index (see below), an integer should have min. 1 digit
	let digits_count = digits.len();

	result_str = digits.iter().enumerate().map(|(index, digit)| {
	    let power_of_ten_index = (digits_count - 1 - index) as u8; // place of the digit within the number, e.g. for thousands it is 3 corresponding to 10^3
	    ROMAN_NUMERALS_CONVERSION_MAP.get(&[power_of_ten_index, *digit]).unwrap_or(&"").to_string()
	}).collect();
    }
    else if number != 0 {
	max_nr_exceeded = true;
    }

    // out-of-range is treated in different ways: 0 results in empty numeral while exceeding threshold is considered an error
    let result = if !max_nr_exceeded {Ok(RomanNumeral::from_string(&result_str))} else {Err(MaxNumberExceededError)};
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
    conversion_map: HashMap::<(DecimalDigit, RomanDigit), DecimalDigit>
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
    pub fn convert(&mut self, numeral: &RomanNumeral) -> u16 {
	self.reset();
	let mut result = 0;

	if !numeral.empty() {
	    let mut current_decimal_digit = DecimalDigit::Thousands(0);

	    for digit in numeral.get_content().iter() {
		let key = (current_decimal_digit.clone(), digit.clone());

		if let Some(digit) = self.conversion_map.get(&key) {
		    current_decimal_digit = digit.clone();
		    self.update_digit(&current_decimal_digit);
		}
		else {
		    panic!("Key not found in numbers conversion map!");
		}
	    }

	    result = (0..Self::MAX_DIGITS_COUNT).map(|index| u16::pow(10, (Self::MAX_DIGITS_COUNT - 1 - index) as u32) * self.resulting_digit_values[index]).sum()
	}

	result
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

    fn build_numbers_conversion_map() -> HashMap::<(DecimalDigit, RomanDigit), DecimalDigit> {
	let result = HashMap::from([
	    ((DecimalDigit::Thousands(0), RomanDigit::M), DecimalDigit::Thousands(1)),
	    ((DecimalDigit::Thousands(1), RomanDigit::M), DecimalDigit::Thousands(2)),
	    ((DecimalDigit::Thousands(2), RomanDigit::M), DecimalDigit::Thousands(3)),
	    ((DecimalDigit::Thousands(3), RomanDigit::M), DecimalDigit::Thousands(4)),
	    ((DecimalDigit::Thousands(0), RomanDigit::D), DecimalDigit::Hundreds(5)),
	    ((DecimalDigit::Thousands(1), RomanDigit::D), DecimalDigit::Hundreds(5)),
	    ((DecimalDigit::Thousands(2), RomanDigit::D), DecimalDigit::Hundreds(5)),
	    ((DecimalDigit::Thousands(3), RomanDigit::D), DecimalDigit::Hundreds(5)),
	    ((DecimalDigit::Thousands(4), RomanDigit::D), DecimalDigit::Hundreds(5)),
	    ((DecimalDigit::Thousands(0), RomanDigit::C), DecimalDigit::Hundreds(1)),
	    ((DecimalDigit::Thousands(1), RomanDigit::C), DecimalDigit::Hundreds(1)),
	    ((DecimalDigit::Thousands(2), RomanDigit::C), DecimalDigit::Hundreds(1)),
	    ((DecimalDigit::Thousands(3), RomanDigit::C), DecimalDigit::Hundreds(1)),
	    ((DecimalDigit::Thousands(4), RomanDigit::C), DecimalDigit::Hundreds(1)),
	    ((DecimalDigit::Thousands(0), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Thousands(1), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Thousands(2), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Thousands(3), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Thousands(4), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Thousands(0), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Thousands(1), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Thousands(2), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Thousands(3), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Thousands(4), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Thousands(0), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Thousands(1), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Thousands(2), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Thousands(3), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Thousands(4), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Thousands(0), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Thousands(1), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Thousands(2), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Thousands(3), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Thousands(4), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::C), DecimalDigit::Hundreds(2)),
	    ((DecimalDigit::Hundreds(2), RomanDigit::C), DecimalDigit::Hundreds(3)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::D), DecimalDigit::Hundreds(4)),
	    ((DecimalDigit::Hundreds(5), RomanDigit::C), DecimalDigit::Hundreds(6)),
	    ((DecimalDigit::Hundreds(6), RomanDigit::C), DecimalDigit::Hundreds(7)),
	    ((DecimalDigit::Hundreds(7), RomanDigit::C), DecimalDigit::Hundreds(8)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::M), DecimalDigit::Hundreds(9)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(2), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(3), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(4), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(5), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(6), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(7), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(8), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(9), RomanDigit::L), DecimalDigit::Tens(5)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(2), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(3), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(4), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(5), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(6), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(7), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(8), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(9), RomanDigit::X), DecimalDigit::Tens(1)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(2), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(3), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(4), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(5), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(6), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(7), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(8), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(9), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Hundreds(1), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(2), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(3), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(4), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(5), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(6), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(7), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(8), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Hundreds(9), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(1), RomanDigit::X), DecimalDigit::Tens(2)),
	    ((DecimalDigit::Tens(2), RomanDigit::X), DecimalDigit::Tens(3)),
	    ((DecimalDigit::Tens(1), RomanDigit::L), DecimalDigit::Tens(4)),
	    ((DecimalDigit::Tens(5), RomanDigit::X), DecimalDigit::Tens(6)),
	    ((DecimalDigit::Tens(6), RomanDigit::X), DecimalDigit::Tens(7)),
	    ((DecimalDigit::Tens(7), RomanDigit::X), DecimalDigit::Tens(8)),
	    ((DecimalDigit::Tens(1), RomanDigit::C), DecimalDigit::Tens(9)),
	    ((DecimalDigit::Tens(1), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(2), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(3), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(4), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(5), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(6), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(7), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(8), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(9), RomanDigit::V), DecimalDigit::Units(5)),
	    ((DecimalDigit::Tens(1), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(2), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(3), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(4), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(5), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(6), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(7), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(8), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Tens(9), RomanDigit::I), DecimalDigit::Units(1)),
	    ((DecimalDigit::Units(1), RomanDigit::I), DecimalDigit::Units(2)),
	    ((DecimalDigit::Units(2), RomanDigit::I), DecimalDigit::Units(3)),
	    ((DecimalDigit::Units(1), RomanDigit::V), DecimalDigit::Units(4)),
	    ((DecimalDigit::Units(5), RomanDigit::I), DecimalDigit::Units(6)),
	    ((DecimalDigit::Units(6), RomanDigit::I), DecimalDigit::Units(7)),
	    ((DecimalDigit::Units(7), RomanDigit::I), DecimalDigit::Units(8)),
	    ((DecimalDigit::Units(1), RomanDigit::X), DecimalDigit::Units(9))
	]);

	result
    }
}
