#[cfg(test)]
use learn_rust_lib::numbers::reverse_int;

#[test]
pub fn test_reverse_int() {
   assert_eq!(reverse_int((12500, 3)), (521000, 2));
   assert_eq!(reverse_int((521000, 2)), (12500, 3));
   assert_eq!(reverse_int((12500, 0)), (521, 2));
   assert_eq!(reverse_int((521, 2)), (12500, 0));
   assert_eq!(reverse_int((5210245762, 0)), (2675420125, 0));
   assert_eq!(reverse_int((2675420125, 0)), (5210245762, 0));
   assert_eq!(reverse_int((24200, 2)), (24200, 2));
   assert_eq!(reverse_int((0, 2)), (0, 2));
   assert_eq!(reverse_int((0, 0)), (0, 0));
   assert_eq!(reverse_int((3, 2)), (300, 0));
   assert_eq!(reverse_int((300, 0)), (3, 2));
   assert_eq!(reverse_int((3, 0)), (3, 0));
   assert_eq!(reverse_int((00, 0)), (0, 0));
   assert_eq!(reverse_int((0, 00)), (0, 0));
   assert_eq!(reverse_int((03, 02)), (300, 0));
}
