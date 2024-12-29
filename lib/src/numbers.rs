use std::collections::HashMap;

pub mod romannumerals;

pub fn reverse_int(input:(u64, u8)) -> (u64, u8) {
    fn update_input_and_output_numbers(input_number: &mut u64, output_number: &mut u64) -> u8 {
	let found_digit = *input_number % 10;
	*output_number = *output_number * 10 + found_digit;
	*input_number = *input_number / 10;

	found_digit as u8
    }

    let mut input_number = input.0;
    let mut output_number = 0;
    let mut output_preceding_zeroes_count:u8 = 0;

    // find all trailing zeroes and convert them to preceding zeroes
    while input_number > 0 {
	let found_digit = update_input_and_output_numbers(&mut input_number, &mut output_number);

	if found_digit != 0 {
            break;
	}

	output_preceding_zeroes_count += 1;
    }

    // handle remaining digits
    while input_number > 0 {
	update_input_and_output_numbers(&mut input_number, &mut output_number);
    }

    if output_number == 0 {
	output_preceding_zeroes_count = input.1;
    }
    else {
	output_number *= u64::pow(10, input.1 as u32);
    }

    (output_number, output_preceding_zeroes_count)
}

pub fn compute_median(numbers_list: &Vec::<i32>) -> Option<i32> {
    let mut result: Option::<i32> = None;
    let list_size = numbers_list.len();

    if list_size > 0 {
	let mut sorted_numbers_list = numbers_list.clone(); // the original list should not be modified, we work on a copy
	sorted_numbers_list.sort();

	let median_index = list_size / 2;
	result = Some(sorted_numbers_list[median_index]);

	if list_size % 2 == 0 {
	    // actually it should be the exact floating point average, i.e. 2 + 3 = 2,5 (not 2), however for simplicity we will do a "floor" average
	    result = Some((sorted_numbers_list[median_index-1] + result.unwrap()) / 2);
	}
    }

    result
}

// computes the value(s) that occur(s) most often (mode = number of occurrences of this/these value(s))
pub fn compute_mode(numbers_list: &Vec::<i32>) -> (usize, Vec::<i32>) {
    let mut occurrences_map = HashMap::new();

    // count the occurrences of each value from input vector
    for value in numbers_list.iter() {
	let value_occurrences = occurrences_map.entry(*value).or_insert(0);
	*value_occurrences += 1;
    }

    let mut values_with_mode = Vec::<i32>::new(); // value(s) with max number of occurrences
    let mut mode:usize = 0; // max number of occurrences

    for (value, occurrences) in &occurrences_map {
	if *occurrences > mode {
	    mode = *occurrences;
	    values_with_mode.clear();
	    values_with_mode.push(*value);
	}
	else if *occurrences == mode {
	    values_with_mode.push(*value);
	}
    }

    // sort the values with max occurrences to get a consistent result (not mandatory but useful for testing)
    values_with_mode.sort();

    (mode, values_with_mode)
}

/* Some use cases for lifetimes */

pub fn divide_higher_number_by_two<'a>(first: &'a mut i32, second: &'a mut i32) -> &'a i32 {
    let result = if first >= second {first} else {second};
    *result /= 2;
    result
}

pub fn move_add<'a>(to_be_added_to: &'a mut Vec<i32>, to_add: &mut Vec<i32>) -> &'a Vec<i32> {
    for (dest_element, src_element) in to_be_added_to.iter_mut().zip(to_add.iter()) {
	*dest_element += src_element;
    }

    to_add.clear();
    to_be_added_to
}

pub struct IntVectorWrapper<'a> {
    int_vector: &'a mut Vec<i32>,
    average: &'a mut i32
}

impl<'a> IntVectorWrapper<'a> {
    pub fn create(int_vec: &'a mut Vec<i32>, avg: &'a mut i32) -> IntVectorWrapper<'a> {
	let mut result = IntVectorWrapper{int_vector: int_vec, average: avg};
	result.compute_average();
	result
    }

    pub fn content(&self) -> &Vec<i32> {
	self.int_vector
    }

    pub fn average(&self) -> &i32 {
	self.average
    }

    pub fn add_vector(&mut self, int_vector: &Vec<i32>) -> usize {
	let mut added_elements_count = 0;

	for (dest_element, src_element) in self.int_vector.iter_mut().zip(int_vector.iter()) {
	    *dest_element += src_element;
	    added_elements_count += 1;
	}
	
	
	if added_elements_count > 0 {
	    self.compute_average();
	}

	added_elements_count
    }

    pub fn push_element(&mut self, element: &i32) -> &i32 {
	self.int_vector.push(*element);
	self.compute_average();
	&self.int_vector[self.int_vector.len() - 1]
    }

    pub fn clear(&mut self) {
	self.int_vector.clear();
	*self.average = 0;
    }

    fn compute_average(&mut self) {
	*self.average = if !self.int_vector.is_empty() {self.int_vector.iter().sum::<i32>() / self.int_vector.len() as i32} else {0};
    }
}
