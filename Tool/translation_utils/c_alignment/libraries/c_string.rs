use crate::translation_utils::*;

use core::cmp::Ordering;

pub fn strcmp(vlocation1: CStr, vlocation2: CStr) -> i32 {
    let result = (vlocation1.to_rust_slice()).cmp(vlocation2.to_rust_slice());
    match result {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }
}

pub fn strlen(vlocation: CStr) -> usize {
    vlocation.length
}

pub fn strdup(vlocation: CStr) -> CStr {
    let length = strlen(vlocation);
    let mut new_vlocation = CStr::new_alloced(length);
    for i in 0..length {
        new_vlocation[i] = vlocation[i];
    }
    new_vlocation
}

macro_rules! c_strcmp {
    ($s1: expr, $s2: expr) => {
        strcmp($s1, $s2)
    };
}

pub(crate) use c_strcmp;

macro_rules! c_strlen {
    ($s: expr) => {
        strlen($s)
    };
}

pub(crate) use c_strlen;

macro_rules! c_strdup {
    ($s: expr) => {
        strdup($s)
    };
}

pub(crate) use c_strdup;

macro_rules! c_sprintf {
    ($buf: expr, $($arg: tt)*) => {
        let tmp = format!($($arg)*);
        let tmp1 = format!("{}\0", tmp);
        $buf = CStr::from(tmp1.as_bytes());
    };
}

pub(crate) use c_sprintf;