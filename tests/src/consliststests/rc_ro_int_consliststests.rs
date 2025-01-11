#[cfg(test)]
use learn_rust_lib::cons::{
    conslisthelpers::{create_rc_ro_list, create_rc_ro_list_from_vec, ro_values},
    conslists::ConsList,
    InvalidIndex,
};
use std::rc::Rc;

#[test]
pub fn test_create() {
    let list = create_rc_ro_list::<i32>();
    assert!(ro_values(&list.content()) == Vec::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_create_from_vec() {
    let mut list = create_rc_ro_list_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    assert!(
        ro_values(&list.content()) == vec![-4, 8, -3, -3, 5, 0, 2, 1]
            && list.size() == 8
            && !list.empty()
    );

    list = create_rc_ro_list_from_vec(&vec![2, 5]);
    assert!(ro_values(&list.content()) == vec![2, 5] && list.size() == 2 && !list.empty());

    list = create_rc_ro_list_from_vec(&vec![-3]);
    assert!(ro_values(&list.content()) == vec![-3] && list.size() == 1 && !list.empty());

    list = create_rc_ro_list_from_vec(&Vec::new());
    assert!(ro_values(&list.content()) == Vec::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_iter() {
    let mut list = create_rc_ro_list_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    let mut list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), -4);

    list_iter.next();
    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap(), 5);

    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap(), 1);
    assert_eq!(list_iter.next(), None);

    list = create_rc_ro_list_from_vec(&vec![-2, 5]);
    list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), -2);
    assert_eq!(*list_iter.next().unwrap(), 5);
    assert!(list_iter.next() == None);

    list = create_rc_ro_list_from_vec(&vec![-3]);
    list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), -3);
    assert!(list_iter.next() == None);

    list = ConsList::create();
    list_iter = list.iter();

    assert!(list_iter.next() == None);
}

#[test]
pub fn test_iter_with_lambda() {
    let list = create_rc_ro_list_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    let arr: Vec<i32> = list.iter().map(|item| *item * 2).collect();

    assert_eq!(arr, vec![-8, 16, -6, -6, 10, 0, 4, 2]);
    assert!(ro_values(&list.content()) == vec![-4, 8, -3, -3, 5, 0, 2, 1] && list.size() == 8);
}

#[test]
pub fn test_push_front() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    let mut value = Rc::new(-4);
    list.push_front(&value);

    assert!(ro_values(&list.content()) == vec![-4, 8, -3, -3, 5, 0, 2, 1] && list.size() == 8);

    list = create_rc_ro_list_from_vec(&vec![5]);
    value = Rc::new(2);
    list.push_front(&value);

    assert!(ro_values(&list.content()) == vec![2, 5] && list.size() == 2);

    list = ConsList::create();
    value = Rc::new(-3);
    list.push_front(&value);

    assert!(ro_values(&list.content()) == vec![-3] && list.size() == 1);
}

#[test]
pub fn test_pop_front() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    let mut result = list.pop_front();

    assert!(
        *result.unwrap() == 8
            && ro_values(&list.content()) == vec![-3, -3, 5, 0, 2, 1, -4]
            && list.size() == 7
    );

    list = create_rc_ro_list_from_vec(&vec![3, 0]);
    result = list.pop_front();

    assert!(*result.unwrap() == 3 && ro_values(&list.content()) == vec![0] && list.size() == 1);

    list = create_rc_ro_list_from_vec(&vec![-2]);
    result = list.pop_front();

    assert!(*result.unwrap() == -2 && list.empty());

    list = ConsList::create();
    result = list.pop_front();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_push_back() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    let mut value = Rc::new(-4);
    list.push_back(&value);

    assert!(ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1, -4] && list.size() == 8);

    list = create_rc_ro_list_from_vec(&vec![5]);
    value = Rc::new(2);
    list.push_back(&value);

    assert!(ro_values(&list.content()) == vec![5, 2] && list.size() == 2);

    list = ConsList::create();
    value = Rc::new(-3);
    list.push_back(&value);

    assert!(ro_values(&list.content()) == vec![-3] && list.size() == 1);
}

