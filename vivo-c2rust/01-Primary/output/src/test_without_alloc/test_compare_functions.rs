#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn pointer_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn pointer_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_equal(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_compare(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_nocase_equal(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_nocase_compare(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_int_compare() {
    let mut a: libc::c_int = 4 as libc::c_int;
    let mut b: libc::c_int = 8 as libc::c_int;
    let mut c: libc::c_int = 4 as libc::c_int;
    if int_compare(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut b as *mut libc::c_int as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"int_compare(&a, &b) < 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            15 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_int_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_1719: {
        if int_compare(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut b as *mut libc::c_int as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_compare(&a, &b) < 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                15 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_int_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if int_compare(
        &mut b as *mut libc::c_int as *mut libc::c_void,
        &mut a as *mut libc::c_int as *mut libc::c_void,
    ) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"int_compare(&b, &a) > 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            19 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_int_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_1666: {
        if int_compare(
            &mut b as *mut libc::c_int as *mut libc::c_void,
            &mut a as *mut libc::c_int as *mut libc::c_void,
        ) > 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_compare(&b, &a) > 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                19 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_int_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if int_compare(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut c as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"int_compare(&a, &c) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_int_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_1606: {
        if int_compare(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut c as *mut libc::c_int as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_compare(&a, &c) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                23 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_int_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_int_equal() {
    let mut a: libc::c_int = 4 as libc::c_int;
    let mut b: libc::c_int = 8 as libc::c_int;
    let mut c: libc::c_int = 4 as libc::c_int;
    if int_equal(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut c as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"int_equal(&a, &c) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            33 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_int_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_1832: {
        if int_equal(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut c as *mut libc::c_int as *mut libc::c_void,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_equal(&a, &c) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                33 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_int_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if int_equal(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut b as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"int_equal(&a, &b) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            37 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_int_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_1776: {
        if int_equal(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut b as *mut libc::c_int as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_equal(&a, &b) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                37 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_int_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_pointer_compare() {
    let mut array: [libc::c_int; 5] = [0; 5];
    if pointer_compare(
        &mut *array.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"pointer_compare(&array[0], &array[4]) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_pointer_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2019: {
        if pointer_compare(
            &mut *array.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
            &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"pointer_compare(&array[0], &array[4]) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                46 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_pointer_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if pointer_compare(
        &mut *array.as_mut_ptr().offset(3 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *array.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
    ) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"pointer_compare(&array[3], &array[2]) > 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_pointer_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_1956: {
        if pointer_compare(
            &mut *array.as_mut_ptr().offset(3 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
            &mut *array.as_mut_ptr().offset(2 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
        ) > 0 as libc::c_int
        {} else {
            __assert_fail(
                b"pointer_compare(&array[3], &array[2]) > 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_pointer_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if pointer_compare(
        &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
        &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize) as *mut libc::c_int
            as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"pointer_compare(&array[4], &array[4]) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_pointer_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_1890: {
        if pointer_compare(
            &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
            &mut *array.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_int as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"pointer_compare(&array[4], &array[4]) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_pointer_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_pointer_equal() {
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if pointer_equal(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut a as *mut libc::c_int as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"pointer_equal(&a, &a) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_pointer_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2140: {
        if pointer_equal(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut a as *mut libc::c_int as *mut libc::c_void,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"pointer_equal(&a, &a) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_pointer_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if pointer_equal(
        &mut a as *mut libc::c_int as *mut libc::c_void,
        &mut b as *mut libc::c_int as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"pointer_equal(&a, &b) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_pointer_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2086: {
        if pointer_equal(
            &mut a as *mut libc::c_int as *mut libc::c_void,
            &mut b as *mut libc::c_int as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"pointer_equal(&a, &b) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                67 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_pointer_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_compare() {
    let mut test1: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Apple\0");
    let mut test2: [libc::c_char; 7] = *::core::mem::transmute::<
        &[u8; 7],
        &mut [libc::c_char; 7],
    >(b"Orange\0");
    let mut test3: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Apple\0");
    if string_compare(
        test1.as_mut_ptr() as *mut libc::c_void,
        test2.as_mut_ptr() as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_compare(test1, test2) < 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_string_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2309: {
        if string_compare(
            test1.as_mut_ptr() as *mut libc::c_void,
            test2.as_mut_ptr() as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_compare(test1, test2) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_string_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_compare(
        test2.as_mut_ptr() as *mut libc::c_void,
        test1.as_mut_ptr() as *mut libc::c_void,
    ) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_compare(test2, test1) > 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_string_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2255: {
        if string_compare(
            test2.as_mut_ptr() as *mut libc::c_void,
            test1.as_mut_ptr() as *mut libc::c_void,
        ) > 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_compare(test2, test1) > 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_string_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_compare(
        test1.as_mut_ptr() as *mut libc::c_void,
        test3.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_compare(test1, test3) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            85 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_string_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2198: {
        if string_compare(
            test1.as_mut_ptr() as *mut libc::c_void,
            test3.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_compare(test1, test3) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                85 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_string_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_equal() {
    let mut test1: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test string\0");
    let mut test2: [libc::c_char; 23] = *::core::mem::transmute::<
        &[u8; 23],
        &mut [libc::c_char; 23],
    >(b"this is a test string \0");
    let mut test3: [libc::c_char; 21] = *::core::mem::transmute::<
        &[u8; 21],
        &mut [libc::c_char; 21],
    >(b"this is a test strin\0");
    let mut test4: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test strinG\0");
    let mut test5: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test string\0");
    if string_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test5.as_mut_ptr() as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_equal(test1, test5) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_string_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2532: {
        if string_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test5.as_mut_ptr() as *mut libc::c_void,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_equal(test1, test5) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_string_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test2.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_equal(test1, test2) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_string_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2479: {
        if string_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test2.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_equal(test1, test2) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_string_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test3.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_equal(test1, test3) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_string_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2426: {
        if string_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test3.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_equal(test1, test3) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_string_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test4.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_equal(test1, test4) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_string_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2370: {
        if string_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test4.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_equal(test1, test4) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_string_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_nocase_compare() {
    let mut test1: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Apple\0");
    let mut test2: [libc::c_char; 7] = *::core::mem::transmute::<
        &[u8; 7],
        &mut [libc::c_char; 7],
    >(b"Orange\0");
    let mut test3: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Apple\0");
    let mut test4: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"Alpha\0");
    let mut test5: [libc::c_char; 6] = *::core::mem::transmute::<
        &[u8; 6],
        &mut [libc::c_char; 6],
    >(b"bravo\0");
    let mut test6: [libc::c_char; 8] = *::core::mem::transmute::<
        &[u8; 8],
        &mut [libc::c_char; 8],
    >(b"Charlie\0");
    if string_nocase_compare(
        test1.as_mut_ptr() as *mut libc::c_void,
        test2.as_mut_ptr() as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_compare(test1, test2) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            119 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_string_nocase_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2812: {
        if string_nocase_compare(
            test1.as_mut_ptr() as *mut libc::c_void,
            test2.as_mut_ptr() as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_compare(test1, test2) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_string_nocase_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_compare(
        test2.as_mut_ptr() as *mut libc::c_void,
        test1.as_mut_ptr() as *mut libc::c_void,
    ) > 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_compare(test2, test1) > 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_string_nocase_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2760: {
        if string_nocase_compare(
            test2.as_mut_ptr() as *mut libc::c_void,
            test1.as_mut_ptr() as *mut libc::c_void,
        ) > 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_compare(test2, test1) > 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_string_nocase_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_compare(
        test1.as_mut_ptr() as *mut libc::c_void,
        test3.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_compare(test1, test3) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            127 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_string_nocase_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2705: {
        if string_nocase_compare(
            test1.as_mut_ptr() as *mut libc::c_void,
            test3.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_compare(test1, test3) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                127 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_string_nocase_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_compare(
        test4.as_mut_ptr() as *mut libc::c_void,
        test5.as_mut_ptr() as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_compare(test4, test5) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_string_nocase_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2653: {
        if string_nocase_compare(
            test4.as_mut_ptr() as *mut libc::c_void,
            test5.as_mut_ptr() as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_compare(test4, test5) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                131 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_string_nocase_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_compare(
        test5.as_mut_ptr() as *mut libc::c_void,
        test6.as_mut_ptr() as *mut libc::c_void,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_compare(test5, test6) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            132 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_string_nocase_compare(void)\0"))
                .as_ptr(),
        );
    }
    'c_2597: {
        if string_nocase_compare(
            test5.as_mut_ptr() as *mut libc::c_void,
            test6.as_mut_ptr() as *mut libc::c_void,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_compare(test5, test6) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_string_nocase_compare(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_string_nocase_equal() {
    let mut test1: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test string\0");
    let mut test2: [libc::c_char; 23] = *::core::mem::transmute::<
        &[u8; 23],
        &mut [libc::c_char; 23],
    >(b"this is a test string \0");
    let mut test3: [libc::c_char; 21] = *::core::mem::transmute::<
        &[u8; 21],
        &mut [libc::c_char; 21],
    >(b"this is a test strin\0");
    let mut test4: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test strinG\0");
    let mut test5: [libc::c_char; 22] = *::core::mem::transmute::<
        &[u8; 22],
        &mut [libc::c_char; 22],
    >(b"this is a test string\0");
    if string_nocase_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test5.as_mut_ptr() as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_equal(test1, test5) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void test_string_nocase_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_3033: {
        if string_nocase_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test5.as_mut_ptr() as *mut libc::c_void,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_equal(test1, test5) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                144 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_string_nocase_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test2.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_equal(test1, test2) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void test_string_nocase_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2981: {
        if string_nocase_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test2.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_equal(test1, test2) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_string_nocase_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test3.as_mut_ptr() as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_equal(test1, test3) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            150 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void test_string_nocase_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2929: {
        if string_nocase_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test3.as_mut_ptr() as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_equal(test1, test3) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                150 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_string_nocase_equal(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if string_nocase_equal(
        test1.as_mut_ptr() as *mut libc::c_void,
        test4.as_mut_ptr() as *mut libc::c_void,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"string_nocase_equal(test1, test4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-compare-functions.c\0" as *const u8
                as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void test_string_nocase_equal(void)\0"))
                .as_ptr(),
        );
    }
    'c_2873: {
        if string_nocase_equal(
            test1.as_mut_ptr() as *mut libc::c_void,
            test4.as_mut_ptr() as *mut libc::c_void,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"string_nocase_equal(test1, test4) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-compare-functions.c\0" as *const u8
                    as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_string_nocase_equal(void)\0"))
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
