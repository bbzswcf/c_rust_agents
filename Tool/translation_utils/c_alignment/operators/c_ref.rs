use crate::translation_utils::*;

use core::ptr::NonNull;

pub trait CRef {
    type Target;
    fn c_ref(&mut self) -> Self::Target;
}

impl <T> CRef for T {
    type Target = Ptr<T>;
    fn c_ref(&mut self) -> Ptr<T> {
        Ptr::new(self)
    }
}

impl <T> From<Ptr<T>> for SpanView<T> {
    fn from(ptr: Ptr<T>) -> SpanView<T> {
        SpanView { ptr: ptr.0, length: 1 }
    }
}

impl <T> From<Ptr<T>> for Unowned<T> {
    fn from(ptr: Ptr<T>) -> Unowned<T> {
        Unowned(ptr.0)
    }
}

impl <T: Copy> From<Ptr<T>> for Vector<T> {
    fn from(ptr: Ptr<T>) -> Vector<T> {
        let value: T = unsafe { ptr.0.unwrap().as_ref().clone() };
        Vector(Some(vec![value]))
    }
}

macro_rules! c_ref {
    ($ptr:expr) => {
        ($ptr).c_ref().into()
    };
}



pub(crate) use c_ref;