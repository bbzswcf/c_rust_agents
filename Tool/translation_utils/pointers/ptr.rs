use crate::translation_utils::*;

use core::ops::*;

use core::ptr::NonNull;

pub struct Ptr<T>(pub Option<NonNull<T>>);

impl <T: std::fmt::Display> std::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "Ptr({})", self.addr())
        if self.0.is_none() {
            write!(f, "Ptr(None)")
        } else {
            write!(f, "Ptr(Some(&{}))", unsafe { self.0.unwrap().as_ref() })
        }
    }
}

impl <T> Default for Ptr<T> {
    fn default() -> Self {
        Ptr(None)
    }
}

impl <T> Ptr<T> {
    pub fn new(value: &T) -> Self {
        Ptr(Some(NonNull::new(value as *const T as *mut T).unwrap()))
    }

    pub fn drop(&mut self) {
        *self = Ptr(None);
    }

    pub fn addr(&self) -> usize {
        self.0.unwrap().as_ptr() as *const T as *const () as usize
    }

    pub unsafe fn malloc_leaked(value: T) -> Ptr<T> {
        let mut result: Ptr<T>;
        let leaked_value: NonNull<T> = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(value))) };
        result = Ptr(Some(leaked_value));
        return result;
    }
    
    pub unsafe fn free_leaked(&mut self) {
        unsafe { Box::from_raw(self.0.unwrap().as_ptr());  }
    }    
}

impl <T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr(self.0)
    }
}

impl <T> PointerTrait for Ptr<T> { }

impl <T> From<Null> for Ptr<T> {
    fn from(_: Null) -> Self {
        Ptr(None)
    }
}

impl <T> Copy for Ptr<T> {}

impl <T> PartialEq for Ptr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl <T> PartialOrd for Ptr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl <T> Deref for Ptr<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            self.0.unwrap().as_ref()
        }
    }
}

impl <T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            self.0.unwrap().as_mut()
        }
    }
}

