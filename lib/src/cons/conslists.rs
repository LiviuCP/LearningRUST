use std::{rc::Rc, result::Result, fmt::Debug};
use crate::cons::InvalidIndex;

#[derive(PartialEq, Debug)]
struct ConsItem<T> {
    value: T,
    next: Option<Rc<ConsItem<T>>>
}

pub struct ConsList<T> {
    first: Option<Rc<ConsItem<T>>>,
    count: usize
}

pub struct ConsIterator<T> {
    current: Option<Rc<ConsItem<T>>>
}

impl<T: Clone + PartialEq + std::fmt::Debug> ConsList<T> {
    pub fn create() -> ConsList<T> {
	ConsList::<T>{first: None, count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> ConsList<T> {
	let mut result = ConsList::create();

	for val in arr.iter().rev() {
	    result.push_front(val);
	}

	result
    }

    pub fn iter(&self) -> ConsIterator<T> {
	ConsIterator {
	    current: self.first.clone()
	}
    }

    pub fn push_front(&mut self, val: &T) {
	self.first = Some(Rc::new(ConsItem{value: val.clone(), next: self.first.clone()}));
	self.count += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
	let mut result = None;
	
	if let Some(first_item) = self.first.clone() {
	    result = Some(first_item.value.clone());
	    self.first = first_item.next.clone();
	    self.count -= 1;
	}

	result
    }

    pub fn push_back(&mut self, val: &T) {
	self.reverse();

	if let Some(mut first_item) = self.first.clone() {
	    self.clear();
	    self.push_front(&val);

	    loop {
		self.first = Some(Rc::new(ConsItem{value: first_item.value.clone(), next: self.first.clone()}));
		self.count += 1;

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		    continue;
		}

		break;
	    }
	}
	else {
	    self.push_front(&val);
	}
    }

    pub fn pop_back(&mut self) -> Option<T> {
	let mut result = None;

	if self.count > 0 {
	    let mut all_items_but_last = ConsList::create();
	    let last = self.stack_items(self.count - 1, &mut all_items_but_last);

	    if let Some(last_item) = last {
		result = Some(last_item.value.clone());

		self.clear();
		self.recover_items_from_stack(&all_items_but_last);
	    }
	    else {
		panic!("Last item is not allowed to be none!");
	    }
	}

	result
    }

    pub fn insert(&mut self, val: &T, index: usize) -> Result<usize, InvalidIndex> {
	let mut result = Err(InvalidIndex);
	
	if index <= self.count {
	    if self.count > 0 {
		// step 1: items preceding the insertion position to be added into a temporary list (in reverse order)
		let mut items_preceding_insert_position = ConsList::create();
		let resulting_first_item = self.stack_items(index, &mut items_preceding_insert_position);

		// step 2: rebuild list by keeping the items that follow the insertion position
		let next_elements_count = self.count - index;
		self.clear();

		if next_elements_count > 0 {
		    self.first = resulting_first_item;
		    self.count = next_elements_count;
		}

		// step 3: add (prepend) the inserted item
		self.push_front(&val);

		// step 4: prepend the preceding items (reversing them back from temporary list guarantees the correct order)
		self.recover_items_from_stack(&items_preceding_insert_position);
	    }
	    else {
		self.push_front(&val); // corner case: insert into an empty list
	    }

	    result = Ok(index);
	}

	result
    }

    pub fn remove(&mut self, index: usize) -> Result<T, InvalidIndex> {
	let mut result = Err(InvalidIndex);

	if index < self.count {
	    // step 1: items preceding the removed position to be added into a temporary list (in reverse order)
	    let mut items_preceding_remove_position = ConsList::create();
	    let resulting_first_item = self.stack_items(index, &mut items_preceding_remove_position);

	    if let Some(mut first_item) = resulting_first_item {
		// step 2: grab the removed value
		result = Ok(first_item.value.clone());

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		}

		// step 3: rebuild list by keeping the items that follow the removal position
		let next_elements_count = self.count - index - 1;
		self.clear();

		if next_elements_count > 0 {
		    self.first = Some(first_item);
		    self.count = next_elements_count;
		}
	    }
	    else {
		panic!("Resulting first item is not allowed to be none!");
	    }

	    // step 4: prepend the items preceding the removed value (reversing them back from temporary list guarantees the correct order)
	    self.recover_items_from_stack(&items_preceding_remove_position);
	}

	result
    }

