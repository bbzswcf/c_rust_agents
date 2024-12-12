use crate::translation_utils::*;
pub fn int_hash(vlocation: Ptr<i32>) -> u32 {
    let mut location: Ptr<i32>;
    location = vlocation;
    return *location as u32;
}
