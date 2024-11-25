#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _ListEntry;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn list_free(list: *mut ListEntry);
    fn list_prepend(list: *mut *mut ListEntry, data: ListValue) -> *mut ListEntry;
    fn list_append(list: *mut *mut ListEntry, data: ListValue) -> *mut ListEntry;
    fn list_prev(listentry: *mut ListEntry) -> *mut ListEntry;
    fn list_next(listentry: *mut ListEntry) -> *mut ListEntry;
    fn list_data(listentry: *mut ListEntry) -> ListValue;
    fn list_nth_entry(list: *mut ListEntry, n: libc::c_uint) -> *mut ListEntry;
    fn list_nth_data(list: *mut ListEntry, n: libc::c_uint) -> ListValue;
    fn list_length(list: *mut ListEntry) -> libc::c_uint;
    fn list_to_array(list: *mut ListEntry) -> *mut ListValue;
    fn list_remove_entry(
        list: *mut *mut ListEntry,
        entry: *mut ListEntry,
    ) -> libc::c_int;
    fn list_remove_data(
        list: *mut *mut ListEntry,
        callback: ListEqualFunc,
        data: ListValue,
    ) -> libc::c_uint;
    fn list_sort(list: *mut *mut ListEntry, compare_func: ListCompareFunc);
    fn list_find_data(
        list: *mut ListEntry,
        callback: ListEqualFunc,
        data: ListValue,
    ) -> *mut ListEntry;
    fn list_iterate(list: *mut *mut ListEntry, iter: *mut ListIterator);
    fn list_iter_has_more(iterator: *mut ListIterator) -> libc::c_int;
    fn list_iter_next(iterator: *mut ListIterator) -> ListValue;
    fn list_iter_remove(iterator: *mut ListIterator);
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
pub type ListEntry = _ListEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ListIterator {
    pub prev_next: *mut *mut ListEntry,
    pub current: *mut ListEntry,
}
pub type ListIterator = _ListIterator;
pub type ListValue = *mut libc::c_void;
pub type ListCompareFunc = Option::<
    unsafe extern "C" fn(ListValue, ListValue) -> libc::c_int,
>;
pub type ListEqualFunc = Option::<
    unsafe extern "C" fn(ListValue, ListValue) -> libc::c_int,
