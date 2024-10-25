#[cfg(test)]

use learn_rust_lib::conslists::{ConsList};

#[test]
pub fn test_create() {
    let list = ConsList::<i32>::create();
    assert!(list.content() == Vec::new() && list.empty());
}

#[test]
pub fn test_create_from_vec() {
    let mut list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    assert!(list.content() == vec![-4, 8, -3, -3, 5, 0, 2, 1] && list.size() == 8);

    list = ConsList::create_from_vec(&vec![2, 5]);
    assert!(list.content() == vec![2, 5] && list.size() == 2);

    list = ConsList::create_from_vec(&vec![-3]);
    assert!(list.content() == vec![-3] && list.size() == 1);

    list = ConsList::create_from_vec(&Vec::new());
    assert!(list.content() == Vec::new() && list.empty());
}

#[test]
pub fn test_im_iter_read() {
    let mut list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    let mut list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), -4);

    list_iter.next();
    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap().borrow(), 5);

    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap().borrow(), 1);
    assert_eq!(list_iter.next(), None);

    list = ConsList::create_from_vec(&vec![-2, 5]);
    list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), -2);
    assert_eq!(*list_iter.next().unwrap().borrow(), 5);
    assert!(list_iter.next() == None);

    list = ConsList::create_from_vec(&vec![-3]);
    list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), -3);
    assert!(list_iter.next() == None);

    list = ConsList::create();
    list_iter = list.im_iter();

    assert!(list_iter.next() == None);
}

#[test]
pub fn test_im_iter_write() {
    let list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    let mut list_iter = list.im_iter();

    *list_iter.next().unwrap().borrow_mut() = 7;

    list_iter.next();
    list_iter.next();

    *list_iter.next().unwrap().borrow_mut() = 9;

    list_iter.next();
    list_iter.next();
    list_iter.next();

    *list_iter.next().unwrap().borrow_mut() = -8;

    assert!(list.content() == vec![7, 8, -3, 9, 5, 0, 2, -8] && list.size() == 8);
    assert_eq!(list_iter.next(), None);
}

#[test]
pub fn test_im_iter_read_write() {
    let list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);

    for item in list.im_iter() {
	*item.borrow_mut() += 2;
    }

    assert!(list.content() == vec![-2, 10, -1, -1, 7, 2, 4, 3] && list.size() == 8);

    for item in list.im_iter() {	
	// variable required as it is not allowed to borrow twice in the same statement
	let temp = *item.borrow();
	
	*item.borrow_mut() = temp / 2;
    }

    assert!(list.content() == vec![-1, 5, 0, 0, 3, 1, 2, 1] && list.size() == 8);
}

#[test]
pub fn test_im_iter_with_lambda() {
    let list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    let arr: Vec<i32> = list.im_iter().map(|item| *item.borrow() * 2).collect();

    assert_eq!(arr, vec![-8, 16, -6, -6, 10, 0, 4, 2]);
    assert!(list.content() == vec![-4, 8, -3, -3, 5, 0, 2, 1] && list.size() == 8);

    list.im_iter().for_each(|item| *item.borrow_mut() %= 2);

    assert!(list.content() == vec![0, 0, -1, -1, 1, 0, 0, 1] && list.size() == 8);
}

#[test]
pub fn test_prepend() {
    let mut list = ConsList::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    let mut value = -4;
    list.prepend(&value);

    assert!(list.content() == vec![-4, 8, -3, -3, 5, 0, 2, 1] && list.size() == 8);

    list = ConsList::create_from_vec(&vec![5]);
    value = 2;
    list.prepend(&value);

    assert!(list.content() == vec![2, 5] && list.size() == 2);

    list = ConsList::create();
    value = -3;
    list.prepend(&value);

    assert!(list.content() == vec![-3] && list.size() == 1);
}

#[test]
pub fn test_append() {
    let mut list = ConsList::create_from_vec(&vec![8, -3, -3, 5, 0, 2, 1]);
    let mut value = -4;
    list.append(&value);

    assert!(list.content() == vec![8, -3, -3, 5, 0, 2, 1, -4] && list.size() == 8);

    list = ConsList::create_from_vec(&vec![5]);
    value = 2;
    list.append(&value);

    assert!(list.content() == vec![5, 2] && list.size() == 2);

    list = ConsList::create();
    value = -3;
    list.append(&value);

    assert!(list.content() == vec![-3] && list.size() == 1);
}

