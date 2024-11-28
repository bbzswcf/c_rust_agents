use crate::translation_utils::*;

use core::ops::*;

pub struct Null();

macro_rules! null {
    () => { Null().into() };
}

pub(crate) use null;