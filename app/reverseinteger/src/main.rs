extern crate learn_rust_lib;
use learn_rust_lib::{numbers::reverse_int, utilities};

fn read_input() -> String {
    let mut input_str = String::new();
    utilities::read_line(&mut input_str).unwrap();
    pre_parse_input_string(&mut input_str);
    input_str
}

fn pre_parse_input_string(input_str: &mut String) {
    let temp = input_str.trim().to_string();
    *input_str = temp;
}

fn parse_input_string(input_str: &String) -> Option<(u64, u8)> {
    if input_str.len() == 0 {
        panic!("A non-empty input string is expected!");
    }

    let mut number_str = String::new();
    let mut preceding_zeroes_count = 0;
    let mut preceding_zeroes_check_required = true;
    let mut is_valid_string = true;

    for ch in input_str.chars() {
        if !ch.is_digit(10) {
            is_valid_string = false;
            break;
        }

        if preceding_zeroes_check_required {
            if ch == '0' {
                preceding_zeroes_count += 1;
                continue;
            }
            number_str.push(ch);
            preceding_zeroes_check_required = false;
        } else {
            number_str.push(ch);
        }
    }

    let mut result: Option<(u64, u8)> = Option::None;

    if is_valid_string {
        let mut number_int: u64 = 0;
        if number_str.len() == 0 {
            preceding_zeroes_count -= 1;
        } else {
            number_int = number_str.parse::<u64>().unwrap();
        }
        result = Option::Some((number_int, preceding_zeroes_count));
    }

    result
}

fn convert_prefixed_int_to_string(number: (u64, u8)) -> String {
    let mut result = String::new();
    let (concrete_number, preceding_zeroes_count) = number;

    if preceding_zeroes_count > 0 {
        result = format!(
            "({one_zero_digit:0>zeroes_count_to_prepend$})",
            one_zero_digit = 0,
            zeroes_count_to_prepend = preceding_zeroes_count as usize
        );
    }

    result.push_str(concrete_number.to_string().as_str());
    result
}

fn main() {
    let max_digits_count = 19; // this is to prevent overflow (a 64bit integer has maximum 20 digits) - preceding zeroes included

    utilities::clear_screen();
    println!("Welcome!");
    println!("---------------------------------------------------------------------------------");

    loop {
        println!("Please enter a positive integer (press ENTER to exit)!");

        let input_str = read_input();
        let resulting_size = input_str.len();

        utilities::clear_screen();

        if resulting_size == 0 {
            println!("Goodbye!");
            break;
        }

        if resulting_size <= max_digits_count {
            let result = parse_input_string(&input_str);
            if result == Option::None {
                println!("Invalid integer! Please try again.")
            } else {
                let unwrapped_result = result.unwrap();
                let reversed_result = reverse_int(unwrapped_result);
                println!(
                    "Reversed integer is: {}",
                    convert_prefixed_int_to_string(reversed_result)
                );
            }
        } else {
            println!(
                "The maximum allowed number of digits ({}) has been exceeded! Please try again.",
                max_digits_count
            );
        }

        println!(
            "---------------------------------------------------------------------------------"
        );
    }
}
