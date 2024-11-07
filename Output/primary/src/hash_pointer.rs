
pub const MAX_BUFFER: usize = 1024;
// Modified: Use the constant directly from the standard library to ensure accuracy
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
pub fn pointer_hash(location: *const ()) -> u32 {
    location as usize as u32
}