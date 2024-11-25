#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn tolower(_: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub extern "C" fn string_hash(mut string: *mut libc::c_void) -> libc::c_uint {
    let mut result: libc::c_uint = 5381;
    let mut p: *mut libc::c_uchar = string as *mut libc::c_uchar;
    unsafe {
        while *p as libc::c_int != 0 {
            result = (result << 5).wrapping_add(result).wrapping_add(*p as libc::c_uint);
            p = p.offset(1);
        }
    }
    result
}
#[no_mangle]
pub extern "C" fn string_nocase_hash(
    mut string: *mut libc::c_void,
) -> libc::c_uint {
    let mut result: libc::c_uint = 5381;
    let mut p: *mut libc::c_uchar = string as *mut libc::c_uchar;
    while unsafe { *p as libc::c_int != '\0' as i32 } {
        result = (result << 5).wrapping_add(result).wrapping_add(unsafe { tolower(*p as libc::c_int) } as libc::c_uint);
        p = unsafe { p.offset(1) };
    }
    result
}
