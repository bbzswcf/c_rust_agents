
pub const MAX_BUFFER: usize = 1024;
// Modified: Replaced approximate value with exact constant from the standard library
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
pub fn string_hash(string: *const u8) -> u32 {
    let mut result: u32 = 5381;
    let mut p = string;

    unsafe {
        while *p != b'\0' {
            // Dereference of raw pointer requires unsafe block
            result = (result << 5).wrapping_add(result).wrapping_add(*p as u32);
            // Call to unsafe function requires unsafe block
            p = p.offset(1);
        }
    }

    result
}