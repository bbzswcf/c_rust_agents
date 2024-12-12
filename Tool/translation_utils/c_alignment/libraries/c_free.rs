use crate::translation_utils::*;

pub trait CFree {
    fn c_free(&mut self);
}

pub fn free<T: CFree>(mut ptr: T) {
    ptr.c_free();
}

impl <T: Default> CFree for Owned<T> {
    fn c_free(&mut self) {
        self.drop();
    }
}

impl <T: Default> CFree for Unowned<T> {
    fn c_free(&mut self) {
        self.drop();
    }
}

impl <T: Default> CFree for Vector<T> {
    fn c_free(&mut self) {
        self.drop();
    }
}

impl <T: Default> CFree for Manual<T> {
    fn c_free(&mut self) {
        self.drop();
    }
}

impl CFree for CStr {
    fn c_free(&mut self) {
        self.drop();
    }
}

impl <T: Default> CFree for Ptr<T> {
    fn c_free(&mut self) {
        unsafe {
            self.free_leaked();
        }
    }
}

macro_rules! c_free {
    () => {
        func!(free);
    };
    ($ptr:expr) => {
        $ptr.c_free();
    };
}
pub(crate) use c_free;