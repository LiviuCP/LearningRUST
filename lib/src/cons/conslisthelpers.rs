use crate::cons::conslists::ConsList;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub fn create_rc_im_list<T>() -> ConsList<Rc<RefCell<T>>>
where
    T: Debug,
    T: PartialEq,
{
    return ConsList::<Rc<RefCell<T>>>::create();
}

pub fn create_rc_im_list_from_vec<T>(arr: &Vec<T>) -> ConsList<Rc<RefCell<T>>>
where
    T: Debug + Clone,
    T: PartialEq,
{
    let mut new_arr = Vec::<Rc<RefCell<T>>>::new();

    for item in arr.iter() {
        let new_item = Rc::new(RefCell::<T>::new(item.clone()));
        new_arr.push(new_item);
    }

    return ConsList::<Rc<RefCell<T>>>::create_from_vec(&new_arr);
}

pub fn values<T>(im_arr: &Vec<Rc<RefCell<T>>>) -> Vec<T>
where
    T: Clone,
{
    let mut new_arr = Vec::<T>::new();

    for item in im_arr.iter() {
        new_arr.push(item.borrow().clone());
    }

    return new_arr;
}

pub fn create_rc_ro_list<T>() -> ConsList<Rc<T>>
where
    T: Debug,
    T: PartialEq,
{
    return ConsList::<Rc<T>>::create();
}

pub fn create_rc_ro_list_from_vec<T>(arr: &Vec<T>) -> ConsList<Rc<T>>
where
    T: Debug + Clone,
    T: PartialEq,
{
    let mut new_arr = Vec::<Rc<T>>::new();

    for item in arr.iter() {
        let new_item = Rc::new(item.clone());
        new_arr.push(new_item);
    }

    return ConsList::<Rc<T>>::create_from_vec(&new_arr);
}

pub fn ro_values<T>(im_arr: &Vec<Rc<T>>) -> Vec<T>
where
    T: Clone,
{
    let mut new_arr = Vec::<T>::new();

    for item in im_arr.iter() {
        new_arr.push((**item).clone());
    }

    return new_arr;
}
