use crate::compare_int::*;
use crate::compare_pointer::*;
use crate::compare_string::*;
use crate::translation_utils::*;
#[test]
fn test_int_compare() {
    let mut a = 4;
    let mut b = 8;
    let mut c = 4;
    assert!(int_compare(c_ref!(a), c_ref!(b)) < 0);
    assert!(int_compare(c_ref!(b), c_ref!(a)) > 0);
    assert_eq!(int_compare(c_ref!(a), c_ref!(c)), 0);
}
#[test]
fn test_int_equal() {
    let mut a = 4;
    let mut b = 8;
    let mut c = 4;
    assert_ne!(int_equal(c_ref!(a), c_ref!(c)), 0);
    assert_eq!(int_equal(c_ref!(a), c_ref!(b)), 0);
}
#[test]
fn test_pointer_compare() {
    let mut array: [i32; 5] = Default::default();
    assert!(pointer_compare(c_ref!(array[0]), c_ref!(array[4])) < 0);
    assert!(pointer_compare(c_ref!(array[3]), c_ref!(array[2])) > 0);
    assert_eq!(pointer_compare(c_ref!(array[4]), c_ref!(array[4])), 0);
}
#[test]
fn test_pointer_equal() {
    let mut a = 0;
    let mut b = 0;
    assert!(pointer_equal(c_ref!(a), c_ref!(a)) != 0);
    assert_eq!(pointer_equal(c_ref!(a), c_ref!(b)), 0);
}
#[test]
fn test_string_compare() {
    let mut test1 = cstr!("Apple");
    let mut test2 = cstr!("Orange");
    let mut test3 = cstr!("Apple");
    assert!(string_compare(test1, test2) < 0);
    assert!(string_compare(test2, test1) > 0);
    assert_eq!(string_compare(test1, test3), 0);
}
#[test]
fn test_string_equal() {
    let mut test1 = cstr!("this is a test string");
    let mut test2 = cstr!("this is a test string ");
    let mut test3 = cstr!("this is a test strin");
    let mut test4 = cstr!("this is a test strinG");
    let mut test5 = cstr!("this is a test string");
    assert!(string_equal(test1, test5) != 0);
    assert_eq!(string_equal(test1, test2), 0);
    assert_eq!(string_equal(test1, test3), 0);
    assert_eq!(string_equal(test1, test4), 0);
}
#[test]
fn test_string_nocase_compare() {
    let mut test1 = cstr!("Apple");
    let mut test2 = cstr!("Orange");
    let mut test3 = cstr!("Apple");
    let mut test4 = cstr!("Alpha");
    let mut test5 = cstr!("bravo");
    let mut test6 = cstr!("Charlie");
    assert!(string_nocase_compare(test1, test2) < 0);
    assert!(string_nocase_compare(test2, test1) > 0);
    assert_eq!(string_nocase_compare(test1, test3), 0);
    assert!(string_nocase_compare(test4, test5) < 0);
    assert!(string_nocase_compare(test5, test6) < 0);
}
#[test]
fn test_string_nocase_equal() {
    let mut test1 = cstr!("this is a test string");
    let mut test2 = cstr!("this is a test string ");
    let mut test3 = cstr!("this is a test strin");
    let mut test4 = cstr!("this is a test strinG");
    let mut test5 = cstr!("this is a test string");
    assert!(string_nocase_equal(test1, test5) != 0);
    assert_eq!(string_nocase_equal(test1, test2), 0);
    assert_eq!(string_nocase_equal(test1, test3), 0);
    assert!(string_nocase_equal(test1, test4) != 0);
}
