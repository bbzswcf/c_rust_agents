use crate::translation_utils::*;

use core::ops::*;
use core::ptr::NonNull;

pub struct SpanView<T> {
    pub ptr: Option<NonNull<T>>,
    pub length: usize,
}

impl <T> SpanView<T> {
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn convert<T2>(self) -> SpanView<T2> {
        SpanView { ptr: self.ptr.map(|ptr| ptr.cast()), length: self.length * std::mem::size_of::<T>() / std::mem::size_of::<T2>() }
    }
    
    pub fn is_null(&self) -> bool {
        self.ptr.is_none()
    }

    pub fn as_rust_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.unwrap().as_ptr(), self.length) }
    }

    pub fn as_rust_slice_mut(&self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.unwrap().as_ptr(), self.length) }
    }
}

impl <T> PointerTrait for SpanView<T> { }

impl <T> Default for SpanView<T> {
    fn default() -> Self {
        SpanView { ptr: None, length: 0 }
    }
}

impl <T> PartialEq for SpanView<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.ptr.is_none() && other.ptr.is_none() {
            true
        } else if self.ptr.is_none() || other.ptr.is_none() {
            false
        } else {
            self.ptr.unwrap().as_ptr() == other.ptr.unwrap().as_ptr()
        }
    }
}

impl <T> PartialOrd for SpanView<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ptr.is_none() && other.ptr.is_none() {
            Some(std::cmp::Ordering::Equal)
        } else if self.ptr.is_none() || other.ptr.is_none() {
            None
        } else {
            self.ptr.unwrap().as_ptr().partial_cmp(&other.ptr.unwrap().as_ptr())
        }
    }
}

impl <T> Deref for SpanView<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.ptr.unwrap().as_ref() }
    }
}

impl <T> DerefMut for SpanView<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.ptr.unwrap().as_mut() }
    }
}

impl <T> Clone for SpanView<T> {
    fn clone(&self) -> Self {
        SpanView { ptr: self.ptr, length: self.length }
    }
}

impl <T> Copy for SpanView<T> { }

impl <T> From<Null> for SpanView<T> {
    fn from(_: Null) -> Self {
        SpanView { ptr: None, length: 0 }
    }
}

impl <T, I: Integer> Index<I> for SpanView<T> {
    type Output = T;
    fn index(&self, index: I) -> &T {
        // &self.as_rust_slice()[index.as_usize()]
        unsafe { self.ptr.unwrap().as_ptr().add(index.as_usize()).as_ref().unwrap() }
    }
}

impl <T, I: Integer> IndexMut<I> for SpanView<T> {
    fn index_mut(&mut self, index: I) -> &mut T {
        // &mut self.as_rust_slice_mut()[index.as_usize()]
        unsafe { self.ptr.unwrap().as_ptr().add(index.as_usize()).as_mut().unwrap() }
    }
}

impl <T> Index<Range<usize>> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: Range<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<Range<usize>> for SpanView<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeTo<usize>> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: RangeTo<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeTo<usize>> for SpanView<T> {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeFrom<usize>> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: RangeFrom<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeFrom<usize>> for SpanView<T> {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeFull> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: RangeFull) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeFull> for SpanView<T> {
    fn index_mut(&mut self, index: RangeFull) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeInclusive<usize>> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: RangeInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeInclusive<usize>> for SpanView<T> {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T> Index<RangeToInclusive<usize>> for SpanView<T> {
    type Output = [T];
    fn index(&self, index: RangeToInclusive<usize>) -> &[T] {
        &self.as_rust_slice()[index]
    }
}

impl <T> IndexMut<RangeToInclusive<usize>> for SpanView<T> {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut [T] {
        &mut self.as_rust_slice_mut()[index]
    }
}

impl <T, I: Integer> Add<I> for SpanView<T> {
    type Output = SpanView<T>;
    fn add(self, rhs: I) -> SpanView<T> {
        if self.length < rhs.as_usize() {
            panic!("Index out of bounds");
        }
        SpanView { ptr: self.ptr.map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().add(rhs.as_usize())) }), length: self.length - rhs.as_usize() }
    }
}

impl <T, I: Integer> AddAssign<I> for SpanView<T> {
    fn add_assign(&mut self, rhs: I) {
        if self.length < rhs.as_usize() {
            panic!("Index out of bounds");
        }
        self.ptr = self.ptr.map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().add(rhs.as_usize())) });
        self.length -= rhs.as_usize();
    }
}

impl <T, I: Integer> Sub<I> for SpanView<T> {
    type Output = SpanView<T>;
    fn sub(self, rhs: I) -> SpanView<T> {
        SpanView { ptr: self.ptr.map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().sub(rhs.as_usize())) }), length: self.length + rhs.as_usize() }
    }
}

impl <T, I: Integer> SubAssign<I> for SpanView<T> {
    fn sub_assign(&mut self, rhs: I) {
        self.ptr = self.ptr.map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_ptr().sub(rhs.as_usize())) });
        self.length += rhs.as_usize();
    }
}

impl <T> Sub<SpanView<T>> for SpanView<T> {
    type Output = usize;
    fn sub(self, rhs: SpanView<T>) -> usize {
        if self.ptr.unwrap().as_ptr() < rhs.ptr.unwrap().as_ptr() {
            panic!("Subtraction overflow");
        }
        self.ptr.unwrap().as_ptr() as usize - rhs.ptr.unwrap().as_ptr() as usize
    }
}
