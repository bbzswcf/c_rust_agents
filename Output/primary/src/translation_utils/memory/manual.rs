use crate::translation_utils::*;

use core::ops::*;
use core::ptr::NonNull;

pub static CREATE_COUNT: GlobalVar<i32> = GlobalVar::new(0);

macro_rules! test_no_memory_leak {
    () => {
        assert_eq!(*CREATE_COUNT.get(), 0);
    };
}

pub(crate) use test_no_memory_leak;

pub struct Manual<T>(pub Option<NonNull<T>>);

impl <T> Manual<T> {
    pub fn new(value: T) -> Self {
        *CREATE_COUNT.get() += 1;
        unsafe {
            Manual(Some(NonNull::new_unchecked(Box::into_raw(Box::new(value)))))
        }
    }
    pub fn drop(&mut self) {
        if self.0.is_none() {
            return;
        }
        *CREATE_COUNT.get() -= 1;
        unsafe { Box::from_raw(self.0.unwrap().as_ptr()) };
        self.0 = None;
    }
}

impl <T> PointerTrait for Manual<T> {}

impl <T> From<Null> for Manual<T> {
    fn from(_: Null) -> Self {
        Manual(None)
    }
}

impl <T> Deref for Manual<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref().unwrap().as_ref() }
    }
}

impl <T> DerefMut for Manual<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut().unwrap().as_mut() }
    }
}

impl <T> Default for Manual<T> {
    fn default() -> Self {
        Manual(None)
    }
}

impl <T> PartialEq for Manual<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() && other.0.is_none() {
            true
        } else if self.0.is_none() || other.0.is_none() {
            false
        } else {
            self.0 == other.0
        }
    }
}

impl <T> Clone for Manual<T> {
    fn clone(&self) -> Self {
        Manual(self.0)
    }
}

impl <T> Copy for Manual<T> { }