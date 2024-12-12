use crate::translation_utils::*;

use core::ops::*;

use core::fmt::Debug;

pub trait GenericValue: PartialEq + PartialOrd + Clone + Copy + Default + Debug + From<Null> {}

impl <T: ?Sized + PartialEq + PartialOrd + Clone + Copy + Default + Debug + From<Null>> GenericValue for T {}