use crate::translation_utils::*;

pub fn calloc<T: CMalloc>(count: usize, size: usize) -> T {
    T::c_malloc(count * size)
}
macro_rules! c_calloc {
    ($count:expr, $size:expr) => {
        calloc($count as usize, $size as usize)
    };
}

pub(crate) use c_calloc;