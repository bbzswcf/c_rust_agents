use crate::translation_utils::*;

use core::ops::*;
use core::ptr::NonNull;

pub struct Owned<T>(pub Option<Box<T>>);

impl <T> Owned<T> {
    pub fn new(value: T) -> Self {
        Owned(Some(Box::new(value)))
    }

    pub fn drop(&mut self) {
        *self = Owned(None);
    }

    pub fn unowned(&mut self) -> Unowned<T> {
        if self.0.is_none() {
            Unowned(None)
        } else {
            Unowned(Some(NonNull::new(self.0.as_mut().unwrap().as_mut() as *mut T).unwrap()))
        }
    }
}

impl <T> PointerTrait for Owned<T> {}

impl <T> From<Null> for Owned<T> {
    fn from(_: Null) -> Self {
        Owned(None)
    }
}

impl <T> Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0.as_ref().unwrap()
    }
}

impl <T> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0.as_mut().unwrap()
    }
}

impl <T> Default for Owned<T> {
    fn default() -> Self {
        Owned(None)
    }
}

impl <T> PartialEq for Owned<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() && other.0.is_none() {
            true
        } else {
            false
        }
    }
}