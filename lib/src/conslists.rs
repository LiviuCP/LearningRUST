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

impl<T: Copy + std::cmp::PartialEq> ConsWrapper<T> {
    pub fn create_from_vec(arr: &Vec<T>) -> ConsWrapper<T> {
	let mut result = ConsWrapper::<T>{list: Rc::new(ConsList::<T>::Nil), count: 0};

	for val in arr.iter().rev() {
	    result.list = Rc::new(ConsList::ConsValue(Rc::new(RefCell::new(*val)), result.list));
	    result.count += 1;
	}
	
	result
    }

    pub fn prepend(&mut self, value: &T) {
	self.list = Rc::new(ConsList::ConsValue(Rc::new(RefCell::new(*value)), Rc::clone(&self.list)));
	self.count += 1;
    }

    pub fn reverse(&mut self) {
	let mut old_list = self.list.clone();
	self.list = Rc::new(ConsList::<T>::Nil);
	loop {
	    if let ConsList::ConsValue(val, remaining_list) = Rc::into_inner(old_list).unwrap() {
		old_list = remaining_list;
		self.list = Rc::new(ConsList::ConsValue(Rc::clone(&val), Rc::clone(&self.list)));
	    }
	    else {
		break;
	    }
	}
    }

    pub fn clear(&mut self) {
	self.list = Rc::new(ConsList::<T>::Nil);
	self.count = 0;
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

/*
    pub fn clear(&mut self) {
    }

    pub fn count(&self) -> usize {
	0 as usize
    }

    pub fn head(&self) -> &ConsList<T> {
	&ConsList::<T>::Nil
    }

    pub fn tail(&self) -> &Box<ConsList<T>> {
	// TODO
    }
*/
