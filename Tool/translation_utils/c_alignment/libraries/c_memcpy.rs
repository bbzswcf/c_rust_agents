use crate::translation_utils::*;

pub fn memcpy<T: Clone>(mut dst: SpanView<T>, mut src: SpanView<T>, count: usize) {
    // unsafe {
    //     std::ptr::copy_nonoverlapping(src.ptr.unwrap().as_ptr(), dst.ptr.unwrap().as_ptr(), count / std::mem::size_of::<T>());
    // }
    let new_count = count / std::mem::size_of::<T>();
    let mut i = 0;
    while i < new_count {
        dst[i] = src[i].clone();
        i += 1;
    }
}

macro_rules! c_memcpy {
    ($dst:expr, $src:expr, $count:expr) => {
        if $count as usize != 0 {
            memcpy($dst, $src, $count as usize);
        }        
    };
}
pub(crate) use c_memcpy;