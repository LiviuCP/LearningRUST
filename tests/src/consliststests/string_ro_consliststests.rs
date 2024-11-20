#[cfg(test)]

use learn_rust_lib::cons::ro_conslists::{RoConsList, InvalidIndex};

#[test]
pub fn test_create() {
    let list = RoConsList::<String>::create();
    assert!(list.content() == Vec::<String>::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_create_from_vec() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8 && !list.empty());

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    assert!(list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2 && !list.empty());

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1 && !list.empty());

    list = RoConsList::create_from_vec(&Vec::<String>::new());
    assert!(list.content() == Vec::<String>::new() && list.size() == 0 && list.empty());
}

#[test]
pub fn test_iter() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), "aB".to_string());

    list_iter.next();
    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap(), "z/x".to_string());

    list_iter.next();
    list_iter.next();

    assert_eq!(*list_iter.next().unwrap(), "j1".to_string());
    assert_eq!(list_iter.next(), None);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), "b2".to_string());
    assert_eq!(*list_iter.next().unwrap(), "d4".to_string());
    assert!(list_iter.next() == None);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    list_iter = list.iter();

    assert_eq!(*list_iter.next().unwrap(), "bbq".to_string());
    assert!(list_iter.next() == None);

    list = RoConsList::<String>::create();
    list_iter = list.iter();

    assert!(list_iter.next() == None);
}

#[test]
pub fn test_iter_with_lambda() {
    let list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let arr: Vec<String> = list.iter().map(|item| {let mut temp = (*item).clone(); temp.push_str("_suff"); temp}).collect();

    assert_eq!(arr, vec!["aB_suff".to_string(), "cD_suff".to_string(), "i_j_suff".to_string(), "gh_suff".to_string(), "z/x_suff".to_string(), "QQ_suff".to_string(), "2_b_suff".to_string(), "j1_suff".to_string()]);
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);
}

#[test]
pub fn test_push_front() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()]);
    let mut value = "new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["new_val".to_string(), "aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "second_new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["second_new_val".to_string(), "bbq".to_string()] && list.size() == 2);

    list = RoConsList::create();
    value = "third_new_val".to_string();
    list.push_front(&value);

    assert!(list.content() == vec!["third_new_val".to_string()] && list.size() == 1);
}

#[test]
pub fn test_pop_front() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.pop_front();

    assert!(*result.unwrap() == "aB".to_string() && list.content() == vec!["cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.pop_front();

    assert!(*result.unwrap() == "b2".to_string() && list.content() == vec!["d4".to_string()] && list.size() == 1);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.pop_front();

    assert!(*result.unwrap() == "bbq".to_string() && list.empty());

    list = RoConsList::<String>::create();
    result = list.pop_front();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_push_back() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()]);
    let mut value = "new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "new_val".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "second_new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["bbq".to_string(), "second_new_val".to_string()] && list.size() == 2);

    list = RoConsList::create();
    value = "third_new_val".to_string();
    list.push_back(&value);

    assert!(list.content() == vec!["third_new_val".to_string()] && list.size() == 1);
}

#[test]
pub fn test_pop_back() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.pop_back();

    assert!(*result.unwrap() == "j1".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 7);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.pop_back();

    assert!(*result.unwrap() == "d4".to_string() && list.content() == vec!["b2".to_string()] && list.size() == 1);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.pop_back();

    assert!(*result.unwrap() == "bbq".to_string() && list.empty());

    list = RoConsList::<String>::create();
    result = list.pop_back();

    assert!(result == None && list.empty());
}

