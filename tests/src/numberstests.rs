#[cfg(test)]
use learn_rust_lib::numbers;

#[test]
pub fn test_reverse_int() {
    assert_eq!(numbers::reverse_int((12500, 3)), (521000, 2));
    assert_eq!(numbers::reverse_int((521000, 2)), (12500, 3));
    assert_eq!(numbers::reverse_int((12500, 0)), (521, 2));
    assert_eq!(numbers::reverse_int((521, 2)), (12500, 0));
    assert_eq!(numbers::reverse_int((5210245762, 0)), (2675420125, 0));
    assert_eq!(numbers::reverse_int((2675420125, 0)), (5210245762, 0));
    assert_eq!(numbers::reverse_int((24200, 2)), (24200, 2));
    assert_eq!(numbers::reverse_int((0, 2)), (0, 2));
    assert_eq!(numbers::reverse_int((0, 0)), (0, 0));
    assert_eq!(numbers::reverse_int((3, 2)), (300, 0));
    assert_eq!(numbers::reverse_int((300, 0)), (3, 2));
    assert_eq!(numbers::reverse_int((3, 0)), (3, 0));
    assert_eq!(numbers::reverse_int((00, 0)), (0, 0));
    assert_eq!(numbers::reverse_int((0, 00)), (0, 0));
    assert_eq!(numbers::reverse_int((03, 02)), (300, 0));
}

#[test]
pub fn test_median() {
    assert!(numbers::compute_median(&vec![-2, 10, 5, 8, -3, -3, 7, 11, 9, 1]) == Some(6));
    assert!(numbers::compute_median(&vec![-2, 10, 8, 11, -3, 7, 8, -3, 9, 1]) == Some(7));
    assert!(numbers::compute_median(&vec![-1, 4, 5, 10, 0, -2, 2, -3, 4]) == Some(2));
    assert!(numbers::compute_median(&vec![10, 4, 0, -1, -3, -2, 5, 4, -1]) == Some(0));
    assert!(numbers::compute_median(&vec![-3, 5, 4, 10, 0, -4, -1, -2, -1]) == Some(-1));
    assert!(numbers::compute_median(&vec![-3, 2, -3, 4, 7, 2, 2, 9]) == Some(2));
    assert!(numbers::compute_median(&vec![8, 8, 8, 8, 8, 8]) == Some(8));
    assert!(numbers::compute_median(&vec![-2, 1, 1, -2, -2, 1]) == Some(0));
    assert!(numbers::compute_median(&vec![-4, 5, 2, 1, 0]) == Some(1));
    assert!(numbers::compute_median(&vec![-4, -4, -4, -4, -4]) == Some(-4));
    assert!(numbers::compute_median(&vec![10, 8, 9, 7]) == Some(8));
    assert!(numbers::compute_median(&vec![4, -2, 5, 2]) == Some(3));
    assert!(numbers::compute_median(&vec![-3, 1, 3, -1]) == Some(0));
    assert!(numbers::compute_median(&vec![5, 9, -2]) == Some(5));
    assert!(numbers::compute_median(&vec![5, 1, -2]) == Some(1));
    assert!(numbers::compute_median(&vec![5, -2]) == Some(1));
    assert!(numbers::compute_median(&vec![4, -2]) == Some(1));
    assert!(numbers::compute_median(&vec![3, -2]) == Some(0));
    assert!(numbers::compute_median(&vec![2, -2]) == Some(0));
    assert!(numbers::compute_median(&vec![1, -2]) == Some(0));
    assert!(numbers::compute_median(&vec![0, -2]) == Some(-1));
    assert!(numbers::compute_median(&vec![5]) == Some(5));
    assert!(numbers::compute_median(&Vec::<i32>::new()) == None);
}

#[test]
pub fn test_mode() {
    assert_eq!(numbers::compute_mode(&vec![-3, 2, -3, 4, 7, 2, 2, 9]), (3, vec![2]));
    assert_eq!(numbers::compute_mode(&vec![-2, 1, 1, -2, -2, 1]), (3, vec![-2, 1]));
    assert_eq!(numbers::compute_mode(&vec![-4, 3, 2, -4, 2, 3, 2, 3, -4, 3, -4, 2]), (4, vec![-4, 2, 3]));
    assert_eq!(numbers::compute_mode(&vec![-4, 3, -5, 2, 5, 5, -4, 2, 3, 7, 2, 3, 7, -4, 3, 0, -4, 2]), (4, vec![-4, 2, 3]));
    assert_eq!(numbers::compute_mode(&vec![7, 8, 5, 8, 7, 6, 8, 8, 7, 3, -2, 8, 4, 0, -4]), (5, vec![8]));
    assert_eq!(numbers::compute_mode(&vec![7, 8, 4, 5, 8, 4, 7, 6, 8, 4, 8, 7, 3, -2, 8, 7, 4, 0, 7, -4]), (5, vec![7, 8]));
    assert_eq!(numbers::compute_mode(&vec![5, -1, 4, 5, 3, 5, -2, 9, 5, 5, -7, 8, 5, 0, 5, 5]), (8, vec![5]));
    assert_eq!(numbers::compute_mode(&vec![-8, 9, -3, 2, 9, 9, -8, 0, -3, -3, 2, 9, 2, 9, -3]), (5, vec![9]));
    assert_eq!(numbers::compute_mode(&vec![-2, 10, 4, 5, 9, 7, -4, 0, 1]), (1, vec![-4, -2, 0, 1, 4, 5, 7, 9, 10]));
    assert_eq!(numbers::compute_mode(&vec![-2, 10, -4, 4, 5, 9, 7, -4, 0, 1]), (2, vec![-4]));
    assert_eq!(numbers::compute_mode(&vec![-1, 2, -1]), (2, vec![-1]));
    assert_eq!(numbers::compute_mode(&vec![2, -1]), (1, vec![-1, 2]));
    assert_eq!(numbers::compute_mode(&vec![1]), (1, vec![1]));
    assert_eq!(numbers::compute_mode(&Vec::<i32>::new()), (0, Vec::<i32>::new()));
}
