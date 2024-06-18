extern crate learn_rust_lib;
use learn_rust_lib::numbers::reverse_int;

fn handle_reverse_int_case(label:&str, number_to_reverse:(u64, u8)) {
   let reversed_number = reverse_int(number_to_reverse);
   println!("Case {}: input is {:?}, output is {:?}", label, number_to_reverse, reversed_number);
}

fn main() {
   println!("");
   println!("Welcome to the LearningRUST repo!");
   println!("First function: reverse_int(), argument is the number and the preceding zeroes...");
   println!("");

   handle_reverse_int_case("1", (12500, 3));
   handle_reverse_int_case("2 (vice-versa)", (521000, 2));

   println!("");
   println!("For more usage scenarios of this function please enter the tests module and run \'cargo test\'.");
   println!("Thank you!");
}
