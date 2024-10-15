#[cfg(test)]
use learn_rust_lib::conslists;
use conslists::ConsList::{ConsValue, Nil};

#[test]
pub fn test_create_from_vec() {
    assert_eq!(ConsValue(2, Box::new(ConsValue(5, Box::new(Nil)))), conslists::ConsList::<i32>::create_from_vec(&vec![2, 5]));
}
