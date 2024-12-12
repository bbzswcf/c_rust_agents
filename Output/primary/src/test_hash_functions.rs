use crate::hash_pointer::*;
use crate::hash_int::*;
use crate::hash_string::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 200 }; }
#[test]
fn test_pointer_hash() {
    let mut array: Array<i32, {NUM_TEST_VALUES!()}> = Default::default();
    let mut i: i32;
    let mut j: i32;

    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        array[i] = 0;
    });

    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        c_for!(j = i + 1; j < NUM_TEST_VALUES!(); j += 1; {
            assert!(pointer_hash(c_ref!(array[i])) != pointer_hash(c_ref!(array[j])));
        });
    });
}
#[test]
fn test_int_hash() {
    let mut array: Array<i32, {NUM_TEST_VALUES!()}> = Default::default();
    let mut i: i32;
    let mut j: i32;
    for i in 0..NUM_TEST_VALUES!() {
        array[i] = i as i32;
    }
    for i in 0..NUM_TEST_VALUES!() {
        for j in (i + 1)..NUM_TEST_VALUES!() {
            assert!(int_hash(c_ref!(array[i])) != int_hash(c_ref!(array[j])));
        }
    }
    i = 5000;
    j = 5000;
    assert!(int_hash(c_ref!(i)) == int_hash(c_ref!(j)));
}
#[test]
fn test_string_hash() {
    let mut test1 = cstr!("this is a test");
    let mut test2 = cstr!("this is a tesu");
    let mut test3 = cstr!("this is a test ");
    let mut test4 = cstr!("this is a test");
    let mut test5 = cstr!("This is a test");
    assert!(string_hash(test1) != string_hash(test2));
    assert!(string_hash(test1) != string_hash(test3));
    assert!(string_hash(test1) != string_hash(test5));
    assert_eq!(string_hash(test1), string_hash(test4));
}
#[test]
fn test_string_nocase_hash() {
    let mut test1 = cstr!("this is a test");
    let mut test2 = cstr!("this is a tesu");
    let mut test3 = cstr!("this is a test ");
    let mut test4 = cstr!("this is a test");
    let mut test5 = cstr!("This is a test");
    assert!(string_nocase_hash(test1) != string_nocase_hash(test2));
    assert!(string_nocase_hash(test1) != string_nocase_hash(test3));
    assert_eq!(string_nocase_hash(test1), string_nocase_hash(test5));
    assert_eq!(string_nocase_hash(test1), string_nocase_hash(test4));
}
