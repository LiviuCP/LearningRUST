/*
TODO: create (mut) iterators
*/

#[derive(PartialEq, Debug, Clone)]
pub enum ConsList<T> {
    ConsValue(T, Box<ConsList<T>>),
    Nil
}

impl<T: Copy> ConsList<T> {
    pub fn create_from_vec(arr: &Vec<T>) -> ConsList<T> {
	let mut result = ConsList::<T>::Nil;
	for val in arr.iter().rev() {
	    result = ConsList::ConsValue(*val, Box::new(result));
	}
	result
    }

    pub fn prepend(&mut self, value: &T) {
	*self = ConsList::ConsValue(*value, Box::new(self.clone()));
    }
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
}