    pub fn reverse(&mut self) {
	if let Some(mut first_item) = self.first.clone() {
	    self.clear();

	    loop {
		self.first = Some(Rc::new(ConsItem{value: first_item.value.clone(), next: self.first.clone()}));
		self.count += 1;

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		    continue;
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
		list.first = Some(Rc::new(ConsItem{value: first_item.value.clone(), next: list.first.clone()}));
		list.count += 1;

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		    continue;
		}

		break;
	    }
	}

	self.first = list.first.clone();
	self.count = list.count;

	list.clear();
    }

    pub fn split(&mut self, list: &mut ConsList::<T>, index: usize) -> Result<usize, InvalidIndex> {
	let mut result = Err(InvalidIndex);

	if index < self.count {
	    // step 1: items preceding the split position to be added into a temporary list (in reverse order)
	    let mut items_preceding_split_position = ConsList::create();
	    let resulting_first_item = self.stack_items(index, &mut items_preceding_split_position);

	    // step 2: add elements starting with split position to the list provided as argument
	    if let Some(first_item) = resulting_first_item {
		let next_elements_count = self.count - index;

		self.clear();
		list.clear();
		list.first = Some(first_item);
		list.count = next_elements_count;
	    }
	    else {
		panic!("Resulting first item is not allowed to be none!");
	    }

	    // step 3: prepend the items preceding the split position to current list (reversing them back from temporary list guarantees the correct order)
	    self.recover_items_from_stack(&items_preceding_split_position);

	    result = Ok(index);
	}

	result
    }

    pub fn head(&self) -> Option<T> {
	if let Some(first_item) = self.first.clone() {
	    Some(first_item.value.clone())
	}
	else {
	    None
	}
    }

    pub fn tail(&self) -> Option<T> {
	let mut result = None;

	if let Some(first_item) = self.first.clone() {
	    let mut current_value = first_item.value.clone();
	    let mut next_item = first_item.next.clone();

	    result = loop {
		if let Some(next) = next_item {
		    current_value = next.value.clone();
		    next_item = next.next.clone();
		    continue;
		}

		break Some(current_value);
	    }
	}

	result
    }

    pub fn at(&self, index: usize) -> Result<T, InvalidIndex> {
	let mut result = Err(InvalidIndex);

	if index < self.count {
	    if let Some(mut first_item) = self.first.clone() {
		let mut current_index: usize = 0;

		while current_index < index {
		    if let Some(next_item) = first_item.next.clone() {
			first_item = next_item;
			current_index += 1;
			continue;
		    }

		    panic!("Next item is not allowed to be none!");
		}

		result = Ok(first_item.value.clone());
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
		result.push(current_value.clone());

		if let Some(next) = next_item {
		    current_value = next.value.clone();
		    next_item = next.next.clone();
		    continue;
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
	let is_empty = self.count == 0;

	if is_empty {
	    debug_assert_eq!(self.first, None);
	}
	else {
	    debug_assert_ne!(self.first, None);
	}

	is_empty
    }

    fn stack_items(&self, items_to_stack_count: usize, stack: &mut ConsList::<T>) -> Option<Rc<ConsItem<T>>> {
	let mut first_unstacked_item = None;
	stack.clear();

	if let Some(mut first_item) = self.first.clone() {
	    first_unstacked_item = Some(first_item.clone());
	    let mut current_index: usize = 0;

	    while current_index < items_to_stack_count {
		stack.first = Some(Rc::new(ConsItem{value: first_item.value.clone(), next: stack.first.clone()}));
		stack.count += 1;

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		    first_unstacked_item = Some(first_item.clone());
		    current_index += 1;
		    continue;
		}

		first_unstacked_item = None;
		break;
	    }
	}

	first_unstacked_item
    }

    fn recover_items_from_stack(&mut self, stack: &ConsList::<T>) {
	if let Some(mut first_item) = stack.first.clone() {
	    loop {
		self.first = Some(Rc::new(ConsItem{value: first_item.value.clone(), next: self.first.clone()}));
		self.count += 1;

		if let Some(next_item) = first_item.next.clone() {
		    first_item = next_item;
		    continue;
		}

		break;
	    }
	}
    }
}

impl<T: Clone> Iterator for ConsIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
	let mut result = None;

	if let Some(current_item) = self.current.clone() {
	    result = Some(current_item.value.clone());
	    self.current = current_item.next.clone();
	}

	result
    }
}
