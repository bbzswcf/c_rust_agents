use crate::translation_utils::*;

pub fn realloc<T: Default>(ptr: Vector<T>, size: usize) -> Vector<T> {
    if size.as_usize() == 0 {
        return null!();
    } else {
        if ptr == null!() {
            return c_malloc!(size);
        } else {
            let mut new_ptr = ptr;
            new_ptr.resize(size / std::mem::size_of::<T>());
            return new_ptr;
        }
    }
}

macro_rules! c_realloc {
    ($ptr:expr, $size:expr) => {
        realloc($ptr.take(), $size as usize)
    };
}

pub(crate) use c_realloc;