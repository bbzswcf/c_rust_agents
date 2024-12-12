use crate::translation_utils::*;

pub trait Integer: PrimitiveType {
    fn as_bool(self) -> bool;
    fn as_usize(self) -> usize;
}

impl Integer for i8 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for i16 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for i32 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for i64 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for i128 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for isize {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for u8 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for u16 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for u32 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for u64 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for u128 {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self as usize
    }
}

impl Integer for usize {
    fn as_bool(self) -> bool {
        self != 0
    }

    fn as_usize(self) -> usize {
        self
    }
}

impl Integer for bool {
    fn as_bool(self) -> bool {
        self
    }

    fn as_usize(self) -> usize {
        if self {
            1
        } else {
            0
        }
    }
}