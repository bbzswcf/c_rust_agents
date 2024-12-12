use crate::translation_utils::*;

use core::ops::*;

use core::ptr::NonNull;

pub struct Vector<T>(pub Option<Vec<T>>);

impl <T> Vector<T> {
    pub fn new<I: Integer>(length: I) -> Self where T: Default {
        Vector(Some(vec![(); length.as_usize()].iter().map(|_| Default::default()).collect()))
    }

    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    pub fn drop(&mut self) {
        *self = Vector(None);
    }
    pub fn resize<I: Integer>(&mut self, length: I) where T: Default {
        self.0.as_mut().unwrap().resize_with(length.as_usize(), Default::default);
    }

    pub fn take(&mut self) -> Vector<T> {
        Vector(self.0.take())
    }

    pub fn len(&self) -> usize {
        self.0.as_ref().unwrap().len()
    }

    pub fn clone(&self) -> Vector<T> where T: Clone {
        Vector(self.0.clone())
    }

    pub fn from(data: &[T]) -> Vector<T> where T: Clone {
        Vector(Some(Vec::from(data)))
    }
    
    pub fn as_rust_slice(&self) -> &[T] {
        self.0.as_ref().unwrap().as_slice()
    }

    pub fn as_rust_slice_mut(&mut self) -> &mut [T] {
        self.0.as_mut().unwrap().as_mut_slice()
    }

    pub fn unowned(&mut self) -> SpanView<T> {
        if self.0.is_none() {
            SpanView { ptr: None, length: 0 }
        } else {
            SpanView { ptr: Some(NonNull::new(self.0.as_mut().unwrap().as_mut_slice().as_ptr() as *mut T).unwrap()), length: self.0.as_ref().unwrap().len() }
        }
    }
}

impl <T> PointerTrait for Vector<T> { }

impl <T> Deref for Vector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.as_rust_slice()[0]
    }
}

impl <T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.as_rust_slice_mut()[0]
    }
}

impl <T> Default for Vector<T> {
    fn default() -> Self {
        Vector(None)
    }
}

impl <T> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_none() && other.0.is_none() {
            true
        } else {
            false
        }
    }
}

impl <T> From<Null> for Vector<T> {
    fn from(_: Null) -> Self {
        Vector(None)
    }
}

impl <T, I: Integer> Index<I> for Vector<T> {
    type Output = T;
    fn index(&self, index: I) -> &T {
        &self.as_rust_slice()[index.as_usize()]
    }
}

impl <T, I: Integer> IndexMut<I> for Vector<T> {
    fn index_mut(&mut self, index: I) -> &mut T {
        &mut self.as_rust_slice_mut()[index.as_usize()]
    }
}

impl <T> Index<Range<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<Range<usize>> for Vector<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeTo<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeTo<usize>> for Vector<T> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeFrom<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeFrom<usize>> for Vector<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeFull> for Vector<T> {
    type Output = [T];
    fn index(&self, index: RangeFull) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeFull> for Vector<T> {
    fn index_mut(&mut self, index: RangeFull) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeInclusive<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: RangeInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeInclusive<usize>> for Vector<T> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeToInclusive<usize>> for Vector<T> {
    type Output = [T];
    fn index(&self, index: RangeToInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeToInclusive<usize>> for Vector<T> {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}