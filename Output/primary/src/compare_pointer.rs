use crate::translation_utils::*;
pub fn pointer_equal<T>(location1: Ptr<T>, location2: Ptr<T>) -> i32 {
    return if location1 == location2 { 1 } else { 0 };
}
pub fn pointer_compare<T>(location1: Ptr<T>, location2: Ptr<T>) -> i32 {
    if location1 < location2 {
        -1
    } else if location1 > location2 {
        1
    } else {
        0
    }
}
