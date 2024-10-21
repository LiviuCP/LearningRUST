#[cfg(test)]

use std::{rc::Rc, cell::RefCell as Rcl};
use learn_rust_lib::conslists::{ConsList, ConsWrapper};

#[test]
pub fn test_create() {
    let wrapper = ConsWrapper::<i32>::create();
    assert!(*wrapper.value() == None && wrapper.empty());
}

#[test]
pub fn test_create_from_vec() {
    let mut wrapper = ConsWrapper::create_from_vec(&Vec::<i32>::new());
    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = ConsWrapper::create_from_vec(&vec![-3]);
    assert!(*wrapper.value() == Some(Rc::new(ConsList{value: Some(Rc::new(Rcl::new(-3))),
							 remaining: None})) && wrapper.size() == 1);

    wrapper = ConsWrapper::create_from_vec(&vec![2, 5]);
    assert!(*wrapper.value() == Some(Rc::new(ConsList{value: Some(Rc::new(Rcl::new(2))),
							 remaining: Some(Rc::new(ConsList{value: Some(Rc::new(Rcl::new(5))),
											     remaining: None}))})) &&
	    wrapper.size() == 2);

    wrapper = ConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    assert!(*wrapper.value() == Some(Rc::new(ConsList{
	value: Some(Rc::new(Rcl::new(-4))),
	remaining: Some(Rc::new(ConsList{
	    value: Some(Rc::new(Rcl::new(8))),
	    remaining: Some(Rc::new(ConsList{
		value: Some(Rc::new(Rcl::new(-3))),
		remaining: Some(Rc::new(ConsList{
		    value: Some(Rc::new(Rcl::new(-3))),
		    remaining: Some(Rc::new(ConsList{
			value: Some(Rc::new(Rcl::new(5))),
			remaining: Some(Rc::new(ConsList{
			    value: Some(Rc::new(Rcl::new(0))),
			    remaining: Some(Rc::new(ConsList{
				value: Some(Rc::new(Rcl::new(2))),
				remaining: Some(Rc::new(ConsList{
				    value: Some(Rc::new(Rcl::new(1))),
				    remaining: None}))}))}))}))}))}))}))})) &&
	    wrapper.size() == 8);
}

#[test]
pub fn test_prepend() {
    let mut wrapper = ConsWrapper::create();
    let mut value = -3;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = ConsWrapper::create_from_vec(&vec![5]);
    value = 2;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![2, 5]).value() && wrapper.size() == 2);

    wrapper = ConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    value = -4;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_append() {
    let mut wrapper = ConsWrapper::create();
    let mut value = -3;
    wrapper.append(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = ConsWrapper::create_from_vec(&vec![5]);
    value = 2;
    wrapper.append(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![5, 2]).value() && wrapper.size() == 2);

    wrapper = ConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    value = -4;
    wrapper.append(&value);

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_reverse() {
    let mut wrapper = ConsWrapper::create();
    wrapper.reverse();

    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = ConsWrapper::create_from_vec(&vec![-3]);
    wrapper.reverse();

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = ConsWrapper::create_from_vec(&vec![2, 5]);
    wrapper.reverse();

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![5, 2]).value() && wrapper.size() == 2);

    wrapper = ConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    wrapper.reverse();

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![1, 2, 0, 5, -3, -3, 8, -4]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_merge() {
    let mut first_wrapper = ConsWrapper::create();
    let mut second_wrapper = ConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == None && first_wrapper.empty());
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![-3]);
    second_wrapper = ConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![-3]).value() && first_wrapper.size() == 1);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create();
    second_wrapper = ConsWrapper::create_from_vec(&vec![-3]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![-3]).value() && first_wrapper.size() == 1);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![5, -4]);
    second_wrapper = ConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![5, -4]).value() && first_wrapper.size() == 2);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create();
    second_wrapper = ConsWrapper::create_from_vec(&vec![5, -4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![5, -4]).value() && first_wrapper.size() == 2);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![-8, 1]);
    second_wrapper = ConsWrapper::create_from_vec(&vec![-4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![-8, 1, -4]).value() && first_wrapper.size() == 3);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![-4]);
    second_wrapper = ConsWrapper::create_from_vec(&vec![-8, 1]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![-4, -8, 1]).value() && first_wrapper.size() == 3);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4]);
    second_wrapper = ConsWrapper::create_from_vec(&vec![9, 1, 1, 8]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]).value() && first_wrapper.size() == 9);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = ConsWrapper::create_from_vec(&vec![9, 1, 1, 8]);
    second_wrapper = ConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *ConsWrapper::create_from_vec(&vec![9, 1, 1, 8, 2, -5, 4, 3, 4]).value() && first_wrapper.size() == 9);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());
}

#[test]
pub fn test_head() {
    let mut wrapper = ConsWrapper::create_from_vec(&vec![2, 5, -3, 4]);
    assert_ne!(wrapper.head(), None);

    let mut read_head = wrapper.head();
    assert_eq!(*read_head.unwrap().borrow(), 2);

    let mut write_head = wrapper.head();
    *write_head.unwrap().borrow_mut() = -10;

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![-10, 5, -3, 4]).value() && wrapper.size() == 4);

    wrapper = ConsWrapper::create_from_vec(&vec![-8]);
    assert_ne!(wrapper.head(), None);

    read_head = wrapper.head();
    assert_eq!(*read_head.unwrap().borrow(), -8);

    write_head = wrapper.head();
    *write_head.unwrap().borrow_mut() = 9;

    assert!(*wrapper.value() == *ConsWrapper::create_from_vec(&vec![9]).value() && wrapper.size() == 1);

    wrapper = ConsWrapper::create();
    assert_eq!(wrapper.head(), None);
}

#[test]
pub fn test_clear() {
    let mut wrapper = ConsWrapper::create();
    wrapper.clear();

    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = ConsWrapper::create_from_vec(&vec![2, 5, -3, 4]);
    wrapper.clear();

    assert!(*wrapper.value() == None && wrapper.empty());
}
