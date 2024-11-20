#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pointer_hash(location: *mut libc::c_void) -> libc::c_uint;
    fn int_hash(location: *mut libc::c_void) -> libc::c_uint;
    fn string_hash(string: *mut libc::c_void) -> libc::c_uint;
    fn string_nocase_hash(string: *mut libc::c_void) -> libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn test_pointer_hash() {
    let mut array: [libc::c_int; 200] = [0; 200];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        array[i as usize] = 0 as libc::c_int;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < 200 as libc::c_int {
            if pointer_hash(
                &mut *array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as *mut libc::c_void,
            )
                != pointer_hash(
                    &mut *array.as_mut_ptr().offset(j as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                )
            {} else {
                __assert_fail(
                    b"pointer_hash(&array[i]) != pointer_hash(&array[j])\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-hash-functions.c\0" as *const u8
                        as *const libc::c_char,
                    25 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void test_pointer_hash(void)\0"))
                        .as_ptr(),
                );
            }
            'c_1598: {
                if pointer_hash(
                    &mut *array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                )
                    != pointer_hash(
                        &mut *array.as_mut_ptr().offset(j as isize) as *mut libc::c_int
                            as *mut libc::c_void,
                    )
                {} else {
                    __assert_fail(
                        b"pointer_hash(&array[i]) != pointer_hash(&array[j])\0"
                            as *const u8 as *const libc::c_char,
                        b"test-without-alloc/test-hash-functions.c\0" as *const u8
                            as *const libc::c_char,
                        25 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 29],
                            &[libc::c_char; 29],
                        >(b"void test_pointer_hash(void)\0"))
                            .as_ptr(),
                    );
                }
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_int_hash() {
    let mut array: [libc::c_int; 200] = [0; 200];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        array[i as usize] = i;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 200 as libc::c_int {
        j = i + 1 as libc::c_int;
        while j < 200 as libc::c_int {
            if int_hash(
                &mut *array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as *mut libc::c_void,
            )
                != int_hash(
                    &mut *array.as_mut_ptr().offset(j as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                )
            {} else {
                __assert_fail(
                    b"int_hash(&array[i]) != int_hash(&array[j])\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-hash-functions.c\0" as *const u8
                        as *const libc::c_char,
                    44 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 25],
                        &[libc::c_char; 25],
                    >(b"void test_int_hash(void)\0"))
                        .as_ptr(),
                );
            }
            'c_1797: {
                if int_hash(
                    &mut *array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                        as *mut libc::c_void,
                )
                    != int_hash(
                        &mut *array.as_mut_ptr().offset(j as isize) as *mut libc::c_int
                            as *mut libc::c_void,
                    )
                {} else {
                    __assert_fail(
                        b"int_hash(&array[i]) != int_hash(&array[j])\0" as *const u8
                            as *const libc::c_char,
                        b"test-without-alloc/test-hash-functions.c\0" as *const u8
                            as *const libc::c_char,
                        44 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 25],
                            &[libc::c_char; 25],
                        >(b"void test_int_hash(void)\0"))
                            .as_ptr(),
                    );
                }
            };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    i = 5000 as libc::c_int;
    j = 5000 as libc::c_int;
    if int_hash(&mut i as *mut libc::c_int as *mut libc::c_void)
        == int_hash(&mut j as *mut libc::c_int as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"int_hash(&i) == int_hash(&j)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_int_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_1724: {
        if int_hash(&mut i as *mut libc::c_int as *mut libc::c_void)
            == int_hash(&mut j as *mut libc::c_int as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"int_hash(&i) == int_hash(&j)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_int_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_hash() {
    let mut test1: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a test\0");
    let mut test2: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a tesu\0");
    let mut test3: [libc::c_char; 16] = *::core::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_char; 16],
    >(b"this is a test \0");
    let mut test4: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a test\0");
    let mut test5: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"This is a test\0");
    if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
        != string_hash(test2.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_hash(test1) != string_hash(test2)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_string_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2090: {
        if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
            != string_hash(test2.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_hash(test1) != string_hash(test2)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                65 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_string_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
        != string_hash(test3.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_hash(test1) != string_hash(test3)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_string_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2033: {
        if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
            != string_hash(test3.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_hash(test1) != string_hash(test3)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_string_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
        != string_hash(test5.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_hash(test1) != string_hash(test5)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_string_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_1977: {
        if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
            != string_hash(test5.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_hash(test1) != string_hash(test5)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_string_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
        == string_hash(test4.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_hash(test1) == string_hash(test4)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_string_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_1917: {
        if string_hash(test1.as_mut_ptr() as *mut libc::c_void)
            == string_hash(test4.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_hash(test1) == string_hash(test4)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_string_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_nocase_hash() {
    let mut test1: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a test\0");
    let mut test2: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a tesu\0");
    let mut test3: [libc::c_char; 16] = *::core::mem::transmute::<
        &[u8; 16],
        &mut [libc::c_char; 16],
    >(b"this is a test \0");
    let mut test4: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"this is a test\0");
    let mut test5: [libc::c_char; 15] = *::core::mem::transmute::<
        &[u8; 15],
        &mut [libc::c_char; 15],
    >(b"This is a test\0");
    if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
        != string_nocase_hash(test2.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_nocase_hash(test1) != string_nocase_hash(test2)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            89 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_string_nocase_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2329: {
        if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
            != string_nocase_hash(test2.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_nocase_hash(test1) != string_nocase_hash(test2)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                89 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_string_nocase_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
        != string_nocase_hash(test3.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_nocase_hash(test1) != string_nocase_hash(test3)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_string_nocase_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2273: {
        if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
            != string_nocase_hash(test3.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_nocase_hash(test1) != string_nocase_hash(test3)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_string_nocase_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
        == string_nocase_hash(test5.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_nocase_hash(test1) == string_nocase_hash(test5)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_string_nocase_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2217: {
        if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
            == string_nocase_hash(test5.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_nocase_hash(test1) == string_nocase_hash(test5)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_string_nocase_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
        == string_nocase_hash(test4.as_mut_ptr() as *mut libc::c_void)
    {} else {
        __assert_fail(
            b"string_nocase_hash(test1) == string_nocase_hash(test4)\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-functions.c\0" as *const u8
                as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_string_nocase_hash(void)\0"))
                .as_ptr(),
        );
    }
    'c_2157: {
        if string_nocase_hash(test1.as_mut_ptr() as *mut libc::c_void)
            == string_nocase_hash(test4.as_mut_ptr() as *mut libc::c_void)
        {} else {
            __assert_fail(
                b"string_nocase_hash(test1) == string_nocase_hash(test4)\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-functions.c\0" as *const u8
                    as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_string_nocase_hash(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
