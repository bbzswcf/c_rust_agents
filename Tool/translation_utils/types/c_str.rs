use crate::translation_utils::*;

use core::ops::*;

use core::ptr::NonNull;

// pub type CStr = Ptr<[u8]>;

pub struct CStr {
    pub ptr: Option<NonNull<u8>>,
    pub length: usize,
    pub _allocated: bool,
}

impl PrimitiveType for CStr { }

impl std::fmt::Debug for CStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.to_rust_slice())
    }
}

impl CStr {
    pub fn len(&self) -> usize {
        self.length
    }

    pub const fn from(data: &[u8]) -> Self {
        let length = data.len();
        let ptr: NonNull<u8> = unsafe {
            NonNull::new_unchecked(data.as_ptr() as *mut u8)
        };
        CStr { ptr: Some(ptr), length: length, _allocated: false }
    }

    pub fn to_rust_slice(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.ptr.unwrap().as_ptr(), self.length)
        }
    }

    pub fn to_rust_slice_mut(&mut self) -> &mut [u8] {
        if !self._allocated {
            panic!("Cannot mutate a non-allocated CStr");
        }
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr.unwrap().as_ptr(), self.length)
        }
    }

    pub fn new_alloced(length: usize) -> Self {
        // Create a vector and leak it
        let data = vec![0; length];
        let mut v = core::mem::ManuallyDrop::new(data);
        let p = v.as_mut_ptr();
        CStr { ptr: Some(NonNull::new(p).unwrap()), length: length, _allocated: true }
    }

    pub fn drop(&mut self) {
        if self._allocated {
            unsafe {
                let p = self.ptr.unwrap().as_mut();
                let _ = Vec::from_raw_parts(p, self.length, self.length);
            }
        }
        self.ptr = None;
    }

    pub fn addr(&self) -> usize {
        self.ptr.unwrap().as_ptr() as usize
    }

    pub fn unowned(&mut self) -> CStr {
        CStr { ptr: self.ptr, length: self.length, _allocated: false }
    }

    pub fn to_view(&self) -> SpanView<u8> {
        SpanView { ptr: self.ptr, length: self.length }
    }
}

macro_rules! cstr {
    ($string: literal) => {
        CStr::from(concat!($string, "\0").as_bytes())
    };
}

pub(crate) use cstr;

impl Clone for CStr {
    fn clone(&self) -> Self {
        CStr { ptr: self.ptr, length: self.length, _allocated: false }
    }
}

impl PointerTrait for CStr { }

impl From<Null> for CStr {
    fn from(_: Null) -> Self {
        CStr { ptr: None, length: 0, _allocated: false }
    }
}

impl Default for CStr {
    fn default() -> Self {
        CStr { ptr: None, length: 0, _allocated: false }
    }
}

impl Copy for CStr {}

impl PartialEq for CStr {
    fn eq(&self, other: &Self) -> bool {
        if self.ptr.is_none() && other.ptr.is_none() {
            return true;
        }
        if self.ptr.is_none() || other.ptr.is_none() {
            return false;
        }
        self.to_rust_slice() == other.to_rust_slice()
    }
}

impl PartialOrd for CStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ptr.is_none() && other.ptr.is_none() {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.ptr.is_none() {
            return Some(std::cmp::Ordering::Less);
        }
        if other.ptr.is_none() {
            return Some(std::cmp::Ordering::Greater);
        }
        self.to_rust_slice().partial_cmp(&other.to_rust_slice())
    }
}

impl <I: Integer> Index<I> for CStr {
    type Output = u8;
    fn index(&self, index: I) -> &u8 {
        &self.to_rust_slice()[index.as_usize()]
    }
}

impl <I: Integer> IndexMut<I> for CStr {
    fn index_mut(&mut self, index: I) -> &mut u8 {
        if !self._allocated {
            panic!("Cannot mutate a non-allocated CStr");
        }
        unsafe {
            &mut self.to_rust_slice_mut()[index.as_usize()]
        }
    }
}

impl Deref for CStr {
    type Target = u8;
    fn deref(&self) -> &u8 {
        unsafe {
            self.ptr.unwrap().as_ref()
        }
    }
}

impl DerefMut for CStr {
    fn deref_mut(&mut self) -> &mut u8 {
        if !self._allocated {
            panic!("Cannot mutate a non-allocated CStr");
        }
        unsafe {
            self.ptr.unwrap().as_mut()
        }
    }
}

impl <I: Integer> Add<I> for CStr {
    type Output = CStr;
    fn add(self, rhs: I) -> CStr {
        let start_ptr = self.ptr.unwrap().as_ptr() as *const u8;
        let current_length = self.length;
        if current_length < rhs.as_usize() {
            panic!("Index out of bounds");
        }
        let data: &[u8] = unsafe { core::slice::from_raw_parts(start_ptr.add(rhs.as_usize()), current_length - rhs.as_usize()) };
        CStr { ptr: Some(NonNull::new(data.as_ptr() as *mut u8).unwrap()), length: current_length - rhs.as_usize(), _allocated: false }
    }
}

impl <I: Integer> AddAssign<I> for CStr {
    fn add_assign(&mut self, rhs: I) {
        if self._allocated {
            panic!("Cannot assign to a allocated CStr");
        }
        let start_ptr = self.ptr.unwrap().as_ptr() as *const u8;
        let current_length = self.length;
        if current_length < rhs.as_usize() {
            panic!("Index out of bounds");
        }
        let data: &[u8] = unsafe { core::slice::from_raw_parts(start_ptr.add(rhs.as_usize()), current_length - rhs.as_usize()) };
        self.ptr = Some(NonNull::new(data.as_ptr() as *mut u8).unwrap());
        self.length = current_length - rhs.as_usize();
        self._allocated = false;
    }
}