#[test]
pub fn test_insert() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut value = "first_val".to_string();
    let mut result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["first_val".to_string(), "aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 9);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["aB".to_string(), "cD".to_string(), "first_val".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 9);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 7);

    assert!(result == Ok(7) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "first_val".to_string(), "j1".to_string()] && list.size() == 9);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 8);

    assert!(result == Ok(8) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string(), "first_val".to_string()] && list.size() == 9);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.insert(&value, 9);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    value = "second_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["second_val".to_string(), "af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["af".to_string(), "second_val".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["af".to_string(), "d_8".to_string(), "second_val".to_string(), "Z-4".to_string()] && list.size() == 4);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 3);

    assert!(result == Ok(3) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string(), "second_val".to_string()] && list.size() == 4);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.insert(&value, 4);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    value = "third_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["third_val".to_string(), "b2".to_string(), "d4".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["b2".to_string(), "third_val".to_string(), "d4".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Ok(2) && list.content() == vec!["b2".to_string(), "d4".to_string(), "third_val".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.insert(&value, 3);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    value = "fourth_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["fourth_val".to_string(), "bbq".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.insert(&value, 1);

    assert!(result == Ok(1) && list.content() == vec!["bbq".to_string(), "fourth_val".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.insert(&value, 2);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = RoConsList::create();
    value = "fifth_val".to_string();
    result = list.insert(&value, 0);

    assert!(result == Ok(0) && list.content() == vec!["fifth_val".to_string()] && list.size() == 1);

    list = RoConsList::create();
    result = list.insert(&value, 1);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_remove() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    let mut result = list.remove(0);

    assert!(*result.unwrap() == "aB".to_string() && list.content() == vec!["cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(4);

    assert!(*result.unwrap() == "z/x".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 7);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(7);

    assert!(*result.unwrap() == "j1".to_string() && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string()] && list.size() == 7);

    list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    result = list.remove(8);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap() == "af".to_string() && list.content() == vec!["d_8".to_string(), "Z-4".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(1);

    assert!(*result.unwrap() == "d_8".to_string() && list.content() == vec!["af".to_string(), "Z-4".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(2);

    assert!(*result.unwrap() == "Z-4".to_string() && list.content() == vec!["af".to_string(), "d_8".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()]);
    result = list.remove(3);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["af".to_string(), "d_8".to_string(), "Z-4".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap() == "b2".to_string() && list.content() == vec!["d4".to_string()] && list.size() == 1);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(1);

    assert!(*result.unwrap() == "d4".to_string() && list.content() == vec!["b2".to_string()] && list.size() == 1);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    result = list.remove(2);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.remove(0);

    assert!(*result.unwrap() == "bbq".to_string() && list.empty());

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    result = list.remove(1);

    assert!(result == Err(InvalidIndex) && list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = RoConsList::<String>::create();
    result = list.remove(0);

    assert!(result == Err(InvalidIndex) && list.empty());
}

#[test]
pub fn test_reverse() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["j1".to_string(), "2_b".to_string(), "QQ".to_string(), "z/x".to_string(), "gh".to_string(), "i_j".to_string(), "cD".to_string(), "aB".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["d4".to_string(), "b2".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    list.reverse();

    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = RoConsList::<String>::create();
    list.reverse();

    assert!(list.empty());
}

#[test]
pub fn test_merge() {
    let mut first_list = RoConsList::<String>::create();
    let mut second_list = RoConsList::<String>::create();
    first_list.merge(&mut second_list);

    assert!(first_list.empty());
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = RoConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = RoConsList::create();
    second_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = RoConsList::create();
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = RoConsList::create();
    second_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["bbq".to_string(), "b2".to_string(), "d4".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()]);
    first_list.merge(&mut second_list);

    assert!(first_list.content() == vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string(), "a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());
}

#[test]
pub fn test_split() {
    let mut first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    let mut second_list = RoConsList::create();
    let mut result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 9);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["a1".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 8);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 5);

    assert_eq!(result, Ok(5));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string()] && first_list.size() == 5);
    assert!(second_list.content() == vec!["f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 4);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 7);

    assert_eq!(result, Ok(7));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string()] && first_list.size() == 7);
    assert!(second_list.content() == vec!["h8".to_string(), "i9".to_string()] && second_list.size() == 2);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 8);

    assert_eq!(result, Ok(8));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string()] && first_list.size() == 8);
    assert!(second_list.content() == vec!["i9".to_string()] && second_list.size() == 1);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && second_list.size() == 3);

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["b2".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["d4".to_string(), "bbq".to_string()] && second_list.size() == 2);

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Ok(2));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.content() == vec!["bbq".to_string()] && second_list.size() == 1);

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string(), "bbq".to_string()] && first_list.size() == 3);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["b2".to_string(), "d4".to_string()] && second_list.size() == 2);

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Ok(1));
    assert!(first_list.content() == vec!["b2".to_string()] && first_list.size() == 1);
    assert!(second_list.content() == vec!["d4".to_string()] && second_list.size() == 1);

    first_list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 2);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["b2".to_string(), "d4".to_string()] && first_list.size() == 2);
    assert!(second_list.empty());

    first_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Ok(0));
    assert!(first_list.empty());
    assert!(second_list.content() == vec!["bbq".to_string()] && second_list.size() == 1);

    first_list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    second_list = RoConsList::create();
    result = first_list.split(&mut second_list, 1);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["bbq".to_string()] && first_list.size() == 1);
    assert!(second_list.empty());

    first_list = RoConsList::<String>::create();
    second_list = RoConsList::<String>::create();
    result = first_list.split(&mut second_list, 0);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.empty());
    assert!(second_list.empty());

    // additional tests, second list initially not empty
    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()]);
    result = first_list.split(&mut second_list, 3);

    assert_eq!(result, Ok(3));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string()] && first_list.size() == 3);
    assert!(second_list.content() == vec!["d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && second_list.size() == 6);

    first_list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()]);
    second_list = RoConsList::create_from_vec(&vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()]);
    result = first_list.split(&mut second_list, 9);

    assert_eq!(result, Err(InvalidIndex));
    assert!(first_list.content() == vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string(), "e5".to_string(), "f6".to_string(), "g7".to_string(), "h8".to_string(), "i9".to_string()] && first_list.size() == 9);
    assert!(second_list.content() == vec!["z2".to_string(), "t4".to_string(), "q9".to_string(), "r5".to_string()] && second_list.size() == 4);
}

