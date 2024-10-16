/*
TODO: create (mut) iterators
*/

pub struct ConsWrapper<T> {
    list: Box<ConsList<T>>,
    count: usize
}

impl<T: Copy> ConsWrapper<T> {
    pub fn create_from_vec(arr: &Vec<T>) -> ConsWrapper<T> {
	let mut result = ConsWrapper::<T>{list: Box::new(ConsList::<T>::Nil), count: 0};

	for val in arr.iter().rev() {
	    result.list = Box::new(ConsList::ConsValue(*val, result.list));
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

#[derive(PartialEq, Debug, Clone)]
pub enum ConsList<T> {
    ConsValue(T, Box<ConsList<T>>),
    Nil
}

impl<T: Copy> ConsList<T> {
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
