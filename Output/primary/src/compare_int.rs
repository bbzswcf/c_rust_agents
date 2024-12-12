use crate::translation_utils::*;
pub fn int_equal(vlocation1: Ptr<i32>, vlocation2: Ptr<i32>) -> i32 {
    let mut location1: Ptr<i32>;
    let mut location2: Ptr<i32>;
    location1 = vlocation1;
    location2 = vlocation2;
    return if *location1 == *location2 { 1 } else { 0 };
}
pub fn int_compare(vlocation1: Ptr<i32>, vlocation2: Ptr<i32>) -> i32 {
    let location1: Ptr<i32> = vlocation1;
    let location2: Ptr<i32> = vlocation2;
    if *location1 < *location2 {
        return -1;
    } else if *location1 > *location2 {
        return 1;
    } else {
        return 0;
    }
}
