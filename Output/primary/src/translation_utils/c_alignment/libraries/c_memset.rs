use crate::translation_utils::*;

pub fn memset<T: CMemset>(mut dst: T, value: u8, count: usize) {
    dst.c_memset(value, count);
}

pub trait CMemset {
    fn c_memset(&mut self, value: u8, count: usize);
}

// impl <T: Clone> CMemset for SpanView<T> {
//     fn c_memset(&mut self, value: u8, count: usize) {
//         let new_count = count / std::mem::size_of::<T>();
//         if new_count > self.length {
//             panic!("CMemset: Count is greater than length");
//         }
//         let mut i = 0;
//         while i < new_count {
//             self[i] = value;
//             i += 1;
//         }
//     }
// }

// impl <T: Clone> CMemset for Vector<T> {
//     fn c_memset(&mut self, value: u8, count: usize) {
//         let new_count = count / std::mem::size_of::<T>();
//         if new_count > self.length {
//             panic!("CMemset: Count is greater than length");
//         }
//         let mut i = 0;
//         while i < new_count {
//             self[i] = value;
//             i += 1;
//         }
//     }
// }

impl CMemset for CStr {
    fn c_memset(&mut self, value: u8, count: usize) {
        if !self._allocated {
            panic!("Cannot memset an unallocated CStr");
        }
        if count > self.length {
            panic!("CMemset: Count is greater than length");
        }
        let mut i = 0;
        while i < count {
            self[i] = value;
            i += 1;
        }
    }
}

macro_rules! c_memset {
    ($dst:expr, $value:expr, $count:expr) => {
        if $count as usize != 0 {
            memset($dst, $value, $count as usize);
        }        
    };
}

pub(crate) use c_memset;