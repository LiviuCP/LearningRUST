#[cfg(test)]
use learn_rust_lib::utilities::random;
use learn_rust_lib::utilities::random::IndexGenerator;
use std::time::Instant;

#[test]
pub fn test_stable_index_generator() {
    let mut total_indexes_count = 5;
    let mut generator = random::StableIndexGenerator::create(total_indexes_count);
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
    total_indexes_count = 1;
    generator = random::StableIndexGenerator::create(total_indexes_count);

    assert!(!generator.is_empty());
    assert_eq!(generator.generate(), Some(0));
    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);

    // corner case: no elements
    total_indexes_count = 0;
    generator = random::StableIndexGenerator::create(total_indexes_count);

    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);

    // display a generated sequence to demonstrate the indexes are not repeated
    total_indexes_count = 20;
    generator = random::StableIndexGenerator::create(total_indexes_count);

    assert!(!generator.is_empty());

    println!(
        "StableIndexGenerator - displaying a {} elements indexes sequence:",
        total_indexes_count
    );

    print!("{}", generator.generate().unwrap());

    while !generator.is_empty() {
        print!(", {}", generator.generate().unwrap());
    }

    print!("\n");
}

#[test]
pub fn test_quick_index_generator() {
    let mut total_indexes_count = 5;
    let mut generator = random::QuickIndexGenerator::create(total_indexes_count);
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
    total_indexes_count = 1;
    generator = random::QuickIndexGenerator::create(total_indexes_count);

    assert!(!generator.is_empty());
    assert_eq!(generator.generate(), Some(0));
    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);

    // corner case: no elements
    total_indexes_count = 0;
    generator = random::QuickIndexGenerator::create(total_indexes_count);

    assert!(generator.is_empty());
    assert_eq!(generator.generate(), None);

    // display a generated sequence to demonstrate the indexes are not repeated
    total_indexes_count = 20;
    generator = random::QuickIndexGenerator::create(total_indexes_count);

    assert!(!generator.is_empty());

    println!(
        "QuickIndexGenerator - displaying a {} elements indexes sequence:",
        total_indexes_count
    );

    print!("{}", generator.generate().unwrap());

    while !generator.is_empty() {
        print!(", {}", generator.generate().unwrap());
    }

    print!("\n");
}

#[test]
#[ignore]
pub fn test_performance_stable_vs_quick_index_generator() {
    let total_indexes_count = 100000;

    let mut stable_generator = random::StableIndexGenerator::create(total_indexes_count);
    let mut quick_generator = random::QuickIndexGenerator::create(total_indexes_count);
    let mut stable_collected_indexes = Vec::new();
    let mut quick_collected_indexes = Vec::new();

    stable_collected_indexes.reserve_exact(total_indexes_count);
    quick_collected_indexes.reserve_exact(total_indexes_count);

    let mut start = Instant::now();

    while !stable_generator.is_empty() {
        stable_collected_indexes.push(stable_generator.generate().unwrap());
    }

    let mut duration = start.elapsed();

    println!(
        "Stable index generator: it took {} seconds to generate {} elements",
        duration.as_secs_f64(),
        total_indexes_count
    );

    start = Instant::now();

    while !quick_generator.is_empty() {
        quick_collected_indexes.push(quick_generator.generate().unwrap());
    }

    duration = start.elapsed();

    println!(
        "Quick index generator: it took {} seconds to generate {} elements",
        duration.as_secs_f64(),
        total_indexes_count
    );

    stable_collected_indexes.sort();
    quick_collected_indexes.sort();

    assert_eq!(stable_collected_indexes, quick_collected_indexes);
}
