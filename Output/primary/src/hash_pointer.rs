use crate::translation_utils::*;
pub fn pointer_hash<T>(location: Ptr<T>) -> u32 {
    return location.addr() as u32;
}
