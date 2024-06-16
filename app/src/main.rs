extern crate learn_rust_lib;
use learn_rust_lib::numbers::reverse_int;

fn handle_reverse_int_case(label:&str, number_to_reverse:(u64, u8)) {
   let reversed_number = reverse_int(number_to_reverse);
   println!("Case {}: input is {:?}, output is {:?}", label, number_to_reverse, reversed_number);
}

fn main() {
   println!("Welcome to the LearningRUST repo!");
   println!("First function: reverse_int(), argument is the number and the preceding zeroes");

   handle_reverse_int_case("1a", (12500, 3));
   handle_reverse_int_case("1b", (521000, 2));
   handle_reverse_int_case("2a", (12500, 0));
   handle_reverse_int_case("2b", (521, 2));
   handle_reverse_int_case("3a", (5210245762, 0));
   handle_reverse_int_case("3b", (2675420125, 0));
   handle_reverse_int_case("4", (24200, 2));
   handle_reverse_int_case("5a", (0, 2));
   handle_reverse_int_case("5b", (0, 0));
   handle_reverse_int_case("6a", (3, 2));
   handle_reverse_int_case("6b", (300, 0));
   handle_reverse_int_case("7", (3, 0));
}
