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
pub fn test_divide_higher_number_by_two() {
    let mut first = 6;
    let mut second = -3;

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 3 && first == 3 && second == -3);

    first = -3;
    second = 6;

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 3 && first == -3 && second == 3);

    first = 4;
    second = 4;

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 2 && first == 2 && second == 4);

    first = 5;
    second = 10;

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 5 && first == 5 && second == 5);

    first = 15;
    second = 7;

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 7 && first == 7 && second == 7);

    /* additional test: use the resulting reference */
    first = 14;
    second = -40;
    let result = numbers::divide_higher_number_by_two(&mut first, &mut second);

    assert_eq!(*result, 7);

    // first += *result; // result should not be used for modifying first as first is already borrowed to result (uncomment to see compiling error)
    second *= -result; // could be written second *= -*result but it's not needed as automatic dereferencing of result is being performed

    assert!(*numbers::divide_higher_number_by_two(&mut first, &mut second) == 140 && first == 7 && second == 140);

    /* additional tests with temporary values - dereferencing should be done immediately */
    assert_eq!(*numbers::divide_higher_number_by_two(&mut 11, &mut 3), 5);
    assert_eq!(*numbers::divide_higher_number_by_two(&mut -6, &mut -6), -3);
    assert_eq!(*numbers::divide_higher_number_by_two(&mut -14, &mut -4), -2);
    assert_eq!(*numbers::divide_higher_number_by_two(&mut 5, &mut 11), 5);
    assert_eq!(*numbers::divide_higher_number_by_two(&mut 14, &mut 7), 7);
}

#[test]
pub fn test_int_vector_wrapper_create() {
    let mut int_vec = vec![5, -3, 4, 2, 8];
    let mut int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);

    assert_eq!(*int_vec_wrapper.content(), vec![5, -3, 4, 2, 8]);

    int_vec = vec![9];
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);

    assert_eq!(*int_vec_wrapper.content(), vec![9]);

    int_vec.clear();
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);

    assert!(int_vec_wrapper.content().is_empty());
}

#[test]
pub fn test_int_vector_wrapper_add_vector() {
    let mut int_vec = vec![5, -3, 9, 2, 9];
    let int_vec_to_add = vec![4, 9, 9, 0, 5];
    let mut int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    let mut added_elements_count = int_vec_wrapper.add_vector(&int_vec_to_add);

    assert!(added_elements_count == 5 && *int_vec_wrapper.content() == vec![9, 6, 18, 2, 14]);

    int_vec = vec![2, 3, -4, 5];
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    added_elements_count = int_vec_wrapper.add_vector(&int_vec_to_add);

    assert!(added_elements_count == 4 && *int_vec_wrapper.content() == vec![6, 12, 5, 5]);

    int_vec = vec![10, 8, 7, 6, 5, 4];
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    added_elements_count = int_vec_wrapper.add_vector(&int_vec_to_add);

    assert!(added_elements_count == 5 && *int_vec_wrapper.content() == vec![14, 17, 16, 6, 10, 4]);

    int_vec = Vec::new();
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    added_elements_count = int_vec_wrapper.add_vector(&int_vec_to_add);

    assert!(added_elements_count == 0 && int_vec_wrapper.content().is_empty());

    int_vec = int_vec_to_add.clone();
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    added_elements_count = int_vec_wrapper.add_vector(&Vec::new());

    assert!(added_elements_count == 0 && *int_vec_wrapper.content() == int_vec_to_add);
}

#[test]
pub fn test_int_vector_wrapper_push_element() {
    let mut int_vec = vec![5, -3, 9, 2, 9];
    let mut int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    let element = 40;

    assert!(*int_vec_wrapper.push_element(&element) == 40 && *int_vec_wrapper.content() == vec![5, -3, 9, 2, 9, 40]);
    assert!(*int_vec_wrapper.push_element(&-30) == -30 && *int_vec_wrapper.content() == vec![5, -3, 9, 2, 9, 40, -30]);

    int_vec = Vec::new();
    int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);

    assert!(*int_vec_wrapper.push_element(&-30) == -30 && *int_vec_wrapper.content() == vec![-30]);
    assert!(*int_vec_wrapper.push_element(&element) == 40 && *int_vec_wrapper.content() == vec![-30, 40]);
}

#[test]
pub fn test_int_vector_wrapper_clear() {
    let mut int_vec = vec![5, -3, 9, 2, 9];
    let mut int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    int_vec_wrapper.clear();

    assert!(int_vec_wrapper.content().is_empty());

    let mut int_vec = Vec::new();
    let mut int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
    int_vec_wrapper.clear();

    assert!(int_vec_wrapper.content().is_empty());
}

#[test]
pub fn test_int_vector_mixed() {
    let mut int_vec = vec![5, -3, 9, 2, 9];

    {
	let mut first_int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
	first_int_vec_wrapper.add_vector(&vec![3, 6, 9, 12]);
    } // not mandatory but to illustrate this wrapper cannot be re-used (int_vec to be borrowed immutably into assert and then mutably to another wrapper)

    assert_eq!(int_vec, vec![8, 3, 18, 14, 9]);

    {
	let mut second_int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
	second_int_vec_wrapper.push_element(&20);
    } // same here

    assert_eq!(int_vec, vec![8, 3, 18, 14, 9, 20]);

    {
	let mut third_int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
	third_int_vec_wrapper.clear();
    } // same here

    assert!(int_vec.is_empty());

    int_vec.push(5);

    {
	let mut fourth_int_vec_wrapper = numbers::IntVectorWrapper::create(&mut int_vec);
	fourth_int_vec_wrapper.push_element(&-9);
	assert_eq!(*fourth_int_vec_wrapper.content(), vec![5, -9]);

	fourth_int_vec_wrapper.add_vector(&vec![9, 10]);
	assert_eq!(*fourth_int_vec_wrapper.content(), vec![14, 1]);
    } // not mandatory but to illustrate this wrapper cannot be re-used (int_vec retakes control)

    int_vec.clear();
    assert!(int_vec.is_empty());
}
