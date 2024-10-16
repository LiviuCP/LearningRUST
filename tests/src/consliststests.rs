#[cfg(test)]

use std::{rc::Rc, cell::RefCell as Rcl};
use learn_rust_lib::conslists::{ConsList, ConsWrapper};
use ConsList::{ConsValue, Nil};

#[test]
pub fn test_create_from_vec() {
    let mut wrapper = ConsWrapper::create_from_vec(&Vec::new());
    assert!(*wrapper.value() == Nil && wrapper.empty());

    wrapper = ConsWrapper::create_from_vec(&vec![-3]);
    assert!(*wrapper.value() == ConsValue(Rc::new(Rcl::new(-3)), Rc::new(Nil)) && wrapper.size() == 1);

    wrapper = ConsWrapper::create_from_vec(&vec![2, 5]);
    assert!(*wrapper.value() == ConsValue(Rc::new(Rcl::new(2)),
					  Rc::new(ConsValue(Rc::new(Rcl::new(5)),
							    Rc::new(Nil)))) &&
	    wrapper.size() == 2);

    wrapper = ConsWrapper::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    assert!(*wrapper.value() == ConsValue(Rc::new(Rcl::new(-4)),
					  Rc::new(ConsValue(Rc::new(Rcl::new(8)),
							    Rc::new(ConsValue(Rc::new(Rcl::new(-3)),
									      Rc::new(ConsValue(Rc::new(Rcl::new(-3)),
												Rc::new(ConsValue(Rc::new(Rcl::new(5)),
														  Rc::new(ConsValue(Rc::new(Rcl::new(0)),
																    Rc::new(ConsValue(Rc::new(Rcl::new(2)),
																		      Rc::new(ConsValue(Rc::new(Rcl::new(1)),
																					Rc::new(Nil)))))))))))))))) &&
	    wrapper.size() == 8);
}

#[test]
pub fn test_prepend() {
    let mut wrapper = ConsWrapper::create_from_vec(&Vec::new());
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
