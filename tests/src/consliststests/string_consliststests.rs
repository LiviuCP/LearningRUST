#[cfg(test)]

use learn_rust_lib::conslists::{ConsList, InvalidIndex};

#[test]
pub fn test_create() {
    let list = ConsList::<String>::create();
    assert!(list.content() == Vec::<String>::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_create_from_vec() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8 && !list.empty());

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    assert!(list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2 && !list.empty());

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1 && !list.empty());

    list = ConsList::create_from_vec(&Vec::<String>::new());
    assert!(list.content() == Vec::<String>::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_im_iter_read() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), "aB".to_string());

    list_iter.next();
    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap().borrow(), "z/x".to_string());

    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap().borrow(), "j1".to_string());
    assert_eq!(list_iter.next(), None);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), "b2".to_string());
    assert_eq!(*list_iter.next().unwrap().borrow(), "d4".to_string());
    assert!(list_iter.next() == None);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    list_iter = list.im_iter();

    assert_eq!(*list_iter.next().unwrap().borrow(), "bbq".to_string());
    assert!(list_iter.next() == None);

    list = ConsList::<String>::create();
    list_iter = list.im_iter();

    assert!(list_iter.next() == None);
}

#[test]
pub fn test_im_iter_write() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut list_iter = list.im_iter();

    *list_iter.next().unwrap().borrow_mut() = "first_val".to_string();

    list_iter.next();
    list_iter.next();

    *list_iter.next().unwrap().borrow_mut() = "second_val".to_string();

    list_iter.next();
    list_iter.next();
    list_iter.next();

    *list_iter.next().unwrap().borrow_mut() = "third_val".to_string();

    assert!(list.content() == vec!["first_val".to_string(), "cD".to_string(), "i_j".to_string(), "second_val".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "third_val".to_string()] && list.size() == 8);
    assert_eq!(list_iter.next(), None);

    // additional test, string specific
    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    list_iter = list.im_iter();

    list_iter.next();
    list_iter.next();
    list_iter.next();
    list_iter.next().unwrap().borrow_mut().push('1');

    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh1".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);
}

#[test]
pub fn test_im_iter_read_write() {
    let list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);

    for item in list.im_iter() {
	item.borrow_mut().push_str("_add");
    }

    assert!(list.content() == vec!["aB_add".to_string(), "cD_add".to_string(), "i_j_add".to_string(), "gh_add".to_string(), "z/x_add".to_string(), "QQ_add".to_string(), "2_b_add".to_string(), "j1_add".to_string()] && list.size() == 8);

    for item in list.im_iter() {	
	// variable required as it is not allowed to borrow twice in the same statement
	let temp = item.borrow().clone();
	
	item.borrow_mut().push_str(&temp);
    }

    assert!(list.content() == vec!["aB_addaB_add".to_string(), "cD_addcD_add".to_string(), "i_j_addi_j_add".to_string(), "gh_addgh_add".to_string(), "z/x_addz/x_add".to_string(), "QQ_addQQ_add".to_string(), "2_b_add2_b_add".to_string(), "j1_addj1_add".to_string()] && list.size() == 8);
}

#[test]
pub fn test_im_iter_with_lambda() {
    let list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let arr: Vec<String> = list.im_iter().map(|item| {let mut temp = item.borrow().clone(); temp.push_str("_suff"); temp}).collect();

    assert_eq!(arr, vec!["aB_suff".to_string(), "cD_suff".to_string(), "i_j_suff".to_string(), "gh_suff".to_string(), "z/x_suff".to_string(), "QQ_suff".to_string(), "2_b_suff".to_string(), "j1_suff".to_string()]);
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list.im_iter().for_each(|item| item.borrow_mut().push_str("_test"));

    assert!(list.content() == vec!["aB_test".to_string(), "cD_test".to_string(), "i_j_test".to_string(), "gh_test".to_string(), "z/x_test".to_string(), "QQ_test".to_string(), "2_b_test".to_string(), "j1_test".to_string()] && list.size() == 8);
}

#[test]
pub fn test_push_front() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()]);
    let mut value = "new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["new_val".to_string(), "aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "second_new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["second_new_val".to_string(), "bbq".to_string()] && list.size() == 2);

    list = ConsList::create();
    value = "third_new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["third_new_val".to_string()] && list.size() == 1);
}

#[test]
pub fn test_pop_front() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.pop_front();

    assert!(*result.unwrap().borrow() == "aB".to_string() && list.content() == vec!["cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.pop_front();

    assert!(*result.unwrap().borrow() == "b2".to_string() && list.content() == vec!["d4".to_string()] && list.size() == 1);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.pop_front();

    assert!(*result.unwrap().borrow() == "bbq".to_string() && list.empty());

    list = ConsList::<String>::create();
    result = list.pop_front();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_push_back() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()]);
    let mut value = "new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "new_val".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "second_new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["bbq".to_string(), "second_new_val".to_string()] && list.size() == 2);

    list = ConsList::create();
    value = "third_new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["third_new_val".to_string()] && list.size() == 1);
}

