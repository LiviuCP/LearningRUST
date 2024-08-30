#[cfg(test)]
use learn_rust_lib::numbers::reverse_int;
use learn_rust_lib::numbers::compute_median;

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

#[test]
pub fn test_median() {
    assert!(compute_median(&vec![-2, 10, 5, 8, -3, -3, 7, 11, 9, 1]) == Some(6));
    assert!(compute_median(&vec![-2, 10, 8, 11, -3, 7, 8, -3, 9, 1]) == Some(7));
    assert!(compute_median(&vec![-1, 4, 5, 10, 0, -2, 2, -3, 4]) == Some(2));
    assert!(compute_median(&vec![10, 4, 0, -1, -3, -2, 5, 4, -1]) == Some(0));
    assert!(compute_median(&vec![-3, 5, 4, 10, 0, -4, -1, -2, -1]) == Some(-1));
    assert!(compute_median(&vec![-3, 2, -3, 4, 7, 2, 2, 9]) == Some(2));
    assert!(compute_median(&vec![8, 8, 8, 8, 8, 8]) == Some(8));
    assert!(compute_median(&vec![-2, 1, 1, -2, -2, 1]) == Some(0));
    assert!(compute_median(&vec![-4, 5, 2, 1, 0]) == Some(1));
    assert!(compute_median(&vec![-4, -4, -4, -4, -4]) == Some(-4));
    assert!(compute_median(&vec![10, 8, 9, 7]) == Some(8));
    assert!(compute_median(&vec![4, -2, 5, 2]) == Some(3));
    assert!(compute_median(&vec![-3, 1, 3, -1]) == Some(0));
    assert!(compute_median(&vec![5, 9, -2]) == Some(5));
    assert!(compute_median(&vec![5, 1, -2]) == Some(1));
    assert!(compute_median(&vec![5, -2]) == Some(1));
    assert!(compute_median(&vec![4, -2]) == Some(1));
    assert!(compute_median(&vec![3, -2]) == Some(0));
    assert!(compute_median(&vec![2, -2]) == Some(0));
    assert!(compute_median(&vec![1, -2]) == Some(0));
    assert!(compute_median(&vec![0, -2]) == Some(-1));
    assert!(compute_median(&vec![5]) == Some(5));
    assert!(compute_median(&Vec::<i32>::new()) == None);
}
