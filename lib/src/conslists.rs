use std::{rc::Rc, cell::RefCell};

struct ConsItem<T> {
    value: Option<Rc<RefCell<T>>>,
    next: Option<Rc<ConsItem<T>>>
}

pub struct ConsList<T> {
    first: Option<Rc<ConsItem<T>>>,
    count: usize
}

pub struct ConsIterator<T> {
    current: Option<Rc<ConsItem<T>>>
}

impl<T: Copy> ConsList<T> {
    pub fn create() -> ConsList<T> {
	ConsList::<T>{first: None, count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> ConsList<T> {
	let mut result = ConsList::create();

	for val in arr.iter().rev() {
	    result.prepend(val);
	}

	result
    }

    // this iterator uses the interior mutability pattern, hence choosing a different name from the "standard" iterators (iter(), iter_mut())
    pub fn im_iter(&self) -> ConsIterator<T> {
	ConsIterator {
	    current: self.first.clone()
	}
    }

    pub fn prepend(&mut self, val: &T) {
	self.first = Some(Rc::new(ConsItem{value: Some(Rc::new(RefCell::new(*val))), next: self.first.clone()}));
	self.count += 1;
    }

    pub fn append(&mut self, val: &T) {
	self.reverse();
	self.prepend(val);
	self.reverse();
    }

    pub fn reverse(&mut self) {
	if let Some(mut first_item) = self.first.clone() {
	    self.clear();

	    loop {
		if let Some(value) = first_item.value.clone() {
		    self.first = Some(Rc::new(ConsItem{value: Some(value), next: self.first.clone()}));
		    self.count += 1;

		    if let Some(next_item) = first_item.next.clone() {
			first_item = next_item;
			continue;
		    }
		}

		break;
	    }
	}
    }

    pub fn merge(&mut self, list: &mut ConsList::<T>) {
	self.reverse();

	if let Some(mut first_item) = self.first.clone() {
	    self.clear();

	    loop {
		if let Some(value) = first_item.value.clone() {
		    list.first = Some(Rc::new(ConsItem{value: Some(value), next: list.first.clone()}));
		    list.count += 1;

		    if let Some(next_item) = first_item.next.clone() {
			first_item = next_item;
			continue;
		    }
		}

		break;
	    }
	}

	self.first = list.first.clone();
	self.count = list.count;

	list.clear();
    }

    pub fn head(&self) -> Option<Rc<RefCell<T>>> {
	if let Some(first_item) = self.first.clone() {
	    first_item.value.clone()
	}
	else {
	    None
	}
    }

    pub fn tail(&self) -> Option<Rc<RefCell<T>>> {
	let mut result = None;

	if let Some(first_item) = self.first.clone() {
	    let mut current_value = first_item.value.clone();
	    let mut next_item = first_item.next.clone();

	    loop {
		if let Some(next) = next_item {
		    current_value = next.value.clone();
		    next_item = next.next.clone();
		    continue;
		}

		result = current_value;
		break;
	    }
	}

	result
    }

    pub fn clear(&mut self) {
	self.first = None;
	self.count = 0;
    }

    pub fn content(&self) -> Vec<T> {
	let mut result = Vec::new();

	if let Some(first_item) = self.first.clone() {
	    let mut current_value = first_item.value.clone();
	    let mut next_item = first_item.next.clone();

	    loop {
		if let Some(value) = current_value {
		    result.push(*value.borrow());

		    if let Some(next) = next_item {
			current_value = next.value.clone();
			next_item = next.next.clone();
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

impl<T> Iterator for ConsIterator<T> {
    type Item = Rc<RefCell<T>>;

    fn next(&mut self) -> Option<Self::Item> {
	let mut result = None;

	if let Some(current_item) = self.current.clone() {
	    if let Some(value) = current_item.value.clone() {
		result = Some(value);
	    }
	    else {
		panic!("Value should not be invalid!");
	    }

	    self.current = current_item.next.clone();
	}

	result
    }
}
