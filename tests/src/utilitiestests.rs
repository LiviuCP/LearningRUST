#[cfg(test)]
use learn_rust_lib::utilities;

#[test]
pub fn test_get_digits() {
    assert_eq!(utilities::get_digits(0), vec![0]);
    assert_eq!(utilities::get_digits(1), vec![1]);
    assert_eq!(utilities::get_digits(2), vec![2]);
    assert_eq!(utilities::get_digits(3), vec![3]);
    assert_eq!(utilities::get_digits(4), vec![4]);
    assert_eq!(utilities::get_digits(5), vec![5]);
    assert_eq!(utilities::get_digits(6), vec![6]);
    assert_eq!(utilities::get_digits(7), vec![7]);
    assert_eq!(utilities::get_digits(8), vec![8]);
    assert_eq!(utilities::get_digits(9), vec![9]);
    assert_eq!(utilities::get_digits(23), vec![2, 3]);
    assert_eq!(utilities::get_digits(412), vec![4, 1, 2]);
    assert_eq!(utilities::get_digits(6295), vec![6, 2, 9, 5]);
    assert_eq!(utilities::get_digits(20543219), vec![2, 0, 5, 4, 3, 2, 1, 9]);
}