#[test]
pub fn test_pop_back() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.pop_back();

    assert!(*result.unwrap().borrow() == "j1".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 7);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.pop_back();

    assert!(*result.unwrap().borrow() == "d4".to_string() && list.content() == vec!["b2".to_string()] && list.size() == 1);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.pop_back();

    assert!(*result.unwrap().borrow() == "bbq".to_string() && list.empty());

    list = ConsList::<String>::create();
    result = list.pop_back();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_insert() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut value = "first_val".to_string();
    let mut result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["first_val".to_string(), "aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 9);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["aB".to_string(), "cD".to_string(), "first_val".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 9);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 7);

    assert!(result == Ok(7) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "first_val".to_string(), "j1".to_string()] && list.size() == 9);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 8);

    assert!(result == Ok(8) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string(), "first_val".to_string()] && list.size() == 9);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 9);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    value = "second_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["second_val".to_string(), "af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["af".to_string(), "second_val".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["af".to_string(), "d_8".to_string(), "second_val".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 3);

    assert!(result == Ok(3) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string(), "second_val".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 4);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    value = "third_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["third_val".to_string(), "b2".to_string(), "d4".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["b2".to_string(), "third_val".to_string(), "d4".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["b2".to_string(), "d4".to_string(), "third_val".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 3);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "fourth_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["fourth_val".to_string(), "bbq".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["bbq".to_string(), "fourth_val".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = ConsList::create();
    value = "fifth_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["fifth_val".to_string()] && list.size() == 1);

    list = ConsList::create();
    result = list.insert(&value, 1);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_remove() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.remove(0);

    assert!(*result.unwrap().borrow() == "aB".to_string() && list.content() == vec!["cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(4);

    assert!(*result.unwrap().borrow() == "z/x".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(7);

    assert!(*result.unwrap().borrow() == "j1".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 7);

    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(8);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap().borrow() == "af".to_string() && list.content() == vec!["d_8".to_string(), "Z-4".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(1);

    assert!(*result.unwrap().borrow() == "d_8".to_string() && list.content() == vec!["af".to_string(), "Z-4".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(2);

    assert!(*result.unwrap().borrow() == "Z-4".to_string() && list.content() == vec!["af".to_string(), "d_8".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(3);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap().borrow() == "b2".to_string() && list.content() == vec!["d4".to_string()] && list.size() == 1);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(1);

    assert!(*result.unwrap().borrow() == "d4".to_string() && list.content() == vec!["b2".to_string()] && list.size() == 1);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(2);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap().borrow() == "bbq".to_string() && list.empty());

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.remove(1);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = ConsList::<String>::create();
    result = list.remove(0);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_reverse() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["j1".to_string(), "2_b".to_string(), "QQ".to_string(), "z/x".to_string(), "gh".to_string(), "i_j".to_string(), "cD".to_string(), "aB".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["d4".to_string(), "b2".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = ConsList::<String>::create();
    list.reverse();

    assert!(list.empty());
}

#[test]
pub fn test_merge() {
    let mut first_list = ConsList::<String>::create();
    let mut second_list = ConsList::<String>::create();
    first_list.merge(&mut second_list);

    assert!(first_list.empty());
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = ConsList::create();
    second_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = ConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = ConsList::create();
    second_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string(), "b2".to_string(), "d4".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string(), "a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());
}

#[test]
pub fn test_split() {
    let mut first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    let mut second_list = ConsList::create();
    let mut result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 9);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["a1".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 8);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 5);

    assert_eq!(result, Ok(5));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()] && first_list.size() == 5);
    assert!(second_list.content() == vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 4);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 7);

    assert_eq!(result, Ok(7));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string()] && first_list.size() == 7);
    assert!(second_list.content() == vec!["h8".to_string(), "i9".to_string()] && second_list.size() == 2);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 8);

    assert_eq!(result, Ok(8));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string()] && first_list.size() == 8);
    assert!(second_list.content() == vec!["i9".to_string()] && second_list.size() == 1);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && second_list.size() == 3);

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["b2".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["d4".to_string(), "bbq".to_string()] && second_list.size() == 2);

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Ok(2));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.content() == vec!["bbq".to_string()] && second_list.size() == 1);

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["b2".to_string(), "d4".to_string()] && second_list.size() == 2);

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["b2".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["d4".to_string()] && second_list.size() == 1);

    first_list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["bbq".to_string()] && second_list.size() == 1);

    first_list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = ConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = ConsList::<String>::create();
    second_list = ConsList::<String>::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.empty());
    assert!(second_list.empty());

    // additional tests, second list initially not empty
    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()]);
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Ok(3));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string()] && first_list.size() == 3);
    assert!(second_list.content() == vec!["d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 6);

    first_list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = ConsList::create_from_vec(&vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()]);
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.content() == vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()] && second_list.size() == 4);
}

