#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub extern "C" fn pointer_hash(mut location: *mut libc::c_void) -> libc::c_uint {
    location as libc::c_ulong as libc::c_uint
}
