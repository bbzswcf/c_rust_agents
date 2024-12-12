use crate::translation_utils::*;

pub fn memmove<T: Clone>(mut dst: SpanView<T>, mut src: SpanView<T>, count: usize) {
    // unsafe {
    //     std::ptr::copy(src.ptr.unwrap().as_ptr(), dst.ptr.unwrap().as_ptr(), count / std::mem::size_of::<T>());
    // }
    let new_count = count / std::mem::size_of::<T>();
    if dst == src {
        return;
    }
    if dst < src {
        let mut i = 0;
        while i < new_count {
            dst[i] = src[i].clone();
            i += 1;
        }
    } else {
        let mut i = new_count;
        while i > 0 {
            i -= 1;
            dst[i] = src[i].clone();
        }
    }
}

macro_rules! c_memmove {
    ($dst:expr, $src:expr, $count:expr) => {
        if $count as usize != 0 {
            memmove($dst, $src, $count as usize);
        }        
    };
}
pub(crate) use c_memmove;