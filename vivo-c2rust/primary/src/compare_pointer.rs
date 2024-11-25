#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub extern "C" fn pointer_equal(
    mut location1: *mut libc::c_void,
    mut location2: *mut libc::c_void,
) -> libc::c_int {
    (location1 == location2) as libc::c_int
}
#[no_mangle]
pub extern "C" fn pointer_compare(
    mut location1: *mut libc::c_void,
    mut location2: *mut libc::c_void,
) -> libc::c_int {
    if location1 < location2 {
        -(1 as libc::c_int)
    } else if location1 > location2 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }
}
