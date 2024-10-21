/*
TODO: create (mut) iterators
 */

use std::{rc::Rc, cell::RefCell};

#[derive(PartialEq, Debug, Clone)]
pub struct ConsList<T> {
    pub value: Option<Rc<RefCell<T>>>,
    pub remaining: Option<Rc<ConsList<T>>>
}

pub struct ConsWrapper<T> {
    list: Option<Rc<ConsList<T>>>,
    count: usize
}

impl<T: Copy + std::cmp::PartialEq> ConsWrapper<T> {
    pub fn create() -> ConsWrapper<T> {
	ConsWrapper::<T>{list: None, count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> ConsWrapper<T> {
	let mut result = ConsWrapper::create();

	for val in arr.iter().rev() {
	    result.prepend(val);
	}

	result
    }

    pub fn prepend(&mut self, val: &T) {
	self.list = Some(Rc::new(ConsList{value: Some(Rc::new(RefCell::new(*val))), remaining: self.list.clone()}));
	self.count += 1;
    }

    pub fn append(&mut self, val: &T) {
	self.reverse();
	self.prepend(val);
	self.reverse();
    }

    pub fn reverse(&mut self) {
	if let Some(current_list) = self.list.clone() {
	    let mut old_list = current_list;
	    self.clear();

	    loop {
		if let Some(val) = old_list.value.clone() {
		    self.list = Some(Rc::new(ConsList{value: Some(val), remaining: self.list.clone()}));
		    self.count += 1;

		    if let Some(rem) = old_list.remaining.clone() {
			old_list = rem;
			continue;
		    }
		}

		break;
	    }
	}
    }

    pub fn merge(&mut self, wrapper: &mut ConsWrapper::<T>) {
	self.reverse();

	if let Some(current_list) = self.list.clone() {
	    let mut old_list = current_list;
	    self.clear();

	    loop {
		if let Some(val) = old_list.value.clone() {
		    wrapper.list = Some(Rc::new(ConsList{value: Some(val), remaining: wrapper.list.clone()}));
		    wrapper.count += 1;

		    if let Some(rem) = old_list.remaining.clone() {
			old_list = rem;
			continue;
		    }
		}

		break;
	    }
	}

	self.list = wrapper.list.clone();
	self.count = wrapper.count;

	wrapper.clear();
    }

    pub fn clear(&mut self) {
	self.list = None;
	self.count = 0;
    }

    pub fn value(&self) -> &Option<Rc<ConsList::<T>>> {
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
    pub fn head(&self) -> &ConsList<T> {
	&ConsList::<T>::Nil
    }

    pub fn tail(&self) -> &Box<ConsList<T>> {
	// TODO
    }
*/
