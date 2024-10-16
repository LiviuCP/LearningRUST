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

/*#[test]
pub fn test_prepend() {
    let mut list = Nil;
    let mut value = -3;
    list.prepend(&value);

    assert_eq!(list, ConsValue(-3, Box::new(Nil)));

    list = ConsValue(5, Box::new(Nil));
    value = 2;
    list.prepend(&value);

    assert_eq!(list, ConsValue(2, Box::new(ConsValue(5, Box::new(Nil)))));

    list = ConsValue(8, Box::new(ConsValue(-3, Box::new(ConsValue(-3, Box::new(ConsValue(5, Box::new(ConsValue(0, Box::new(ConsValue(2, Box::new(ConsValue(1, Box::new(Nil))))))))))))));
    value = -4;
    list.prepend(&value);

    assert_eq!(list, ConsValue(-4, Box::new(ConsValue(8, Box::new(ConsValue(-3, Box::new(ConsValue(-3, Box::new(ConsValue(5, Box::new(ConsValue(0, Box::new(ConsValue(2, Box::new(ConsValue(1, Box::new(Nil)))))))))))))))));
}*/
