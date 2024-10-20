/*
TODO: create (mut) iterators
 */

use std::{rc::Rc, cell::RefCell};

#[derive(PartialEq, Debug, Clone)]
pub struct AltConsList<T> {
    pub value: Option<Rc<RefCell<T>>>,
    pub remaining: Option<Rc<AltConsList<T>>>
}

pub struct AltConsWrapper<T> {
    list: Option<Rc<AltConsList<T>>>,
    count: usize
}

impl<T: Copy + std::cmp::PartialEq> AltConsWrapper<T> {
    pub fn create() -> AltConsWrapper<T> {
	AltConsWrapper::<T>{list: None, count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> AltConsWrapper<T> {
	let mut result = AltConsWrapper::create();

	for val in arr.iter().rev() {
	    result.prepend(val);
	}

	result
    }

    pub fn prepend(&mut self, val: &T) {
	self.list = Some(Rc::new(AltConsList{value: Some(Rc::new(RefCell::new(*val))), remaining: self.list.clone()}));
	self.count += 1;
    }

    pub fn value(&self) -> &Option<Rc<AltConsList::<T>>> {
	&self.list
    }

    pub fn size(&self) -> usize {
	self.count
    }

    pub fn empty(&self) -> bool {
	self.count == 0
    }
}


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
    pub fn create() -> ConsWrapper<T> {
	ConsWrapper::<T>{list: Rc::new(ConsList::<T>::Nil), count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> ConsWrapper<T> {
	let mut result = ConsWrapper::create();

	for val in arr.iter().rev() {
	    result.prepend(val);
	}
	
	result
    }

    pub fn prepend(&mut self, value: &T) {
	self.list = Rc::new(ConsList::ConsValue(Rc::new(RefCell::new(*value)), Rc::clone(&self.list)));
	self.count += 1;
    }

    pub fn append(&mut self, value: &T) {
	self.reverse();
	self.prepend(value);
	self.reverse();
    }

    pub fn reverse(&mut self) {
	let mut old_list = self.list.clone();
	self.clear();

	loop {
	    if let ConsList::ConsValue(val, remaining_list) = Rc::into_inner(old_list).unwrap() {
		old_list = remaining_list;
		self.list = Rc::new(ConsList::ConsValue(Rc::clone(&val), Rc::clone(&self.list)));
		self.count += 1;
		continue;
	    }

	    break;
	}
    }

    pub fn merge(&mut self, wrapper: &mut ConsWrapper::<T>) {
	self.reverse();
	let mut old_list = self.list.clone();
	self.clear();

	loop {
	    if let ConsList::ConsValue(val, remaining_list) = Rc::into_inner(old_list).unwrap() {
		old_list = remaining_list;
		wrapper.list = Rc::new(ConsList::ConsValue(Rc::clone(&val), Rc::clone(&wrapper.list)));
		wrapper.count += 1;
		continue;
	    }

	    break;
	}

	self.list = wrapper.list.clone();
	self.count = wrapper.count;

	wrapper.clear();
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
    pub fn head(&self) -> &ConsList<T> {
	&ConsList::<T>::Nil
    }

    pub fn tail(&self) -> &Box<ConsList<T>> {
	// TODO
    }
*/
