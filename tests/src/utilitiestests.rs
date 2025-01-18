#[cfg(test)]
use learn_rust_lib::utilities;
use learn_rust_lib::utilities::random;

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
    assert_eq!(
        utilities::get_digits(20543219),
        vec![2, 0, 5, 4, 3, 2, 1, 9]
    );
}

#[test]
pub fn test_index_generator() {
    let total_indexes_count = 5;
    let mut generator = random::IndexGenerator::create(total_indexes_count);
    let mut collected_indexes = Vec::new();
    let mut required_indexes_count = total_indexes_count;

    assert!(!generator.is_empty());

    while required_indexes_count > 0 {
        let index = generator.generate();
        assert_ne!(index, None);

        let index = index.unwrap();
        assert!(index < total_indexes_count);

        collected_indexes.push(index);
        required_indexes_count -= 1;
    }

    assert!(generator.is_empty());

    collected_indexes.sort();

    assert_eq!(collected_indexes, vec![0, 1, 2, 3, 4]);
    assert_eq!(generator.generate(), None);

    // special case: 1 element
    generator = random::IndexGenerator::create(1);

    assert!(!generator.is_empty());
    assert_eq!(generator.generate(), Some(0));
    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);

    // corner case: no elements
    generator = random::IndexGenerator::create(0);

    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);
}
