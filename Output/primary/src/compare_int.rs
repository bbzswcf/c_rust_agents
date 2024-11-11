
pub const MAX_BUFFER: usize = 1024;
// Modified: Replaced the approximate value of PI with the constant directly from the standard library
pub const PI: f64 = std::f64::consts::PI;

pub struct Data {
    pub value: i32,
    pub description: String,
}

pub struct Config {
    pub flag: i32,
    pub threshold: f64,
}

pub enum Status {
    SUCCESS,
    ERROR,
    PENDING,
}

pub type byte = u8;

pub static mut GLOBAL_VARIABLE: i32 = 0;
pub fn int_equal(vlocation1: &i32, vlocation2: &i32) -> bool {
    let location1 = vlocation1;
    let location2 = vlocation2;

    return *location1 == *location2;
}
pub unsafe fn int_compare(vlocation1: *mut i32, vlocation2: *mut i32) -> i32 {
    let location1 = vlocation1 as *const i32;
    let location2 = vlocation2 as *const i32;

    if *location1 < *location2 {
        return -1;
    } else if *location1 > *location2 {
        return 1;
    } else {
        return 0;
    }
}