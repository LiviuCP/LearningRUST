/* Read-only cons lists where the item value cannot be modified (no interior mutability pattern) */

use std::{rc::Rc, result::Result};
use crate::cons::InvalidIndex;

#[derive(PartialEq, Debug)]
struct RoConsItem<T> {
    value: Option<Rc<T>>,
    next: Option<Rc<RoConsItem<T>>>
}

pub struct RoConsList<T> {
    first: Option<Rc<RoConsItem<T>>>,
    count: usize
}

pub struct RoConsIterator<T> {
    current: Option<Rc<RoConsItem<T>>>
}

impl<T: Clone + PartialEq + std::fmt::Debug> RoConsList<T> {
    pub fn create() -> RoConsList<T> {
	RoConsList::<T>{first: None, count: 0}
    }

    pub fn create_from_vec(arr: &Vec<T>) -> RoConsList<T> {
	let mut result = RoConsList::create();

	for val in arr.iter().rev() {
	    result.push_front(val);
	}

	result
    }

    pub fn iter(&self) -> RoConsIterator<T> {
	RoConsIterator {
	    current: self.first.clone()
	}
    }

    pub fn push_front(&mut self, val: &T) {
	self.first = Some(Rc::new(RoConsItem{value: Some(Rc::new(val.clone())), next: self.first.clone()}));
	self.count += 1;
    }

    pub fn pop_front(&mut self) -> Option<Rc<T>> {
	let mut result = None;
	
	if let Some(first_item) = self.first.clone() {
	    result = first_item.value.clone();
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
		if let Some(first_value) = first_item.value.clone() {
		    self.first = Some(Rc::new(RoConsItem{value: Some(first_value), next: self.first.clone()}));
		    self.count += 1;

		    if let Some(next_item) = first_item.next.clone() {
			first_item = next_item;
			continue;
		    }

		    break;
		}
		else {
		    panic!("Value is not allowed to be none!");
		}
	    }
	}
	else {
	    self.push_front(&val);
	}
    }

    pub fn pop_back(&mut self) -> Option<Rc<T>> {
	let mut result = None;

	if self.count > 0 {
	    let mut all_items_but_last = RoConsList::create();
	    let last = self.stack_items(self.count - 1, &mut all_items_but_last);

	    if let Some(last_item) = last {
		result = last_item.value.clone();

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
		let mut items_preceding_insert_position = RoConsList::create();
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

    pub fn remove(&mut self, index: usize) -> Result<Rc<T>, InvalidIndex> {
	let mut result = Err(InvalidIndex);

	if index < self.count {
	    // step 1: items preceding the removed position to be added into a temporary list (in reverse order)
	    let mut items_preceding_remove_position = RoConsList::create();
	    let resulting_first_item = self.stack_items(index, &mut items_preceding_remove_position);

	    if let Some(mut first_item) = resulting_first_item {
		// step 2: grab the removed value
		if let Some(first_value) = first_item.value.clone() {
		    result = Ok(first_value);

		    if let Some(next_item) = first_item.next.clone() {
			first_item = next_item;
		    }
		}
		else {
		    panic!("Removed item value is not allowed to be none!");
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
		if let Some(value) = first_item.value.clone() {
		    self.first = Some(Rc::new(RoConsItem{value: Some(value), next: self.first.clone()}));
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

    pub fn merge(&mut self, list: &mut RoConsList::<T>) {
	self.reverse();

	if let Some(mut first_item) = self.first.clone() {
	    self.clear();

	    loop {
		if let Some(value) = first_item.value.clone() {
		    list.first = Some(Rc::new(RoConsItem{value: Some(value), next: list.first.clone()}));
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

    pub fn split(&mut self, list: &mut RoConsList::<T>, index: usize) -> Result<usize, InvalidIndex> {
	let mut result = Err(InvalidIndex);

	if index < self.count {
	    // step 1: items preceding the split position to be added into a temporary list (in reverse order)
	    let mut items_preceding_split_position = RoConsList::create();
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

    pub fn head(&self) -> Option<Rc<T>> {
	if let Some(first_item) = self.first.clone() {
	    first_item.value.clone()
	}
	else {
	    None
	}
    }

    pub fn tail(&self) -> Option<Rc<T>> {
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

    pub fn at(&self, index: usize) -> Result<Rc<T>, InvalidIndex> {
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

		if let Some(first_value) = first_item.value.clone() {
		    result = Ok(first_value);
		}
		else {
		    panic!("Value is not allowed to be none!");
		}
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
		    result.push((*value).clone());

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
	let is_empty = self.count == 0;

	if is_empty {
	    debug_assert_eq!(self.first, None);
	}
	else {
	    debug_assert_ne!(self.first, None);
	}

	is_empty
    }

    fn stack_items(&self, items_to_stack_count: usize, stack: &mut RoConsList::<T>) -> Option<Rc<RoConsItem<T>>> {
	let mut first_unstacked_item = None;
	stack.clear();

	if let Some(mut first_item) = self.first.clone() {
	    first_unstacked_item = Some(first_item.clone());
	    let mut current_index: usize = 0;

	    while current_index < items_to_stack_count {
		if let Some(first_value) = first_item.value.clone() {
		    stack.first = Some(Rc::new(RoConsItem{value: Some(first_value), next: stack.first.clone()}));
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

		panic!("Value is not allowed to be none!");
	    }
	}

	first_unstacked_item
    }

    fn recover_items_from_stack(&mut self, stack: &RoConsList::<T>) {
	if let Some(mut first_item) = stack.first.clone() {
	    loop {
		if let Some(first_value) = first_item.value.clone() {
		    self.first = Some(Rc::new(RoConsItem{value: Some(first_value), next: self.first.clone()}));
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
}

impl<T> Iterator for RoConsIterator<T> {
    type Item = Rc<T>;

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
