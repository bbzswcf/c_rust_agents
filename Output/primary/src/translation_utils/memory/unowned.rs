use crate::translation_utils::*;

use core::ops::*;
use core::ptr::NonNull;

pub struct Unowned<T>(pub Option<NonNull<T>>);

impl <T> Unowned<T> {
    pub fn drop(&mut self) {        
    }
}

impl <T> PointerTrait for Unowned<T> {}

impl <T> From<Null> for Unowned<T> {
    fn from(_: Null) -> Self {
        Unowned(None)
    }
}

impl <T> Deref for Unowned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref().unwrap().as_ref() }
    }
}

impl <T> DerefMut for Unowned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut().unwrap().as_mut() }
    }
}

impl <T> Default for Unowned<T> {
    fn default() -> Self {
        Unowned(None)
    }
}

impl <T> PartialEq for Unowned<T> {
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

impl <T> Clone for Unowned<T> {
    fn clone(&self) -> Self {
        Unowned(self.0)
    }
}

impl <T> Copy for Unowned<T> { }