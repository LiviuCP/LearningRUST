/*
TODO: create (mut) iterators
*/

enum ConsList<T> {
    ConsValue(T, Box<ConsList<T>>),
    Nil
}

impl<T> ConsList<T> {
    pub fn create_from_vec(arr: &Vec<T>) -> ConsList<T> {
	ConsList::<T>::Nil
    }

    pub fn append(&mut self, value: &T) {
    }

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

//    pub fn tail(&self) -> &Box<ConsList<T>> {
	// TODO
//    }
}
