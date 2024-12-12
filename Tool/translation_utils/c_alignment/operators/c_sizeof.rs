use crate::translation_utils::*;

macro_rules! c_sizeof {
    ($t:ty) => {
        core::mem::size_of::<$t>()
    };
    ($v:ident) => {
        core::mem::size_of_val(&$v)
    };
}

pub(crate) use c_sizeof;