#[test]
pub fn test_pop_back() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    let mut result = list.pop_back();

    assert!(
        *result.unwrap() == -4
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1]
            && list.size() == 7
    );

    list = create_rc_ro_list_from_vec(&vec![3, 0]);
    result = list.pop_back();

    assert!(*result.unwrap() == 0 && ro_values(&list.content()) == vec![3] && list.size() == 1);

    list = create_rc_ro_list_from_vec(&vec![-2]);
    result = list.pop_back();

    assert!(*result.unwrap() == -2 && list.empty());

    list = ConsList::create();
    result = list.pop_back();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_insert() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    let mut value = Rc::new(9);
    let mut result = list.insert(&value, 0);

    assert!(
        result == Ok(0)
            && ro_values(&list.content()) == vec![9, 8, -3, -3, 5, 0, 2, 1, -4]
            && list.size() == 9
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.insert(&value, 2);

    assert!(
        result == Ok(2)
            && ro_values(&list.content()) == vec![8, -3, 9, -3, 5, 0, 2, 1, -4]
            && list.size() == 9
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.insert(&value, 7);

    assert!(
        result == Ok(7)
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1, 9, -4]
            && list.size() == 9
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.insert(&value, 8);

    assert!(
        result == Ok(8)
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1, -4, 9]
            && list.size() == 9
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.insert(&value, 9);

    assert!(
        result == Err(InvalidIndex)
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1, -4]
            && list.size() == 8
    );

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    value = Rc::new(8);
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && ro_values(&list.content()) == vec![8, 2, 4, -3] && list.size() == 4);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && ro_values(&list.content()) == vec![2, 8, 4, -3] && list.size() == 4);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && ro_values(&list.content()) == vec![2, 4, 8, -3] && list.size() == 4);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.insert(&value, 3);

    assert!(result == Ok(3) && ro_values(&list.content()) == vec![2, 4, -3, 8] && list.size() == 4);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.insert(&value, 4);

    assert!(
        result == Err(InvalidIndex)
            && ro_values(&list.content()) == vec![2, 4, -3]
            && list.size() == 3
    );

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    value = Rc::new(-7);
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && ro_values(&list.content()) == vec![-7, 9, 5] && list.size() == 3);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && ro_values(&list.content()) == vec![9, -7, 5] && list.size() == 3);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && ro_values(&list.content()) == vec![9, 5, -7] && list.size() == 3);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.insert(&value, 3);

    assert!(
        result == Err(InvalidIndex) && ro_values(&list.content()) == vec![9, 5] && list.size() == 2
    );

    list = create_rc_ro_list_from_vec(&vec![-4]);
    value = Rc::new(5);
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && ro_values(&list.content()) == vec![5, -4] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![-4]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && ro_values(&list.content()) == vec![-4, 5] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![-4]);
    result = list.insert(&value, 2);

    assert!(
        result == Err(InvalidIndex) && ro_values(&list.content()) == vec![-4] && list.size() == 1
    );

    list = ConsList::create();
    value = Rc::new(-3);
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && ro_values(&list.content()) == vec![-3] && list.size() == 1);

    list = ConsList::create();
    result = list.insert(&value, 1);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_remove() {
    let mut list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    let mut result = list.remove(0);

    assert!(
        *result.unwrap() == 8
            && ro_values(&list.content()) == vec![-3, -3, 5, 0, 2, 1, -4]
            && list.size() == 7
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.remove(4);

    assert!(
        *result.unwrap() == 0
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 2, 1, -4]
            && list.size() == 7
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.remove(7);

    assert!(
        *result.unwrap() == -4
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1]
            && list.size() == 7
    );

    list = create_rc_ro_list_from_vec(&vec![8, -3, -3, 5, 0, 2, 1, -4]);
    result = list.remove(8);

    assert!(
        result == Err(InvalidIndex)
            && ro_values(&list.content()) == vec![8, -3, -3, 5, 0, 2, 1, -4]
            && list.size() == 8
    );

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.remove(0);

    assert!(*result.unwrap() == 2 && ro_values(&list.content()) == vec![4, -3] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.remove(1);

    assert!(*result.unwrap() == 4 && ro_values(&list.content()) == vec![2, -3] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.remove(2);

    assert!(*result.unwrap() == -3 && ro_values(&list.content()) == vec![2, 4] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);
    result = list.remove(3);

    assert!(
        result == Err(InvalidIndex)
            && ro_values(&list.content()) == vec![2, 4, -3]
            && list.size() == 3
    );

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.remove(0);

    assert!(*result.unwrap() == 9 && ro_values(&list.content()) == vec![5] && list.size() == 1);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.remove(1);

    assert!(*result.unwrap() == 5 && ro_values(&list.content()) == vec![9] && list.size() == 1);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);
    result = list.remove(2);

    assert!(
        result == Err(InvalidIndex) && ro_values(&list.content()) == vec![9, 5] && list.size() == 2
    );

    list = create_rc_ro_list_from_vec(&vec![-4]);
    result = list.remove(0);

    assert!(*result.unwrap() == -4 && list.empty());

    list = create_rc_ro_list_from_vec(&vec![-4]);
    result = list.remove(1);

    assert!(
        result == Err(InvalidIndex) && ro_values(&list.content()) == vec![-4] && list.size() == 1
    );

    list = ConsList::create();
    result = list.remove(0);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_reverse() {
    let mut list = create_rc_ro_list_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    list.reverse();

    assert!(ro_values(&list.content()) == vec![1, 2, 0, 5, -3, -3, 8, -4] && list.size() == 8);

    list = create_rc_ro_list_from_vec(&vec![2, 5]);
    list.reverse();

    assert!(ro_values(&list.content()) == vec![5, 2] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![-3]);
    list.reverse();

    assert!(ro_values(&list.content()) == vec![-3] && list.size() == 1);

    list = ConsList::create();
    list.reverse();

    assert!(list.empty());
}

#[test]
pub fn test_merge() {
    let mut first_list = ConsList::create();
    let mut second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.empty());
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![-3]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![-3] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = ConsList::create();
    second_list = create_rc_ro_list_from_vec(&vec![-3]);
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![-3] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![5, -4]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![5, -4] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = ConsList::create();
    second_list = create_rc_ro_list_from_vec(&vec![5, -4]);
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![5, -4] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![-8, 1]);
    second_list = create_rc_ro_list_from_vec(&vec![-4]);
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![-8, 1, -4] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![-4]);
    second_list = create_rc_ro_list_from_vec(&vec![-8, 1]);
    first_list.merge(&mut second_list);

    assert!(ro_values(&first_list.content()) == vec![-4, -8, 1] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4]);
    second_list = create_rc_ro_list_from_vec(&vec![9, 1, 1, 8]);
    first_list.merge(&mut second_list);

    assert!(
        ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4, 9, 1, 1, 8]
            && first_list.size() == 9
    );
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![9, 1, 1, 8]);
    second_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4]);
    first_list.merge(&mut second_list);

    assert!(
        ro_values(&first_list.content()) == vec![9, 1, 1, 8, 2, -5, 4, 3, 4]
            && first_list.size() == 9
    );
    assert!(second_list.empty());
}

