/*
TODO: create (mut) iterators
 */

use std::{rc::Rc, cell::RefCell};

#[derive(PartialEq, Debug, Clone)]
pub enum ConsList<T> {
    ConsValue(Rc<RefCell<T>>, Rc<ConsList<T>>),
    Nil
}

pub struct ConsWrapper<T> {
    list: Rc<ConsList<T>>,
    count: usize
}

impl<T: Copy> ConsWrapper<T> {
    pub fn create_from_vec(arr: &Vec<T>) -> ConsWrapper<T> {
	let mut result = ConsWrapper::<T>{list: Rc::new(ConsList::<T>::Nil), count: 0};

	for val in arr.iter().rev() {
	    result.list = Rc::new(ConsList::ConsValue(Rc::new(RefCell::new(*val)), result.list));
	    result.count += 1;
	}
	
	result
    }

    pub fn value(&self) -> &ConsList::<T> {
	&self.list
    }

    pub fn size(&self) -> usize {
	self.count
    }

    pub fn empty(&self) -> bool {
	self.count == 0
    }
}

/*impl<T: Copy> ConsList<T> {
    pub fn prepend(&mut self, value: &T) {
	*self = ConsList::ConsValue(*value, Box::new(self.clone()));
    }*/
/*
    pub fn reverse(&self) -> ConsList<T> {
	ConsList::<T>::Nil
    }

    pub fn clear(&mut self) {
    }

    pub fn count(&self) -> usize {
	0 as usize
    }

    pub fn head(&self) -> &ConsList<T> {
	&ConsList::<T>::Nil
    }
*/
//    pub fn tail(&self) -> &Box<ConsList<T>> {
	// TODO
//    }
//}
