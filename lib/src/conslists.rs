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

    pub fn iter(&self) -> ConsIterator<T> {
	ConsIterator {
	    list: self.list.clone()
	}
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

    pub fn head(&self) -> Option<Rc<RefCell<T>>> {
	if let Some(current_list) = self.list.clone() {
	    current_list.value.clone()
	}
	else {
	    None
	}
    }

    pub fn tail(&self) -> Option<Rc<RefCell<T>>> {
	let mut result = None;

	if let Some(current_list) = self.list.clone() {
	    let mut current_value = current_list.value.clone();
	    let mut remaining_list = current_list.remaining.clone();

	    loop {
		if let Some(rem_list) = remaining_list {
		    current_value = rem_list.value.clone();
		    remaining_list = rem_list.remaining.clone();
		    continue;
		}

		result = current_value;
		break;
	    }
	}

	result
    }

    pub fn clear(&mut self) {
	self.list = None;
	self.count = 0;
    }

    pub fn content(&self) -> Vec<T> {
	let mut result = Vec::new();

	if let Some(current_list) = self.list.clone() {
	    let mut current_value = current_list.value.clone();
	    let mut remaining_list = current_list.remaining.clone();

	    loop {
		if let Some(value) = current_value {
		    result.push(*value.borrow());

		    if let Some(rem_list) = remaining_list {
			current_value = rem_list.value.clone();
			remaining_list = rem_list.remaining.clone();
			continue;
		    }
		}

		break;
	    }
	}

	result
    }

    pub fn size(&self) -> usize {
	self.count
    }

    pub fn empty(&self) -> bool {
	self.count == 0
    }
}

pub struct ConsIterator<T> {
    list: Option<Rc<ConsList<T>>>
}

impl<T> Iterator for ConsIterator<T> {
    type Item = Rc<ConsList<T>>;

    fn next(&mut self) -> Option<Self::Item> {
	let result = self.list.clone();

	if let Some(cons_list) = self.list.clone() {
	    self.list = cons_list.remaining.clone();
	}

	result
    }
}
