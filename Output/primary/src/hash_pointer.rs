

pub fn pointer_hash(location: *const ()) -> u32 {
    location as u32
}