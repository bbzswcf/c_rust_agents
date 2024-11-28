use crate::translation_utils::*;

use std::sync::{Mutex, MutexGuard};

pub struct GlobalVar<T>(pub Mutex<T>);

impl<T> GlobalVar<T> {
    pub const fn new(value: T) -> Self {
        GlobalVar(Mutex::new(value))
    }

    pub fn get(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}