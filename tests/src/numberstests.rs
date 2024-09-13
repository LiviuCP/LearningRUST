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

#[test]
pub fn test_convert_number_to_roman_numeral() {
    // building blocks
    assert_eq!(numbers::convert_number_to_roman_numeral(1), vec!['I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4), vec!['I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(5), vec!['V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(9), vec!['I', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral(10), vec!['X']);
    assert_eq!(numbers::convert_number_to_roman_numeral(40), vec!['X', 'L']);
    assert_eq!(numbers::convert_number_to_roman_numeral(50), vec!['L']);
    assert_eq!(numbers::convert_number_to_roman_numeral(90), vec!['X', 'C']);
    assert_eq!(numbers::convert_number_to_roman_numeral(100), vec!['C']);
    assert_eq!(numbers::convert_number_to_roman_numeral(400), vec!['C', 'D']);
    assert_eq!(numbers::convert_number_to_roman_numeral(500), vec!['D']);
    assert_eq!(numbers::convert_number_to_roman_numeral(900), vec!['C', 'M']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1000), vec!['M']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4000), vec!['M', 'M', 'M', 'M']);

    // bounds and beyond bounds
    assert_eq!(numbers::convert_number_to_roman_numeral(0), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral(5000), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral(5001), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral(9875), Vec::<char>::new());

    // random
    assert_eq!(numbers::convert_number_to_roman_numeral(2), vec!['I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(8), vec!['V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(25), vec!['X', 'X', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(44), vec!['X', 'L', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(76), vec!['L', 'X', 'X', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(237), vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(412), vec!['C', 'D', 'X', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(555), vec!['D', 'L', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(777), vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1111), vec!['M', 'C', 'X', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1234), vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1453), vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1877), vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(1918), vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(2020), vec!['M', 'M', 'X', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral(2222), vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(2394), vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(2695), vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(2800), vec!['M', 'M', 'D', 'C', 'C', 'C']);
    assert_eq!(numbers::convert_number_to_roman_numeral(3000), vec!['M', 'M', 'M']);
    assert_eq!(numbers::convert_number_to_roman_numeral(3333), vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(3456), vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(3879), vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral(3987), vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4166), vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4444), vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4888), vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4987), vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral(4999), vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']);
}

#[test]
pub fn test_convert_number_to_roman_numeral_using_hash() {
    // building blocks
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1), vec!['I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4), vec!['I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(5), vec!['V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(9), vec!['I', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(10), vec!['X']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(40), vec!['X', 'L']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(50), vec!['L']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(90), vec!['X', 'C']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(100), vec!['C']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(400), vec!['C', 'D']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(500), vec!['D']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(900), vec!['C', 'M']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1000), vec!['M']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4000), vec!['M', 'M', 'M', 'M']);

    // bounds and beyond bounds
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(0), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(5000), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(5001), Vec::<char>::new());
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(9875), Vec::<char>::new());

    // random
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2), vec!['I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(8), vec!['V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(25), vec!['X', 'X', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(44), vec!['X', 'L', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(76), vec!['L', 'X', 'X', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(237), vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(412), vec!['C', 'D', 'X', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(555), vec!['D', 'L', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(777), vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1111), vec!['M', 'C', 'X', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1234), vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1453), vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1877), vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(1918), vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2020), vec!['M', 'M', 'X', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2222), vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2394), vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2695), vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(2800), vec!['M', 'M', 'D', 'C', 'C', 'C']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(3000), vec!['M', 'M', 'M']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(3333), vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(3456), vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(3879), vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(3987), vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4166), vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4444), vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4888), vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4987), vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']);
    assert_eq!(numbers::convert_number_to_roman_numeral_using_hash(4999), vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']);
}

#[test]
pub fn test_convert_roman_numeral_to_number() {
    // building blocks
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I']), 1);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'V']), 4);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['V']), 5);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'X']), 9);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X']), 10);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'L']), 40);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['L']), 50);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'C']), 90);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C']), 100);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'D']), 400);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D']), 500);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'M']), 900);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M']), 1000);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M']), 4000);

    // random
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'I']), 2);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['V', 'I', 'I', 'I']), 8);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'X', 'V']), 25);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'L', 'I', 'V']), 44);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['L', 'X', 'X', 'V', 'I']), 76);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'C', 'X', 'X', 'X', 'V', 'I', 'I']), 237);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'D', 'X', 'I', 'I']), 412);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D', 'L', 'V']), 555);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']), 777);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'X', 'I']), 1111);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'C', 'X', 'X', 'X', 'I', 'V']), 1234);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'D', 'L', 'I', 'I', 'I']), 1453);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'V', 'I', 'I']), 1877);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'M', 'X', 'V', 'I', 'I', 'I']), 1918);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'X', 'X']), 2020);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'C', 'C', 'X', 'X', 'I', 'I']), 2222);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'C', 'C', 'C', 'X', 'C', 'I', 'V']), 2394);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'D', 'C', 'X', 'C', 'V']), 2695);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'D', 'C', 'C', 'C']), 2800);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M']), 3000);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'C', 'C', 'C', 'X', 'X', 'X', 'I', 'I', 'I']), 3333);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'C', 'D', 'L', 'V', 'I']), 3456);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'I', 'X']), 3879);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']), 3987);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'L', 'X', 'V', 'I']), 4166);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'D', 'X', 'L', 'I', 'V']), 4444);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'D', 'C', 'C', 'C', 'L', 'X', 'X', 'X', 'V', 'I', 'I', 'I']), 4888);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'L', 'X', 'X', 'X', 'V', 'I', 'I']), 4987);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', 'I', 'X']), 4999);

    // lower or mixed case numerals
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'm', 'D', 'C', 'c', 'l', 'X', 'x', 'V', 'i', 'I']), 2777);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['d', 'L', 'v']), 555);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['m', 'd', 'c', 'l', 'x', 'v', 'i']), 1666);

    // error cases
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'M', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'D', 'C', 'C', 'C', 'C', 'X', 'V', 'I', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'L', 'X', 'X', 'X', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'V', 'I', 'I', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D', 'D', 'C', 'L']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['L', 'L', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'V', 'V', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D', 'M']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'L', 'M', 'V']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'M', 'V']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'V', 'M', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'M', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['L', 'C', 'X', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['V', 'C', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'C', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['V', 'X', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'L', 'D', 'X', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'X', 'D', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['V', 'D', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'D', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'V', 'L', 'I', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['C', 'C', 'I', 'L', 'I']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'C', 'M']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'C', 'C', 'D', 'X', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['D', 'X', 'X', 'C', 'V']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'X', 'L', 'V']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['X', 'I', 'I', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['I', 'I', 'V']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'N', 'C', 'M', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'M', '_', 'C', 'I', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'M', 'X', 'C', '9', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['M', 'M', 'M', 'M', 'C', 'M', ' ', 'X', 'C', 'I', 'X']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['N']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['2']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&vec!['-']), 0);
    assert_eq!(numbers::convert_roman_numeral_to_number(&Vec::<char>::new()), 0);
}
