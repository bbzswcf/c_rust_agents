use crate::translation_utils::*;
pub fn string_hash(mut vlocation: CStr) -> u32 {
    let mut result: u32 = 5381;
    let mut p: CStr;
    p = vlocation.unowned();
    while *p != 0 {
        result = (result << 5) + result + (*p) as u32;
        p += 1;
    }
    result
}
pub fn string_nocase_hash(vlocation: CStr) -> u32 {
    let mut result: u32 = 5381;
    let mut p: CStr;
    p = vlocation;
    while *p != '\0' as u8 {        
        result = (result << 5) + result + c_tolower!(*p) as u32;
        p += 1;
    }
    return result 
}
