use crate::translation_utils::*;

use core::ops::*;

pub trait PointerTrait: From<Null> + Deref + PartialEq + Default { }