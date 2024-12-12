use crate::translation_utils::*;

pub trait CMalloc {
    fn c_malloc(size: usize) -> Self;
}

impl <T: Default> CMalloc for Owned<T> {
    fn c_malloc(size: usize) -> Self {
        if size == 0 {
            null!()
        } else if size == c_sizeof!(T) {
            Owned::new(T::default())
        } else {
            panic!("OwnedPtr Malloc: Size Not Match")
        }
    }
}

impl <T: Default> CMalloc for Vector<T> {
    fn c_malloc(size: usize) -> Self {
        Vector::new(size / std::mem::size_of::<T>())
    }
}

impl <T: Default> CMalloc for Manual<T> {
    fn c_malloc(size: usize) -> Self {
        if size == 0 {
            null!()
        } else if size == c_sizeof!(T) {
            Manual::new(T::default())
        } else {
            panic!("ManualPtr Malloc: Size Not Match")
        }
    }
}

impl CMalloc for CStr {
    fn c_malloc(size: usize) -> Self {
        if size == 0 {
            null!()
        } else {
            CStr::new_alloced(size)
        }
    }
}

pub fn malloc<T: CMalloc>(size: usize) -> T {
    T::c_malloc(size)
}

impl <T: Default> CMalloc for Ptr<T> {
    fn c_malloc(size: usize) -> Self {        
        if size == 0 {
            null!()
        } else if size == c_sizeof!(T) {
            unsafe {
                Ptr::malloc_leaked(T::default())
            }
        } else {
            panic!("Ptr Malloc: Size Not Match")
        }
    }
}

macro_rules! c_malloc {
    ($size:expr) => {
        malloc($size as usize)
    };
}

pub(crate) use c_malloc;