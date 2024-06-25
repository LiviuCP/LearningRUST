extern crate learn_rust_lib;
use learn_rust_lib::numbers::reverse_int;

fn pre_parse_input_string(number_as_str:&mut String) -> u8 {
   let temp = number_as_str.trim().to_string();
   *number_as_str = temp;

   return number_as_str.len() as u8;
}

fn parse_input_string(number_as_str:&String) -> Option<(u64,u8)> {
   let mut result:Option<(u64,u8)> = Option::Some((0, 0));
   let mut preceding_zeroes_count = 0;
   let mut preceding_zeroes_check_required = true;
   let mut number = String::new();
   for ch in number_as_str.chars(){
      if !ch.is_digit(10) {
         result = Option::None;
	 break;
      }
      else if preceding_zeroes_check_required {
         if ch == '0' {
	    preceding_zeroes_count += 1;
	 }
	 else {
	    number.push(ch);
	    preceding_zeroes_check_required = false;
	 }
      }
      else {
         number.push(ch);
      }
   }

   if result != Option::None {
      if preceding_zeroes_count == number_as_str.len() as u8 {
         result = Option::Some((0, preceding_zeroes_count - 1));
      }
      else {
         result = Option::Some((number.parse::<u64>().unwrap(), preceding_zeroes_count));
      }
   }

   return result;
}

fn convert_prefixed_int_to_string(number:(u64, u8)) -> String {
   let mut result = String::new();
   let mut zeroes_count = number.1;
   while zeroes_count > 0 {
      result.push('0');
      zeroes_count -= 1;
   }
   result.push_str(number.0.to_string().as_str());
   return result;
}

fn clear_screen() {
   std::process::Command::new("clear").status().unwrap();
}

fn main() {
   let max_digits_count = 19; // this is to prevent overflow (a 64bit integer has maximum 20 digits) - preceding zeroes included

   clear_screen();
   println!("Welcome!");
   println!("---------------------------------------------------------------------------------");

   loop {
      println!("Please enter a positive integer (press ENTER to exit)!");
      let mut number_as_str = String::new();
      std::io::stdin().read_line(&mut number_as_str).unwrap();
      clear_screen();
      let resulting_size = pre_parse_input_string(&mut number_as_str);

      if resulting_size == 0 {
         println!("Goodbye!");
	 break;
      }
      else if resulting_size <= max_digits_count {
         let result = parse_input_string(&number_as_str);
         if result == Option::None {
            println!("Invalid integer! Please try again.")
         }
         else
         {
             let unwrapped_result = result.unwrap();
	     let reversed_result = reverse_int(unwrapped_result);
	     println!("Reversed integer is: {}", convert_prefixed_int_to_string(reversed_result));
         }
      }
      else {
         println!("The maximum allowed number of digits ({}) has been exceeded! Please try again.", max_digits_count);
      }

      println!("---------------------------------------------------------------------------------");
   }
}
