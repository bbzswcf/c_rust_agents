use crate::translation_utils::*;

pub fn atoi(vlocation1: CStr) -> i64 {
    let mut result = 0;
    let mut sign = 1;
    let mut i = 0;
    let length = vlocation1.length;
    if vlocation1[0] == b'-' {
        sign = -1;
        i = 1;
    }
    while i < length {
        let curr_char: u8 = vlocation1[i];
        if curr_char == b'\0' {
            break;
        }
        if curr_char < b'0' || curr_char > b'9' {
            return 0;
        }
        result = result * 10 + (curr_char - b'0') as i64;
        i += 1;
    }
    sign * result
}

macro_rules! c_atoi {
    ($s: expr) => {
        atoi($s)
    };
}

pub(crate) use c_atoi;