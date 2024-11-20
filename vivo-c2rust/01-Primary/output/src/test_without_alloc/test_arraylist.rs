#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn arraylist_new(length: libc::c_uint) -> *mut ArrayList;
    fn arraylist_free(arraylist: *mut ArrayList);
    fn arraylist_append(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_prepend(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_remove(arraylist: *mut ArrayList, index: libc::c_uint);
    fn arraylist_remove_range(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        length: libc::c_uint,
    );
    fn arraylist_insert(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_index_of(
        arraylist: *mut ArrayList,
        callback: ArrayListEqualFunc,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_clear(arraylist: *mut ArrayList);
    fn arraylist_sort(arraylist: *mut ArrayList, compare_func: ArrayListCompareFunc);
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type ArrayListValue = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ArrayList {
    pub data: *mut ArrayListValue,
    pub length: libc::c_uint,
    pub _alloced: libc::c_uint,
}
pub type ArrayList = _ArrayList;
pub type ArrayListEqualFunc = Option::<
    unsafe extern "C" fn(ArrayListValue, ArrayListValue) -> libc::c_int,
>;
pub type ArrayListCompareFunc = Option::<
    unsafe extern "C" fn(ArrayListValue, ArrayListValue) -> libc::c_int,
>;
#[no_mangle]
pub static mut variable1: libc::c_int = 0;
#[no_mangle]
pub static mut variable2: libc::c_int = 0;
#[no_mangle]
pub static mut variable3: libc::c_int = 0;
#[no_mangle]
pub static mut variable4: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn generate_arraylist() -> *mut ArrayList {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        arraylist_append(
            arraylist,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable2 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable3 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        );
        i += 1;
        i;
    }
    return arraylist;
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_new_free() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    if !arraylist.is_null() {} else {
        __assert_fail(
            b"arraylist != NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_1798: {
        if !arraylist.is_null() {} else {
            __assert_fail(
                b"arraylist != NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
    arraylist = arraylist_new(10 as libc::c_int as libc::c_uint);
    if !arraylist.is_null() {} else {
        __assert_fail(
            b"arraylist != NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_1739: {
        if !arraylist.is_null() {} else {
            __assert_fail(
                b"arraylist != NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
    arraylist_free(0 as *mut ArrayList);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_append() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2506: {
        if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_append(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable1) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2458: {
        if arraylist_append(
            arraylist,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, &variable1) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2416: {
        if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                58 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_append(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable2) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2368: {
        if arraylist_append(
            arraylist,
            &mut variable2 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, &variable2) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                60 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2326: {
        if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_append(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable3) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2278: {
        if arraylist_append(
            arraylist,
            &mut variable3 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, &variable3) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2236: {
        if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_append(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2187: {
        if arraylist_append(
            arraylist,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, &variable4) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2144: {
        if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                67 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2094: {
        if *((*arraylist).data).offset(0 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[0] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2044: {
        if *((*arraylist).data).offset(1 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[1] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_1994: {
        if *((*arraylist).data).offset(2 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[2] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_1944: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if arraylist_append(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, NULL) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1886: {
            if arraylist_append(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"arraylist_append(arraylist, NULL) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-arraylist.c\0" as *const u8
                        as *const libc::c_char,
                    77 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void test_arraylist_append(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    arraylist_free(arraylist);
    arraylist = arraylist_new(100 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_prepend() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_3213: {
        if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_prepend(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable1) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_3165: {
        if arraylist_prepend(
            arraylist,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, &variable1) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                98 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_3123: {
        if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                99 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_prepend(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable2) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_3075: {
        if arraylist_prepend(
            arraylist,
            &mut variable2 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, &variable2) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_3033: {
        if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_prepend(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable3) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2985: {
        if arraylist_prepend(
            arraylist,
            &mut variable3 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, &variable3) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2943: {
        if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_prepend(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2894: {
        if arraylist_prepend(
            arraylist,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, &variable4) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2852: {
        if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2802: {
        if *((*arraylist).data).offset(0 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[0] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2752: {
        if *((*arraylist).data).offset(1 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[1] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                111 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2702: {
        if *((*arraylist).data).offset(2 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[2] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2652: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                113 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if arraylist_prepend(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, NULL) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                118 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2594: {
            if arraylist_prepend(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"arraylist_prepend(arraylist, NULL) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-arraylist.c\0" as *const u8
                        as *const libc::c_char,
                    118 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 34],
                        &[libc::c_char; 34],
                    >(b"void test_arraylist_prepend(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    arraylist_free(arraylist);
    arraylist = arraylist_new(100 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_insert() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = generate_arraylist();
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4801: {
        if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                137 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_insert(
        arraylist,
        17 as libc::c_int as libc::c_uint,
        &mut variable1 as *mut libc::c_int as ArrayListValue,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_insert(arraylist, 17, &variable1) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4749: {
        if arraylist_insert(
            arraylist,
            17 as libc::c_int as libc::c_uint,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_insert(arraylist, 17, &variable1) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4707: {
        if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            143 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4665: {
        if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4615: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                144 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4565: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4515: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                146 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_insert(
        arraylist,
        5 as libc::c_int as libc::c_uint,
        &mut variable4 as *mut libc::c_int as ArrayListValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_insert(arraylist, 5, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4463: {
        if arraylist_insert(
            arraylist,
            5 as libc::c_int as libc::c_uint,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_insert(arraylist, 5, &variable4) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 17 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 17\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4421: {
        if (*arraylist).length == 17 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 17\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                150 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4371: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4321: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4271: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(7 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[7] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4221: {
        if *((*arraylist).data).offset(7 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[7] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                154 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4171: {
        if *((*arraylist).data).offset(0 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[0] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                158 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4121: {
        if *((*arraylist).data).offset(1 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[1] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                159 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4071: {
        if *((*arraylist).data).offset(2 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[2] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                160 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_insert(
        arraylist,
        0 as libc::c_int as libc::c_uint,
        &mut variable4 as *mut libc::c_int as ArrayListValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_insert(arraylist, 0, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_4018: {
        if arraylist_insert(
            arraylist,
            0 as libc::c_int as libc::c_uint,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_insert(arraylist, 0, &variable4) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 18 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 18\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3976: {
        if (*arraylist).length == 18 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 18\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                164 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3926: {
        if *((*arraylist).data).offset(0 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[0] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                165 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3876: {
        if *((*arraylist).data).offset(1 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[1] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3826: {
        if *((*arraylist).data).offset(2 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[2] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3776: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(15 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[15] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3726: {
        if *((*arraylist).data).offset(15 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[15] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(16 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[16] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3676: {
        if *((*arraylist).data).offset(16 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[16] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(17 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[17] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3626: {
        if *((*arraylist).data).offset(17 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[17] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if arraylist_insert(
        arraylist,
        18 as libc::c_int as libc::c_uint,
        &mut variable1 as *mut libc::c_int as ArrayListValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_insert(arraylist, 18, &variable1) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3573: {
        if arraylist_insert(
            arraylist,
            18 as libc::c_int as libc::c_uint,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_insert(arraylist, 18, &variable1) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                176 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (*arraylist).length == 19 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 19\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3530: {
        if (*arraylist).length == 19 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 19\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                178 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(15 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[15] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3480: {
        if *((*arraylist).data).offset(15 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[15] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(16 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[16] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3430: {
        if *((*arraylist).data).offset(16 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[16] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(17 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[17] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3380: {
        if *((*arraylist).data).offset(17 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[17] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                181 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(18 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[18] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_3330: {
        if *((*arraylist).data).offset(18 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[18] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                182 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        arraylist_insert(
            arraylist,
            10 as libc::c_int as libc::c_uint,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        );
        i += 1;
        i;
    }
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_remove_range() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    arraylist = generate_arraylist();
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5386: {
        if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5336: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                199 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5286: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                200 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5236: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                201 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5186: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_remove_range(
        arraylist,
        4 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    );
    if (*arraylist).length == 13 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 13\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5135: {
        if (*arraylist).length == 13 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 13\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                206 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5085: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                207 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_5035: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                208 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_4985: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                209 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_4935: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_remove_range(
        arraylist,
        10 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    );
    arraylist_remove_range(
        arraylist,
        0 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
    );
    if (*arraylist).length == 13 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 13\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    }
    'c_4874: {
        if (*arraylist).length == 13 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 13\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                217 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_arraylist_remove_range(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_remove() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    arraylist = generate_arraylist();
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5956: {
        if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                227 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5906: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5856: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                229 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5806: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5756: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_remove(arraylist, 4 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5707: {
        if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                235 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5657: {
        if *((*arraylist).data).offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[3] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                236 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5607: {
        if *((*arraylist).data).offset(4 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[4] == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                237 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5557: {
        if *((*arraylist).data).offset(5 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[5] == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                238 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5507: {
        if *((*arraylist).data).offset(6 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[6] == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                239 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_remove(arraylist, 15 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_5457: {
        if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                245 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_index_of() {
    let mut entries: [libc::c_int; 10] = [
        89 as libc::c_int,
        4 as libc::c_int,
        23 as libc::c_int,
        42 as libc::c_int,
        16 as libc::c_int,
        15 as libc::c_int,
        8 as libc::c_int,
        99 as libc::c_int,
        50 as libc::c_int,
        30 as libc::c_int,
    ];
    let mut num_entries: libc::c_int = 0;
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    num_entries = (::core::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < num_entries {
        arraylist_append(
            arraylist,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as ArrayListValue,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < num_entries {
        val = entries[i as usize];
        index = arraylist_index_of(
            arraylist,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ArrayListValue,
        );
        if index == i {} else {
            __assert_fail(
                b"index == i\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                275 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_index_of(void)\0"))
                    .as_ptr(),
            );
        }
        'c_6147: {
            if index == i {} else {
                __assert_fail(
                    b"index == i\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-arraylist.c\0" as *const u8
                        as *const libc::c_char,
                    275 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"void test_arraylist_index_of(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    val = 0 as libc::c_int;
    if arraylist_index_of(
        arraylist,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ArrayListValue,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_index_of(arraylist, int_equal, &val) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            281 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_index_of(void)\0"))
                .as_ptr(),
        );
    }
    'c_6085: {
        if arraylist_index_of(
            arraylist,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ArrayListValue,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_index_of(arraylist, int_equal, &val) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                281 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_index_of(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 57 as libc::c_int;
    if arraylist_index_of(
        arraylist,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ArrayListValue,
    ) < 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_index_of(arraylist, int_equal, &val) < 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            283 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_index_of(void)\0"))
                .as_ptr(),
        );
    }
    'c_6026: {
        if arraylist_index_of(
            arraylist,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ArrayListValue,
        ) < 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_index_of(arraylist, int_equal, &val) < 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                283 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_index_of(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_clear() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    arraylist_clear(arraylist);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_arraylist_clear(void)\0"))
                .as_ptr(),
        );
    }
    'c_6370: {
        if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                296 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_arraylist_clear(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_append(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue);
    arraylist_clear(arraylist);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            307 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_arraylist_clear(void)\0"))
                .as_ptr(),
        );
    }
    'c_6290: {
        if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                307 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_arraylist_clear(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_sort() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut entries: [libc::c_int; 13] = [
        89 as libc::c_int,
        4 as libc::c_int,
        23 as libc::c_int,
        42 as libc::c_int,
        4 as libc::c_int,
        16 as libc::c_int,
        15 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
        99 as libc::c_int,
        50 as libc::c_int,
        30 as libc::c_int,
        4 as libc::c_int,
    ];
    let mut sorted: [libc::c_int; 13] = [
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
        23 as libc::c_int,
        30 as libc::c_int,
        42 as libc::c_int,
        50 as libc::c_int,
        89 as libc::c_int,
        99 as libc::c_int,
    ];
    let mut num_entries: libc::c_uint = (::core::mem::size_of::<[libc::c_int; 13]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
    let mut i: libc::c_uint = 0;
    arraylist = arraylist_new(10 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        arraylist_prepend(
            arraylist,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as ArrayListValue,
        );
        i = i.wrapping_add(1);
        i;
    }
    arraylist_sort(
        arraylist,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if (*arraylist).length == num_entries {} else {
        __assert_fail(
            b"arraylist->length == num_entries\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_6764: {
        if (*arraylist).length == num_entries {} else {
            __assert_fail(
                b"arraylist->length == num_entries\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                329 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
        value = *((*arraylist).data).offset(i as isize) as *mut libc::c_int;
        if *value == sorted[i as usize] {} else {
            __assert_fail(
                b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                337 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
        }
        'c_6668: {
            if *value == sorted[i as usize] {} else {
                __assert_fail(
                    b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-arraylist.c\0" as *const u8
                        as *const libc::c_char,
                    337 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_arraylist_sort(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    arraylist_free(arraylist);
    arraylist = arraylist_new(5 as libc::c_int as libc::c_uint);
    arraylist_sort(
        arraylist,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_6600: {
        if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                348 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
    arraylist = arraylist_new(5 as libc::c_int as libc::c_uint);
    arraylist_prepend(
        arraylist,
        &mut *entries.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_int
            as ArrayListValue,
    );
    arraylist_sort(
        arraylist,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_6527: {
        if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                359 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut *entries.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &entries[0]\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_6453: {
        if *((*arraylist).data).offset(0 as libc::c_int as isize)
            == &mut *entries.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut libc::c_int as ArrayListValue
        {} else {
            __assert_fail(
                b"arraylist->data[0] == &entries[0]\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-arraylist.c\0" as *const u8
                    as *const libc::c_char,
                360 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    arraylist_free(arraylist);
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