#[test]
pub fn test_head() {
    let mut list = RoConsList::create_from_vec(&vec!["a_1".to_string(), "b_2".to_string(), "c_3".to_string(), "d_4".to_string()]);
    assert_ne!(list.head(), None);

    let mut read_head = list.head();
    assert_eq!(*read_head.unwrap(), "a_1".to_string());

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert_ne!(list.head(), None);

    read_head = list.head();
    assert_eq!(*read_head.unwrap(), "bbq".to_string());

    list = RoConsList::<String>::create();
    assert_eq!(list.head(), None);
}

#[test]
pub fn test_tail() {
    let mut list = RoConsList::create_from_vec(&vec!["a_1".to_string(), "b_2".to_string(), "c_3".to_string(), "d_4".to_string()]);
    assert_ne!(list.tail(), None);

    let mut read_tail = list.tail();
    assert_eq!(*read_tail.unwrap(), "d_4".to_string());

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);
    assert_ne!(list.tail(), None);

    read_tail = list.tail();
    assert_eq!(*read_tail.unwrap(), "bbq".to_string());

    list = RoConsList::<String>::create();
    assert_eq!(list.tail(), None);
}

#[test]
pub fn test_at() {
    let mut list = RoConsList::create_from_vec(&vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()]);

    assert_eq!(*list.at(0).unwrap(), "aB".to_string());
    assert_eq!(*list.at(4).unwrap(), "z/x".to_string());
    assert_eq!(*list.at(7).unwrap(), "j1".to_string());
    assert_eq!(list.at(8), Err(InvalidIndex));
    assert!(list.content() == vec!["aB".to_string(), "cD".to_string(), "i_j".to_string(), "gh".to_string(), "z/x".to_string(), "QQ".to_string(), "2_b".to_string(), "j1".to_string()] && list.size() == 8);

    list = RoConsList::create_from_vec(&vec!["kl".to_string(), "m_n".to_string(), "O/p".to_string()]);

    assert_eq!(*list.at(0).unwrap(), "kl".to_string());
    assert_eq!(*list.at(1).unwrap(), "m_n".to_string());
    assert_eq!(*list.at(2).unwrap(), "O/p".to_string());
    assert_eq!(list.at(3), Err(InvalidIndex));
    assert!(list.content() == vec!["kl".to_string(), "m_n".to_string(), "O/p".to_string()] && list.size() == 3);

    list = RoConsList::create_from_vec(&vec!["b2".to_string(), "d4".to_string()]);

    assert_eq!(*list.at(0).unwrap(), "b2".to_string());
    assert_eq!(*list.at(1).unwrap(), "d4".to_string());
    assert_eq!(list.at(2), Err(InvalidIndex));
    assert!(list.content() == vec!["b2".to_string(), "d4".to_string()] && list.size() == 2);

    list = RoConsList::create_from_vec(&vec!["bbq".to_string()]);

    assert_eq!(*list.at(0).unwrap(), "bbq".to_string());
    assert_eq!(list.at(1), Err(InvalidIndex));
    assert!(list.content() == vec!["bbq".to_string()] && list.size() == 1);

    list = RoConsList::<String>::create();

    assert_eq!(list.at(0), Err(InvalidIndex));
    assert!(list.empty());
}

#[test]
pub fn test_clear() {
    let mut list = RoConsList::create_from_vec(&vec!["a1".to_string(), "b2".to_string(), "c3".to_string(), "d4".to_string()]);
    list.clear();

    assert!(list.empty());

    list = RoConsList::<String>::create();
    list.clear();

    assert!(list.empty());
}