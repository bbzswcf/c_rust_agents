
pub type ArrayListValue = *mut std::ffi::c_void;

pub struct ArrayList {
    pub data: *mut ArrayListValue,
    pub length: u32,
    pub _alloced: u32,
}

pub type ArrayListEqualFunc = extern "C" fn(ArrayListValue, ArrayListValue) -> i32;
pub type ArrayListCompareFunc = extern "C" fn(ArrayListValue, ArrayListValue) -> i32;
fn main() {}