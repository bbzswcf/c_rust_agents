use crate::translation_utils::*;
use std::cmp::Ordering;

pub fn string_equal(mut vlocation1: CStr, mut vlocation2: CStr) -> i32 {
    return if c_strcmp!(vlocation1, vlocation2) == 0 { 1 } else { 0 };
}
use std::cmp::Ordering;

pub fn string_compare(mut vlocation1: CStr, mut vlocation2: CStr) -> i32 {
    let result = vlocation1.cmp(&vlocation2);
    match result {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}
pub fn string_nocase_equal(mut vlocation1: CStr, mut vlocation2: CStr) -> i32 {
    return if string_nocase_compare(vlocation1, vlocation2) == 0 { 1 } else { 0 };
}
use std::cmp::Ordering;

pub fn string_nocase_compare(mut vlocation1: CStr, mut vlocation2: CStr) -> i32 {
    let mut p1: CStr;
    let mut p2: CStr;
    let mut c1: i32;
    let mut c2: i32;
    p1 = vlocation1.unowned();
    p2 = vlocation2.unowned();
    loop {
        c1 = c_tolower!(*p1);
        c2 = c_tolower!(*p2);
        if c1 != c2 {
            if c1 < c2 {
                return -1;
            } else {
                return 1;
            }
        }
        if c1 == '\0' as i32 {
            break;
        }
        p1 += 1;
        p2 += 1;
    }
    return 0;
}