>;
#[no_mangle]
pub static mut variable1: libc::c_int = 50 as libc::c_int;
#[no_mangle]
pub static mut variable2: libc::c_int = 0;
#[no_mangle]
pub static mut variable3: libc::c_int = 0;
#[no_mangle]
pub static mut variable4: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn generate_list() -> *mut ListEntry {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"ListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1893: {
        if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                39 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"ListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(list_append(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"ListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1839: {
        if !(list_append(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"ListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(list_append(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"ListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1785: {
        if !(list_append(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"ListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(list_append(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"ListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1725: {
        if !(list_append(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"ListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn check_list_integrity(mut list: *mut ListEntry) {
    let mut prev: *mut ListEntry = 0 as *mut ListEntry;
    let mut rover: *mut ListEntry = 0 as *mut ListEntry;
    prev = 0 as *mut ListEntry;
    rover = list;
    while !rover.is_null() {
        if list_prev(rover) == prev {} else {
            __assert_fail(
                b"list_prev(rover) == prev\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void check_list_integrity(ListEntry *)\0"))
                    .as_ptr(),
            );
        }
        'c_1974: {
            if list_prev(rover) == prev {} else {
                __assert_fail(
                    b"list_prev(rover) == prev\0" as *const u8 as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    56 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[libc::c_char; 39],
                    >(b"void check_list_integrity(ListEntry *)\0"))
                        .as_ptr(),
                );
            }
        };
        prev = rover;
        rover = list_next(rover);
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_list_append() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2494: {
        if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_append(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2435: {
        if !(list_append(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_append(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2376: {
        if !(list_append(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_append(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2317: {
        if !(list_append(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if list_length(list) == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 4\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2266: {
        if list_length(list) == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 4\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 0) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2216: {
        if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 0) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 1) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2166: {
        if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 1) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 2) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2116: {
        if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 2) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 3) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2063: {
        if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 3) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_prepend() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    if !(list_prepend(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_prepend(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2958: {
        if !(list_prepend(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_prepend(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_prepend(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2899: {
        if !(list_prepend(&mut list, &mut variable2 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                99 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_prepend(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_prepend(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2840: {
        if !(list_prepend(&mut list, &mut variable3 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if !(list_prepend(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_prepend(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2780: {
        if !(list_prepend(&mut list, &mut variable4 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 0) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2725: {
        if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 0) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 1) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2675: {
        if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 1) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 2) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2625: {
        if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 2) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 3) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2574: {
        if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 3) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_free() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    list_free(list);
    list_free(0 as *mut ListEntry);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_next() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut rover: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    rover = list;
    if list_data(rover) == &mut variable1 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(rover) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3271: {
        if list_data(rover) == &mut variable1 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(rover) == &variable1\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = list_next(rover);
    if list_data(rover) == &mut variable2 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(rover) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3218: {
        if list_data(rover) == &mut variable2 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(rover) == &variable2\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                147 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = list_next(rover);
    if list_data(rover) == &mut variable3 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(rover) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3165: {
        if list_data(rover) == &mut variable3 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(rover) == &variable3\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                149 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = list_next(rover);
    if list_data(rover) == &mut variable4 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(rover) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3111: {
        if list_data(rover) == &mut variable4 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(rover) == &variable4\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = list_next(rover);
    if rover.is_null() {} else {
        __assert_fail(
            b"rover == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3059: {
        if rover.is_null() {} else {
            __assert_fail(
                b"rover == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_nth_entry() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut entry: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    entry = list_nth_entry(list, 0 as libc::c_int as libc::c_uint);
    if list_data(entry) == &mut variable1 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(entry) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3619: {
        if list_data(entry) == &mut variable1 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(entry) == &variable1\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = list_nth_entry(list, 1 as libc::c_int as libc::c_uint);
    if list_data(entry) == &mut variable2 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(entry) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3564: {
        if list_data(entry) == &mut variable2 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(entry) == &variable2\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = list_nth_entry(list, 2 as libc::c_int as libc::c_uint);
    if list_data(entry) == &mut variable3 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(entry) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3509: {
        if list_data(entry) == &mut variable3 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(entry) == &variable3\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = list_nth_entry(list, 3 as libc::c_int as libc::c_uint);
    if list_data(entry) == &mut variable4 as *mut libc::c_int as ListValue {} else {
        __assert_fail(
            b"list_data(entry) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3454: {
        if list_data(entry) == &mut variable4 as *mut libc::c_int as ListValue {} else {
            __assert_fail(
                b"list_data(entry) == &variable4\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = list_nth_entry(list, 4 as libc::c_int as libc::c_uint);
    if entry.is_null() {} else {
        __assert_fail(
            b"entry == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3403: {
        if entry.is_null() {} else {
            __assert_fail(
                b"entry == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                179 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = list_nth_entry(list, 400 as libc::c_int as libc::c_uint);
    if entry.is_null() {} else {
        __assert_fail(
            b"entry == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3350: {
        if entry.is_null() {} else {
            __assert_fail(
                b"entry == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                181 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_nth_data() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 0) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3948: {
        if list_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 0) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                194 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 1) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3898: {
        if list_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 1) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 2) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3848: {
        if list_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 2) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                196 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as ListValue
    {} else {
        __assert_fail(
            b"list_nth_data(list, 3) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3798: {
        if list_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as ListValue
        {} else {
            __assert_fail(
                b"list_nth_data(list, 3) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (list_nth_data(list, 4 as libc::c_int as libc::c_uint)).is_null() {} else {
        __assert_fail(
            b"list_nth_data(list, 4) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3748: {
        if (list_nth_data(list, 4 as libc::c_int as libc::c_uint)).is_null() {} else {
            __assert_fail(
                b"list_nth_data(list, 4) == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                201 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (list_nth_data(list, 400 as libc::c_int as libc::c_uint)).is_null() {} else {
        __assert_fail(
            b"list_nth_data(list, 400) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3696: {
        if (list_nth_data(list, 400 as libc::c_int as libc::c_uint)).is_null() {} else {
            __assert_fail(
                b"list_nth_data(list, 400) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_length() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    if list_length(list) == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 4\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_4164: {
        if list_length(list) == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 4\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(list_prepend(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_prepend(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_4110: {
        if !(list_prepend(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == 5 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 5\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_4066: {
        if list_length(list) == 5 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 5\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                221 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
    if list_length(0 as *mut ListEntry) == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(NULL) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_list_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_4012: {
        if list_length(0 as *mut ListEntry) == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(NULL) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                227 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_list_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_list_remove_entry() {
    let mut empty_list: *mut ListEntry = 0 as *mut ListEntry;
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut entry: *mut ListEntry = 0 as *mut ListEntry;
    list = generate_list();
    entry = list_nth_entry(list, 2 as libc::c_int as libc::c_uint);
    if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
        __assert_fail(
            b"list_remove_entry(&list, entry) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4770: {
        if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
            __assert_fail(
                b"list_remove_entry(&list, entry) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                241 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            242 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4726: {
        if list_length(list) == 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 3\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    entry = list_nth_entry(list, 0 as libc::c_int as libc::c_uint);
    if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
        __assert_fail(
            b"list_remove_entry(&list, entry) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4666: {
        if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
            __assert_fail(
                b"list_remove_entry(&list, entry) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 2\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4622: {
        if list_length(list) == 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 2\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                249 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    if list_remove_entry(&mut list, 0 as *mut ListEntry) == 0 as libc::c_int {} else {
        __assert_fail(
            b"list_remove_entry(&list, NULL) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4567: {
        if list_remove_entry(&mut list, 0 as *mut ListEntry) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"list_remove_entry(&list, NULL) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                256 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_remove_entry(&mut empty_list, 0 as *mut ListEntry) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"list_remove_entry(&empty_list, NULL) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4511: {
        if list_remove_entry(&mut empty_list, 0 as *mut ListEntry) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"list_remove_entry(&empty_list, NULL) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                260 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
    list = 0 as *mut ListEntry;
    if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"list_append(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            267 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4446: {
        if !(list_append(&mut list, &mut variable1 as *mut libc::c_int as ListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !list.is_null() {} else {
        __assert_fail(
            b"list != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4404: {
        if !list.is_null() {} else {
            __assert_fail(
                b"list != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_remove_entry(&mut list, list) != 0 as libc::c_int {} else {
        __assert_fail(
            b"list_remove_entry(&list, list) != 0\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4357: {
        if list_remove_entry(&mut list, list) != 0 as libc::c_int {} else {
            __assert_fail(
                b"list_remove_entry(&list, list) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                269 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list.is_null() {} else {
        __assert_fail(
            b"list == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            270 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4314: {
        if list.is_null() {} else {
            __assert_fail(
                b"list == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                270 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list = generate_list();
    entry = list_nth_entry(list, 3 as libc::c_int as libc::c_uint);
    if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
        __assert_fail(
            b"list_remove_entry(&list, entry) != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            276 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_list_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4251: {
        if list_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
            __assert_fail(
                b"list_remove_entry(&list, entry) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                276 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_list_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_remove_data() {
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
    let mut num_entries: libc::c_uint = (::core::mem::size_of::<[libc::c_int; 13]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
    let mut val: libc::c_int = 0;
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut i: libc::c_uint = 0;
    list = 0 as *mut ListEntry;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        if !(list_prepend(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &entries[i]) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                294 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5351: {
            if !(list_prepend(
                &mut list,
                &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as ListValue,
            ))
                .is_null()
            {} else {
                __assert_fail(
                    b"list_prepend(&list, &entries[i]) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    294 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 33],
                        &[libc::c_char; 33],
                    >(b"void test_list_remove_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    val = 0 as libc::c_int;
    if list_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ) == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"list_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5289: {
        if list_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ) == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"list_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                300 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 56 as libc::c_int;
    if list_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ) == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"list_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5232: {
        if list_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ) == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"list_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    val = 8 as libc::c_int;
    if list_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ) == 1 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"list_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5170: {
        if list_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ) == 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"list_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == num_entries.wrapping_sub(1 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"list_length(list) == num_entries - 1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5120: {
        if list_length(list)
            == num_entries.wrapping_sub(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"list_length(list) == num_entries - 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    val = 4 as libc::c_int;
    if list_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"list_remove_data(&list, int_equal, &val) == 4\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5058: {
        if list_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ) == 4 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"list_remove_data(&list, int_equal, &val) == 4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                315 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == num_entries.wrapping_sub(5 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"list_length(list) == num_entries - 5\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            316 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5008: {
        if list_length(list)
            == num_entries.wrapping_sub(5 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"list_length(list) == num_entries - 5\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                316 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    val = 89 as libc::c_int;
    if list_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ) == 1 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"list_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            322 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4942: {
        if list_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ) == 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"list_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == num_entries.wrapping_sub(6 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"list_length(list) == num_entries - 6\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            323 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_list_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4868: {
        if list_length(list)
            == num_entries.wrapping_sub(6 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"list_length(list) == num_entries - 6\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                323 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_list_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    check_list_integrity(list);
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_sort() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
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
    list = 0 as *mut ListEntry;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        if !(list_prepend(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &entries[i]) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                340 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_sort(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5682: {
            if !(list_prepend(
                &mut list,
                &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as ListValue,
            ))
                .is_null()
            {} else {
                __assert_fail(
                    b"list_prepend(&list, &entries[i]) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    340 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void test_list_sort(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    list_sort(
        &mut list,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if list_length(list) == num_entries {} else {
        __assert_fail(
            b"list_length(list) == num_entries\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            347 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_5626: {
        if list_length(list) == num_entries {} else {
            __assert_fail(
                b"list_length(list) == num_entries\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                347 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
        value = list_nth_data(list, i) as *mut libc::c_int;
        if *value == sorted[i as usize] {} else {
            __assert_fail(
                b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                355 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_sort(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5515: {
            if *value == sorted[i as usize] {} else {
                __assert_fail(
                    b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    355 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"void test_list_sort(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    list_free(list);
    list = 0 as *mut ListEntry;
    list_sort(
        &mut list,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if list.is_null() {} else {
        __assert_fail(
            b"list == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            366 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_list_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_5446: {
        if list.is_null() {} else {
            __assert_fail(
                b"list == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_list_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_list_find_data() {
    let mut entries: [libc::c_int; 10] = [
        89 as libc::c_int,
        23 as libc::c_int,
        42 as libc::c_int,
        16 as libc::c_int,
        15 as libc::c_int,
        4 as libc::c_int,
        8 as libc::c_int,
        99 as libc::c_int,
        50 as libc::c_int,
        30 as libc::c_int,
    ];
    let mut num_entries: libc::c_int = (::core::mem::size_of::<[libc::c_int; 10]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut result: *mut ListEntry = 0 as *mut ListEntry;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    list = 0 as *mut ListEntry;
    i = 0 as libc::c_int;
    while i < num_entries {
        if !(list_append(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_append(&list, &entries[i]) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                383 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_find_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_6065: {
            if !(list_append(
                &mut list,
                &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as ListValue,
            ))
                .is_null()
            {} else {
                __assert_fail(
                    b"list_append(&list, &entries[i]) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    383 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_list_find_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < num_entries {
        val = entries[i as usize];
        result = list_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        );
        if !result.is_null() {} else {
            __assert_fail(
                b"result != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                394 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_find_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5967: {
            if !result.is_null() {} else {
                __assert_fail(
                    b"result != NULL\0" as *const u8 as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    394 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_list_find_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        data = list_data(result) as *mut libc::c_int;
        if *data == val {} else {
            __assert_fail(
                b"*data == val\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                397 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_find_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5915: {
            if *data == val {} else {
                __assert_fail(
                    b"*data == val\0" as *const u8 as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    397 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_list_find_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    val = 0 as libc::c_int;
    if (list_find_data(
        list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"list_find_data(list, int_equal, &val) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            403 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_find_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5845: {
        if (list_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_find_data(list, int_equal, &val) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                403 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_find_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 56 as libc::c_int;
    if (list_find_data(
        list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as ListValue,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"list_find_data(list, int_equal, &val) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_list_find_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5782: {
        if (list_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_find_data(list, int_equal, &val) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                405 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_list_find_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_to_array() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    list = generate_list();
    array = list_to_array(list);
    if *array.offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            419 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_6305: {
        if *array.offset(0 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[0] == &variable1\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_6259: {
        if *array.offset(1 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[1] == &variable2\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                420 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_6213: {
        if *array.offset(2 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[2] == &variable3\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                421 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            422 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_list_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_6167: {
        if *array.offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[3] == &variable4\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                422 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_list_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    free(array as *mut libc::c_void);
    list_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_list_iterate() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut iter: ListIterator = _ListIterator {
        prev_next: 0 as *mut *mut ListEntry,
        current: 0 as *mut ListEntry,
    };
    let mut i: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    list = 0 as *mut ListEntry;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        if !(list_prepend(&mut list, &mut a as *mut libc::c_int as ListValue)).is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &a) != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                450 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_iterate(void)\0"))
                    .as_ptr(),
            );
        }
        'c_6662: {
            if !(list_prepend(&mut list, &mut a as *mut libc::c_int as ListValue))
                .is_null()
            {} else {
                __assert_fail(
                    b"list_prepend(&list, &a) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    450 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 29],
                        &[libc::c_char; 29],
                    >(b"void test_list_iterate(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    counter = 0 as libc::c_int;
    list_iterate(&mut list, &mut iter);
    list_iter_remove(&mut iter);
    while list_iter_has_more(&mut iter) != 0 {
        data = list_iter_next(&mut iter) as *mut libc::c_int;
        counter += 1;
        counter;
        if counter % 2 as libc::c_int == 0 as libc::c_int {
            list_iter_remove(&mut iter);
            list_iter_remove(&mut iter);
        }
    }
    if (list_iter_next(&mut iter)).is_null() {} else {
        __assert_fail(
            b"list_iter_next(&iter) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            482 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_6557: {
        if (list_iter_next(&mut iter)).is_null() {} else {
            __assert_fail(
                b"list_iter_next(&iter) == NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                482 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_iter_remove(&mut iter);
    if counter == 50 as libc::c_int {} else {
        __assert_fail(
            b"counter == 50\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            488 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_6515: {
        if counter == 50 as libc::c_int {} else {
            __assert_fail(
                b"counter == 50\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                488 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if list_length(list) == 25 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"list_length(list) == 25\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            489 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_6470: {
        if list_length(list) == 25 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"list_length(list) == 25\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                489 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    list_free(list);
    list = 0 as *mut ListEntry;
    counter = 0 as libc::c_int;
    list_iterate(&mut list, &mut iter);
    while list_iter_has_more(&mut iter) != 0 {
        data = list_iter_next(&mut iter) as *mut libc::c_int;
        counter += 1;
        counter;
    }
    if counter == 0 as libc::c_int {} else {
        __assert_fail(
            b"counter == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-list.c\0" as *const u8 as *const libc::c_char,
            505 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_list_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_6388: {
        if counter == 0 as libc::c_int {} else {
            __assert_fail(
                b"counter == 0\0" as *const u8 as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                505 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_list_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_list_iterate_bad_remove() {
    let mut list: *mut ListEntry = 0 as *mut ListEntry;
    let mut iter: ListIterator = _ListIterator {
        prev_next: 0 as *mut *mut ListEntry,
        current: 0 as *mut ListEntry,
    };
    let mut values: [libc::c_int; 49] = [0; 49];
    let mut i: libc::c_int = 0;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    list = 0 as *mut ListEntry;
    i = 0 as libc::c_int;
    while i < 49 as libc::c_int {
        values[i as usize] = i;
        if !(list_prepend(
            &mut list,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int as ListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"list_prepend(&list, &values[i]) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                525 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 40],
                    &[libc::c_char; 40],
                >(b"void test_list_iterate_bad_remove(void)\0"))
                    .as_ptr(),
            );
        }
        'c_6853: {
            if !(list_prepend(
                &mut list,
                &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as ListValue,
            ))
                .is_null()
            {} else {
                __assert_fail(
                    b"list_prepend(&list, &values[i]) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    525 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void test_list_iterate_bad_remove(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    list_iterate(&mut list, &mut iter);
    while list_iter_has_more(&mut iter) != 0 {
        val = list_iter_next(&mut iter) as *mut libc::c_int;
        if *val % 2 as libc::c_int == 0 as libc::c_int {
            if list_remove_data(
                &mut list,
                Some(
                    int_equal
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                val as ListValue,
            ) != 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"list_remove_data(&list, int_equal, val) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                    542 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 40],
                        &[libc::c_char; 40],
                    >(b"void test_list_iterate_bad_remove(void)\0"))
                        .as_ptr(),
                );
            }
            'c_6764: {
                if list_remove_data(
                    &mut list,
                    Some(
                        int_equal
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    val as ListValue,
                ) != 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"list_remove_data(&list, int_equal, val) != 0\0" as *const u8
                            as *const libc::c_char,
                        b"test/test-list.c\0" as *const u8 as *const libc::c_char,
                        542 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 40],
                            &[libc::c_char; 40],
                        >(b"void test_list_iterate_bad_remove(void)\0"))
                            .as_ptr(),
                    );
                }
            };
            list_iter_remove(&mut iter);
        }
    }
    list_free(list);
}
static mut tests: [UnitTestFunction; 15] = unsafe {
    [
        Some(test_list_append as unsafe extern "C" fn() -> ()),
        Some(test_list_prepend as unsafe extern "C" fn() -> ()),
        Some(test_list_free as unsafe extern "C" fn() -> ()),
        Some(test_list_next as unsafe extern "C" fn() -> ()),
        Some(test_list_nth_entry as unsafe extern "C" fn() -> ()),
        Some(test_list_nth_data as unsafe extern "C" fn() -> ()),
        Some(test_list_length as unsafe extern "C" fn() -> ()),
        Some(test_list_remove_entry as unsafe extern "C" fn() -> ()),
        Some(test_list_remove_data as unsafe extern "C" fn() -> ()),
        Some(test_list_sort as unsafe extern "C" fn() -> ()),
        Some(test_list_find_data as unsafe extern "C" fn() -> ()),
        Some(test_list_to_array as unsafe extern "C" fn() -> ()),
        Some(test_list_iterate as unsafe extern "C" fn() -> ()),
        Some(test_list_iterate_bad_remove as unsafe extern "C" fn() -> ()),
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
