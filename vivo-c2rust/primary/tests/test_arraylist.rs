#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern crate libc;
use primary::arraylist::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
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
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_new_free(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_free(arraylist);
    arraylist = arraylist_new(10 as libc::c_int as libc::c_uint);
    if !arraylist.is_null() {} else {
        __assert_fail(
            b"arraylist != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_new_free(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_append(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable1) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_append(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable2) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_append(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable3) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_append(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_append(arraylist, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_append(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if arraylist_append(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_append(arraylist, NULL) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_arraylist_append(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_prepend() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_prepend(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable1) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 1 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_prepend(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable2) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_prepend(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable3) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if arraylist_prepend(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"arraylist_prepend(arraylist, &variable4) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_arraylist_prepend(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        if arraylist_prepend(arraylist, 0 as *mut libc::c_void) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"arraylist_prepend(arraylist, NULL) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_arraylist_prepend(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    arraylist_free(arraylist);
}
#[no_mangle]
pub unsafe extern "C" fn test_arraylist_insert() {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = generate_arraylist();
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 16 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 16\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            205 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 17 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 17\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(7 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[7] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 18 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 18\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(1 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[1] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(2 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[2] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(15 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[15] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(16 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[16] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(17 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[17] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if (*arraylist).length == 19 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 19\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(15 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[15] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(16 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[16] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(17 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[17] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(18 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[18] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            244 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_insert(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_remove_range(
        arraylist,
        4 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    );
    if (*arraylist).length == 13 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 13\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_arraylist_remove_range(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            294 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            295 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_remove(arraylist, 4 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(4 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[4] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(5 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[5] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(6 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[6] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            303 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_remove(arraylist, 15 as libc::c_int as libc::c_uint);
    if (*arraylist).length == 15 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 15\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_arraylist_remove(void)\0"))
                .as_ptr(),
        );
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
                b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_arraylist_index_of(void)\0"))
                    .as_ptr(),
            );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            346 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_index_of(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_arraylist_index_of(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            362 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_arraylist_clear(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_append(arraylist, &mut variable1 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable2 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable3 as *mut libc::c_int as ArrayListValue);
    arraylist_append(arraylist, &mut variable4 as *mut libc::c_int as ArrayListValue);
    arraylist_clear(arraylist);
    if (*arraylist).length == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"arraylist->length == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            373 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_arraylist_clear(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            396 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
        value = *((*arraylist).data).offset(i as isize) as *mut libc::c_int;
        if *value == sorted[i as usize] {} else {
            __assert_fail(
                b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
                404 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_arraylist_sort(void)\0"))
                    .as_ptr(),
            );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            426 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    };
    if *((*arraylist).data).offset(0 as libc::c_int as isize)
        == &mut *entries.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut libc::c_int as ArrayListValue
    {} else {
        __assert_fail(
            b"arraylist->data[0] == &entries[0]\0" as *const u8 as *const libc::c_char,
            b"test/test-arraylist.c\0" as *const u8 as *const libc::c_char,
            427 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_arraylist_sort(void)\0"))
                .as_ptr(),
        );
    };
    arraylist_free(arraylist);
}
static mut tests: [UnitTestFunction; 10] = unsafe {
    [
        Some(test_arraylist_new_free as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_append as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_prepend as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_insert as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_remove as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_remove_range as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_index_of as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_clear as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_sort as unsafe extern "C" fn() -> ()),
        None,
    ]
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    run_tests(tests.as_mut_ptr());
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


#[test]
fn s_test_arraylist_new_free() {
    unsafe {
        test_arraylist_new_free();
    }
}


#[test]
fn s_test_arraylist_append() {
    unsafe {
        test_arraylist_append();
    }
}


#[test]
fn s_test_arraylist_prepend() {
    unsafe {
        test_arraylist_prepend();
    }
}


#[test]
fn s_test_arraylist_insert() {
    unsafe {
        test_arraylist_insert();
    }
}


#[test]
fn s_test_arraylist_remove_range() {
    unsafe {
        test_arraylist_remove_range();
    }
}


#[test]
fn s_test_arraylist_remove() {
    unsafe {
        test_arraylist_remove();
    }
}


#[test]
fn s_test_arraylist_index_of() {
    unsafe {
        test_arraylist_index_of();
    }
}


#[test]
fn s_test_arraylist_clear() {
    unsafe {
        test_arraylist_clear();
    }
}


#[test]
fn s_test_arraylist_sort() {
    unsafe {
        test_arraylist_sort();
    }
}
