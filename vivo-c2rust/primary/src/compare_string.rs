#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn tolower(_: libc::c_int) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub extern "C" fn string_equal(
    mut string1: *mut libc::c_void,
    mut string2: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        (strcmp(string1 as *mut libc::c_char, string2 as *mut libc::c_char) == 0) as libc::c_int
    }
}
#[no_mangle]
pub extern "C" fn string_compare(
    mut string1: *mut libc::c_void,
    mut string2: *mut libc::c_void,
) -> libc::c_int {
    let result = unsafe { strcmp(string1 as *mut libc::c_char, string2 as *mut libc::c_char) };
    if result < 0 {
        -1
    } else if result > 0 {
        1
    } else {
        0
    }
}
#[no_mangle]
pub extern "C" fn string_nocase_compare(
    mut string1: *mut libc::c_void,
    mut string2: *mut libc::c_void,
) -> libc::c_int {
    let mut p1: *mut libc::c_char = string1 as *mut libc::c_char;
    let mut p2: *mut libc::c_char = string2 as *mut libc::c_char;
    loop {
        let c1 = unsafe { tolower(*p1 as libc::c_int) };
        let c2 = unsafe { tolower(*p2 as libc::c_int) };
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
        if c1 == '\0' as i32 {
            break;
        }
        p1 = unsafe { p1.offset(1) };
        p2 = unsafe { p2.offset(1) };
    }
    return 0;
}
#[no_mangle]
pub extern "C" fn string_nocase_equal(
    mut string1: *mut libc::c_void,
    mut string2: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        (string_nocase_compare(
            string1 as *mut libc::c_char as *mut libc::c_void,
            string2 as *mut libc::c_char as *mut libc::c_void,
        ) == 0) as libc::c_int
    }
}