#[test]
pub fn test_reverse() {
    let mut list = ConsList::create_from_vec(&vec![-4, 8, -3, -3, 5, 0, 2, 1]);
    list.reverse();

    assert!(list.content() == vec![1, 2, 0, 5, -3, -3, 8, -4] && list.size() == 8);

    list = ConsList::create_from_vec(&vec![2, 5]);
    list.reverse();

    assert!(list.content() == vec![5, 2] && list.size() == 2);

    list = ConsList::create_from_vec(&vec![-3]);
    list.reverse();

    assert!(list.content() == vec![-3] && list.size() == 1);

    list = ConsList::create();
    list.reverse();

    assert!(list.content() == Vec::new() && list.empty());
}

#[test]
pub fn test_merge() {
    let mut first_list = ConsList::create();
    let mut second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == Vec::new() && first_list.empty());
    assert!(second_list.content() == Vec::new()  && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![-3]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![-3] && first_list.size() == 1);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create();
    second_list = ConsList::create_from_vec(&vec![-3]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![-3] && first_list.size() == 1);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![5, -4]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![5, -4] && first_list.size() == 2);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create();
    second_list = ConsList::create_from_vec(&vec![5, -4]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![5, -4] && first_list.size() == 2);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![-8, 1]);
    second_list = ConsList::create_from_vec(&vec![-4]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![-8, 1, -4] && first_list.size() == 3);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![-4]);
    second_list = ConsList::create_from_vec(&vec![-8, 1]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![-4, -8, 1] && first_list.size() == 3);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![2, -5, 4, 3, 4]);
    second_list = ConsList::create_from_vec(&vec![9, 1, 1, 8]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![2, -5, 4, 3, 4, 9, 1, 1, 8] && first_list.size() == 9);
    assert!(second_list.content() == Vec::new() && second_list.empty());

    first_list = ConsList::create_from_vec(&vec![9, 1, 1, 8]);
    second_list = ConsList::create_from_vec(&vec![2, -5, 4, 3, 4]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec![9, 1, 1, 8, 2, -5, 4, 3, 4] && first_list.size() == 9);
    assert!(second_list.content() == Vec::new() && second_list.empty());
}

#[test]
pub fn test_head() {
    let mut list = ConsList::create_from_vec(&vec![2, 5, -3, 4]);
    assert_ne!(list.head(), None);

    let mut read_head = list.head();
    assert_eq!(*read_head.unwrap().borrow(), 2);

    let mut write_head = list.head();
    *write_head.unwrap().borrow_mut() = -10;

    assert!(list.content() == vec![-10, 5, -3, 4] && list.size() == 4);

    list = ConsList::create_from_vec(&vec![-8]);
    assert_ne!(list.head(), None);

    read_head = list.head();
    assert_eq!(*read_head.unwrap().borrow(), -8);

    write_head = list.head();
    *write_head.unwrap().borrow_mut() = 9;

    assert!(list.content() == vec![9] && list.size() == 1);

    list = ConsList::create();
    assert_eq!(list.head(), None);
}

#[test]
pub fn test_tail() {
    let mut list = ConsList::create_from_vec(&vec![2, 5, -3, 4]);
    assert_ne!(list.tail(), None);

    let mut read_tail = list.tail();
    assert_eq!(*read_tail.unwrap().borrow(), 4);

    let mut write_tail = list.tail();
    *write_tail.unwrap().borrow_mut() = 9;

    assert!(list.content() == vec![2, 5, -3, 9] && list.size() == 4);

    list = ConsList::create_from_vec(&vec![-8]);
    assert_ne!(list.tail(), None);

    read_tail = list.tail();
    assert_eq!(*read_tail.unwrap().borrow(), -8);

    write_tail = list.tail();
    *write_tail.unwrap().borrow_mut() = -10;

    assert!(list.content() == vec![-10] && list.size() == 1);

    list = ConsList::create();
    assert_eq!(list.tail(), None);
}

#[test]
pub fn test_clear() {
    let mut list = ConsList::create_from_vec(&vec![2, 5, -3, 4]);
    list.clear();

    assert!(list.content() == Vec::new() && list.empty());

    list = ConsList::create();
    list.clear();

    assert!(list.content() == Vec::new() && list.empty());
}
