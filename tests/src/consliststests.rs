#[cfg(test)]

use std::{rc::Rc, cell::RefCell as Rcl};
use learn_rust_lib::conslists::{AltConsList, AltConsWrapper};

#[test]
pub fn test_alt_create() {
    let wrapper = AltConsWrapper::<i32>::create();
    assert!(*wrapper.value() == None && wrapper.empty());
}

#[test]
pub fn test_alt_create_from_vec() {
    let mut wrapper = AltConsWrapper::create_from_vec(&Vec::<i32>::new());
    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = AltConsWrapper::create_from_vec(&vec![-3]);
    assert!(*wrapper.value() == Some(Rc::new(AltConsList{value: Some(Rc::new(Rcl::new(-3))),
							 remaining: None})) && wrapper.size() == 1);

    wrapper = AltConsWrapper::create_from_vec(&vec![2, 5]);
    assert!(*wrapper.value() == Some(Rc::new(AltConsList{value: Some(Rc::new(Rcl::new(2))),
							 remaining: Some(Rc::new(AltConsList{value: Some(Rc::new(Rcl::new(5))),
											     remaining: None}))})) &&
	    wrapper.size() == 2);

    wrapper = AltConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    assert!(*wrapper.value() == Some(Rc::new(AltConsList{
	value: Some(Rc::new(Rcl::new(-4))),
	remaining: Some(Rc::new(AltConsList{
	    value: Some(Rc::new(Rcl::new(8))),
	    remaining: Some(Rc::new(AltConsList{
		value: Some(Rc::new(Rcl::new(-3))),
		remaining: Some(Rc::new(AltConsList{
		    value: Some(Rc::new(Rcl::new(-3))),
		    remaining: Some(Rc::new(AltConsList{
			value: Some(Rc::new(Rcl::new(5))),
			remaining: Some(Rc::new(AltConsList{
			    value: Some(Rc::new(Rcl::new(0))),
			    remaining: Some(Rc::new(AltConsList{
				value: Some(Rc::new(Rcl::new(2))),
				remaining: Some(Rc::new(AltConsList{
				    value: Some(Rc::new(Rcl::new(1))),
				    remaining: None}))}))}))}))}))}))}))})) &&
	    wrapper.size() == 8);
}

#[test]
pub fn test_alt_prepend() {
    let mut wrapper = AltConsWrapper::create();
    let mut value = -3;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = AltConsWrapper::create_from_vec(&vec![5]);
    value = 2;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![2, 5]).value() && wrapper.size() == 2);

    wrapper = AltConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    value = -4;
    wrapper.prepend(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_alt_append() {
    let mut wrapper = AltConsWrapper::create();
    let mut value = -3;
    wrapper.append(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = AltConsWrapper::create_from_vec(&vec![5]);
    value = 2;
    wrapper.append(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![5, 2]).value() && wrapper.size() == 2);

    wrapper = AltConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    value = -4;
    wrapper.append(&value);

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_alt_reverse() {
    let mut wrapper = AltConsWrapper::create();
    wrapper.reverse();

    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = AltConsWrapper::create_from_vec(&vec![-3]);
    wrapper.reverse();

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-3]).value() && wrapper.size() == 1);

    wrapper = AltConsWrapper::create_from_vec(&vec![2, 5]);
    wrapper.reverse();

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![5, 2]).value() && wrapper.size() == 2);

    wrapper = AltConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    wrapper.reverse();

    assert!(*wrapper.value() == *AltConsWrapper::create_from_vec(&vec![1, 2, 0, 5, -3, -3, 8, -4]).value() && wrapper.size() == 8);
}

#[test]
pub fn test_alt_merge() {
    let mut first_wrapper = AltConsWrapper::create();
    let mut second_wrapper = AltConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == None && first_wrapper.empty());
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![-3]);
    second_wrapper = AltConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-3]).value() && first_wrapper.size() == 1);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create();
    second_wrapper = AltConsWrapper::create_from_vec(&vec![-3]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-3]).value() && first_wrapper.size() == 1);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![5, -4]);
    second_wrapper = AltConsWrapper::create();
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![5, -4]).value() && first_wrapper.size() == 2);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create();
    second_wrapper = AltConsWrapper::create_from_vec(&vec![5, -4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![5, -4]).value() && first_wrapper.size() == 2);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![-8, 1]);
    second_wrapper = AltConsWrapper::create_from_vec(&vec![-4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-8, 1, -4]).value() && first_wrapper.size() == 3);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![-4]);
    second_wrapper = AltConsWrapper::create_from_vec(&vec![-8, 1]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![-4, -8, 1]).value() && first_wrapper.size() == 3);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4]);
    second_wrapper = AltConsWrapper::create_from_vec(&vec![9, 1, 1, 8]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]).value() && first_wrapper.size() == 9);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());

    first_wrapper = AltConsWrapper::create_from_vec(&vec![9, 1, 1, 8]);
    second_wrapper = AltConsWrapper::create_from_vec(&vec![2, -5, 4, 3, 4]);
    first_wrapper.merge(&mut second_wrapper);

    assert!(*first_wrapper.value() == *AltConsWrapper::create_from_vec(&vec![9, 1, 1, 8, 2, -5, 4, 3, 4]).value() && first_wrapper.size() == 9);
    assert!(*second_wrapper.value() == None && second_wrapper.empty());
}

#[test]
pub fn test_alt_clear() {
    let mut wrapper = AltConsWrapper::create();
    wrapper.clear();

    assert!(*wrapper.value() == None && wrapper.empty());

    wrapper = AltConsWrapper::create_from_vec(&vec![2, 5, -3, 4]);
    wrapper.clear();

    assert!(*wrapper.value() == None && wrapper.empty());
}