#[test]
pub fn test_split() {
    let mut first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    let mut second_list = ConsList::create();
    let mut result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(
        ro_values(&second_list.content()) == vec![2, -5, 4, 3, 4, 9, 1, 1, 8]
            && second_list.size() == 9
    );

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(ro_values(&first_list.content()) == vec![2] && first_list.size() == 1);
    assert!(
        ro_values(&second_list.content()) == vec![-5, 4, 3, 4, 9, 1, 1, 8]
            && second_list.size() == 8
    );

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 5);

    assert_eq!(result, Ok(5));
    assert!(ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4] && first_list.size() == 5);
    assert!(ro_values(&second_list.content()) == vec![9, 1, 1, 8] && second_list.size() == 4);

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 7);

    assert_eq!(result, Ok(7));
    assert!(
        ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4, 9, 1] && first_list.size() == 7
    );
    assert!(ro_values(&second_list.content()) == vec![1, 8] && second_list.size() == 2);

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 8);

    assert_eq!(result, Ok(8));
    assert!(
        ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4, 9, 1, 1] && first_list.size() == 8
    );
    assert!(ro_values(&second_list.content()) == vec![8] && second_list.size() == 1);

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(
        ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4, 9, 1, 1, 8]
            && first_list.size() == 9
    );
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![9, -7, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(ro_values(&second_list.content()) == vec![9, -7, 8] && second_list.size() == 3);

    first_list = create_rc_ro_list_from_vec(&vec![9, -7, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(ro_values(&first_list.content()) == vec![9] && first_list.size() == 1);
    assert!(ro_values(&second_list.content()) == vec![-7, 8] && second_list.size() == 2);

    first_list = create_rc_ro_list_from_vec(&vec![9, -7, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Ok(2));
    assert!(ro_values(&first_list.content()) == vec![9, -7] && first_list.size() == 2);
    assert!(ro_values(&second_list.content()) == vec![8] && second_list.size() == 1);

    first_list = create_rc_ro_list_from_vec(&vec![9, -7, 8]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Err(InvalidIndex));
    assert!(ro_values(&first_list.content()) == vec![9, -7, 8] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![6, -9]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(ro_values(&second_list.content()) == vec![6, -9] && second_list.size() == 2);

    first_list = create_rc_ro_list_from_vec(&vec![6, -9]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(ro_values(&first_list.content()) == vec![6] && first_list.size() == 1);
    assert!(ro_values(&second_list.content()) == vec![-9] && second_list.size() == 1);

    first_list = create_rc_ro_list_from_vec(&vec![6, -9]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Err(InvalidIndex));
    assert!(ro_values(&first_list.content()) == vec![6, -9] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = create_rc_ro_list_from_vec(&vec![5]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(ro_values(&second_list.content()) == vec![5] && second_list.size() == 1);

    first_list = create_rc_ro_list_from_vec(&vec![5]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Err(InvalidIndex));
    assert!(ro_values(&first_list.content()) == vec![5] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = ConsList::create();
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.empty());
    assert!(second_list.empty());

    // additional tests, second list initially not empty
    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = create_rc_ro_list_from_vec(&vec![4, 0, -2, -4]);
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Ok(3));
    assert!(ro_values(&first_list.content()) == vec![2, -5, 4] && first_list.size() == 3);
    assert!(ro_values(&second_list.content()) == vec![3, 4, 9, 1, 1, 8] && second_list.size() == 6);

    first_list = create_rc_ro_list_from_vec(&vec![2, -5, 4, 3, 4, 9, 1, 1, 8]);
    second_list = create_rc_ro_list_from_vec(&vec![4, 0, -2, -4]);
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(
        ro_values(&first_list.content()) == vec![2, -5, 4, 3, 4, 9, 1, 1, 8]
            && first_list.size() == 9
    );
    assert!(ro_values(&second_list.content()) == vec![4, 0, -2, -4] && second_list.size() == 4);
}

#[test]
pub fn test_head() {
    let mut list = create_rc_ro_list_from_vec(&vec![2, 5, -3, 4]);
    assert_ne!(list.head(), None);

    let mut read_head = list.head();
    assert_eq!(*read_head.unwrap(), 2);

    list = create_rc_ro_list_from_vec(&vec![-8]);
    assert_ne!(list.head(), None);

    read_head = list.head();
    assert_eq!(*read_head.unwrap(), -8);

    list = ConsList::create();
    assert_eq!(list.head(), None);
}

#[test]
pub fn test_tail() {
    let mut list = create_rc_ro_list_from_vec(&vec![2, 5, -3, 4]);
    assert_ne!(list.tail(), None);

    let mut read_tail = list.tail();
    assert_eq!(*read_tail.unwrap(), 4);

    list = create_rc_ro_list_from_vec(&vec![-8]);
    assert_ne!(list.tail(), None);

    read_tail = list.tail();
    assert_eq!(*read_tail.unwrap(), -8);

    list = ConsList::create();
    assert_eq!(list.tail(), None);
}

#[test]
pub fn test_at() {
    let mut list = create_rc_ro_list_from_vec(&vec![9, -3, -3, 5, 0, 2, 1, -4]);

    assert_eq!(*list.at(0).unwrap(), 9);
    assert_eq!(*list.at(4).unwrap(), 0);
    assert_eq!(*list.at(7).unwrap(), -4);
    assert_eq!(list.at(8), Err(InvalidIndex));
    assert!(ro_values(&list.content()) == vec![9, -3, -3, 5, 0, 2, 1, -4] && list.size() == 8);

    list = create_rc_ro_list_from_vec(&vec![2, 4, -3]);

    assert_eq!(*list.at(0).unwrap(), 2);
    assert_eq!(*list.at(1).unwrap(), 4);
    assert_eq!(*list.at(2).unwrap(), -3);
    assert_eq!(list.at(3), Err(InvalidIndex));
    assert!(ro_values(&list.content()) == vec![2, 4, -3] && list.size() == 3);

    list = create_rc_ro_list_from_vec(&vec![9, 5]);

    assert_eq!(*list.at(0).unwrap(), 9);
    assert_eq!(*list.at(1).unwrap(), 5);
    assert_eq!(list.at(2), Err(InvalidIndex));
    assert!(ro_values(&list.content()) == vec![9, 5] && list.size() == 2);

    list = create_rc_ro_list_from_vec(&vec![-4]);

    assert_eq!(*list.at(0).unwrap(), -4);
    assert_eq!(list.at(1), Err(InvalidIndex));
    assert!(ro_values(&list.content()) == vec![-4] && list.size() == 1);

    list = ConsList::create();

    assert_eq!(list.at(0), Err(InvalidIndex));
    assert!(list.empty());
}

#[test]
pub fn test_clear() {
    let mut list = create_rc_ro_list_from_vec(&vec![2, 5, -3, 4]);
    list.clear();

    assert!(list.empty());

    list = ConsList::create();
    list.clear();

    assert!(list.empty());
}