#[test]
pub fn test_head() {
    let mut list = ConsList::create_from_vec(&vec!["a_1".to_string(), "b_2".to_string(), "c_3".to_string(), "d_4".to_string()]);
    assert_ne!(list.head(), None);

    let mut read_head = list.head();
    assert_eq!(*read_head.unwrap().borrow(), "a_1".to_string());

    let mut write_head = list.head();
    *write_head.unwrap().borrow_mut() = "new_val".to_string();

    assert!(list.content() == vec!["new_val".to_string(), "b_2".to_string(), "c_3".to_string(), "d_4".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert_ne!(list.head(), None);

    read_head = list.head();
    assert_eq!(*read_head.unwrap().borrow(), "bbq".to_string());

    write_head = list.head();
    *write_head.unwrap().borrow_mut() = "another_new_val".to_string();

    assert!(list.content() == vec!["another_new_val".to_string()] && list.size() == 1);

    list = ConsList::<String>::create();
    assert_eq!(list.head(), None);
}

#[test]
pub fn test_tail() {
    let mut list = ConsList::create_from_vec(&vec!["a_1".to_string(), "b_2".to_string(), "c_3".to_string(), "d_4".to_string()]);
    assert_ne!(list.tail(), None);

    let mut read_tail = list.tail();
    assert_eq!(*read_tail.unwrap().borrow(), "d_4".to_string());

    let mut write_tail = list.tail();
    *write_tail.unwrap().borrow_mut() = "new_val".to_string();

    assert!(list.content() == vec!["a_1".to_string(), "b_2".to_string(), "c_3".to_string(), "new_val".to_string()] && list.size() == 4);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert_ne!(list.tail(), None);

    read_tail = list.tail();
    assert_eq!(*read_tail.unwrap().borrow(), "bbq".to_string());

    write_tail = list.tail();
    *write_tail.unwrap().borrow_mut() = "another_new_val".to_string();

    assert!(list.content() == vec!["another_new_val".to_string()] && list.size() == 1);

    list = ConsList::<String>::create();
    assert_eq!(list.tail(), None);
}

#[test]
pub fn test_at_read() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);

    assert_eq!(*list.at(0).unwrap().borrow(), "aB".to_string());
    assert_eq!(*list.at(4).unwrap().borrow(), "z/x".to_string());
    assert_eq!(*list.at(7).unwrap().borrow(), "j1".to_string());
    assert_eq!(list.at(8), Err(InvalidIndex));
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["kl".to_string(), "m_n".to_string(), "O/p".to_string()]);

    assert_eq!(*list.at(0).unwrap().borrow(), "kl".to_string());
    assert_eq!(*list.at(1).unwrap().borrow(), "m_n".to_string());
    assert_eq!(*list.at(2).unwrap().borrow(), "O/p".to_string());
    assert_eq!(list.at(3), Err(InvalidIndex));
    assert!(list.content() == vec!["kl".to_string(), "m_n".to_string(), "O/p".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);

    assert_eq!(*list.at(0).unwrap().borrow(), "b2".to_string());
    assert_eq!(*list.at(1).unwrap().borrow(), "d4".to_string());
    assert_eq!(list.at(2), Err(InvalidIndex));
    assert!(list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);

    assert_eq!(*list.at(0).unwrap().borrow(), "bbq".to_string());
    assert_eq!(list.at(1), Err(InvalidIndex));
    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = ConsList::<String>::create();

    assert_eq!(list.at(0), Err(InvalidIndex));
    assert!(list.empty());
}

#[test]
pub fn test_at_write() {
    let mut list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);

    *list.at(0).unwrap().borrow_mut() = "first_val".to_string();
    *list.at(4).unwrap().borrow_mut() = "second_val".to_string();
    *list.at(7).unwrap().borrow_mut() = "third_val".to_string();

    assert!(list.content() == vec!["first_val".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "second_val".to_string(), "QQ".to_string(), "2_b".to_string(), "third_val".to_string()] && list.size() == 8);

    list = ConsList::create_from_vec(&vec!["kl".to_string(), "m_n".to_string(), "O/p".to_string()]);

    *list.at(0).unwrap().borrow_mut() = "Kl".to_string();
    *list.at(1).unwrap().borrow_mut() = "m/n".to_string();
    *list.at(2).unwrap().borrow_mut() = "o_P".to_string();

    assert!(list.content() == vec!["Kl".to_string(), "m/n".to_string(), "o_P".to_string()] && list.size() == 3);

    list = ConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);

    *list.at(0).unwrap().borrow_mut() = "1st".to_string();
    *list.at(1).unwrap().borrow_mut() = "2nd".to_string();

    assert!(list.content() == vec!["1st".to_string(), "2nd".to_string()] && list.size() == 2);

    list = ConsList::create_from_vec(&vec!["bbq".to_string()]);

    *list.at(0).unwrap().borrow_mut() = "Another_bbq".to_string();

    assert!(list.content() == vec!["Another_bbq".to_string()] && list.size() == 1);

    // additional test, string specific
    list = ConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    list.at(4).unwrap().borrow_mut().push('k');

    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/xk".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);
}

#[test]
pub fn test_clear() {
    let mut list = ConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string()]);
    list.clear();

    assert!(list.empty());

    list = ConsList::<String>::create();
    list.clear();

    assert!(list.empty());
}
