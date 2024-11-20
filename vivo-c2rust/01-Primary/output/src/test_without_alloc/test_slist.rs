#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _SListEntry;
    fn free(_: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn slist_free(list: *mut SListEntry);
    fn slist_prepend(list: *mut *mut SListEntry, data: SListValue) -> *mut SListEntry;
    fn slist_append(list: *mut *mut SListEntry, data: SListValue) -> *mut SListEntry;
    fn slist_next(listentry: *mut SListEntry) -> *mut SListEntry;
    fn slist_data(listentry: *mut SListEntry) -> SListValue;
    fn slist_nth_entry(list: *mut SListEntry, n: libc::c_uint) -> *mut SListEntry;
    fn slist_nth_data(list: *mut SListEntry, n: libc::c_uint) -> SListValue;
    fn slist_length(list: *mut SListEntry) -> libc::c_uint;
    fn slist_to_array(list: *mut SListEntry) -> *mut SListValue;
    fn slist_remove_entry(
        list: *mut *mut SListEntry,
        entry: *mut SListEntry,
    ) -> libc::c_int;
    fn slist_remove_data(
        list: *mut *mut SListEntry,
        callback: SListEqualFunc,
        data: SListValue,
    ) -> libc::c_uint;
    fn slist_sort(list: *mut *mut SListEntry, compare_func: SListCompareFunc);
    fn slist_find_data(
        list: *mut SListEntry,
        callback: SListEqualFunc,
        data: SListValue,
    ) -> *mut SListEntry;
    fn slist_iterate(list: *mut *mut SListEntry, iter: *mut SListIterator);
    fn slist_iter_has_more(iterator: *mut SListIterator) -> libc::c_int;
    fn slist_iter_next(iterator: *mut SListIterator) -> SListValue;
    fn slist_iter_remove(iterator: *mut SListIterator);
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type SListEntry = _SListEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SListIterator {
    pub prev_next: *mut *mut SListEntry,
    pub current: *mut SListEntry,
}
pub type SListIterator = _SListIterator;
pub type SListValue = *mut libc::c_void;
pub type SListCompareFunc = Option::<
    unsafe extern "C" fn(SListValue, SListValue) -> libc::c_int,
>;
pub type SListEqualFunc = Option::<
    unsafe extern "C" fn(SListValue, SListValue) -> libc::c_int,
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
pub unsafe extern "C" fn generate_list() -> *mut SListEntry {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    if !(slist_append(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"SListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1865: {
        if !(slist_append(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                11 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"SListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"SListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1811: {
        if !(slist_append(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                12 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"SListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            13 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"SListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1757: {
        if !(slist_append(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                13 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"SListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"SListEntry *generate_list(void)\0"))
                .as_ptr(),
        );
    }
    'c_1698: {
        if !(slist_append(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                14 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"SListEntry *generate_list(void)\0"))
                    .as_ptr(),
            );
        }
    };
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_append() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    if !(slist_append(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            22 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2347: {
        if !(slist_append(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                22 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            23 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2293: {
        if !(slist_append(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                23 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            24 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2239: {
        if !(slist_append(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                24 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_append(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_append(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2185: {
        if !(slist_append(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_append(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                25 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2139: {
        if slist_length(list) == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                26 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 0) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2089: {
        if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 0) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                28 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 1) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_2039: {
        if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 1) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                29 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 2) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_1989: {
        if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 2) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                30 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 3) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_append(void)\0"))
                .as_ptr(),
        );
    }
    'c_1931: {
        if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 3) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_append(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_prepend() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    if !(slist_prepend(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_prepend(&list, &variable1) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2781: {
        if !(slist_prepend(&mut list, &mut variable1 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_prepend(&list, &variable1) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_prepend(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_prepend(&list, &variable2) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2727: {
        if !(slist_prepend(&mut list, &mut variable2 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_prepend(&list, &variable2) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_prepend(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_prepend(&list, &variable3) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2673: {
        if !(slist_prepend(&mut list, &mut variable3 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_prepend(&list, &variable3) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                42 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(slist_prepend(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_prepend(&list, &variable4) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2618: {
        if !(slist_prepend(&mut list, &mut variable4 as *mut libc::c_int as SListValue))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_prepend(&list, &variable4) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 0) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2568: {
        if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 0) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 1) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2518: {
        if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 1) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                46 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 2) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2468: {
        if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 2) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                47 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 3) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_prepend(void)\0"))
                .as_ptr(),
        );
    }
    'c_2412: {
        if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 3) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                48 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_prepend(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_free() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    slist_free(list);
    slist_free(0 as *mut SListEntry);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_next() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut rover: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    rover = list;
    if slist_data(rover) == &mut variable1 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(rover) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3096: {
        if slist_data(rover) == &mut variable1 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(rover) == &variable1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = slist_next(rover);
    if slist_data(rover) == &mut variable2 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(rover) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_3043: {
        if slist_data(rover) == &mut variable2 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(rover) == &variable2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = slist_next(rover);
    if slist_data(rover) == &mut variable3 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(rover) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_2990: {
        if slist_data(rover) == &mut variable3 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(rover) == &variable3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = slist_next(rover);
    if slist_data(rover) == &mut variable4 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(rover) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_2936: {
        if slist_data(rover) == &mut variable4 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(rover) == &variable4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                81 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    rover = slist_next(rover);
    if rover.is_null() {} else {
        __assert_fail(
            b"rover == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_next(void)\0"))
                .as_ptr(),
        );
    }
    'c_2883: {
        if rover.is_null() {} else {
            __assert_fail(
                b"rover == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                83 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_next(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_nth_entry() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut entry: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    entry = slist_nth_entry(list, 0 as libc::c_int as libc::c_uint);
    if slist_data(entry) == &mut variable1 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(entry) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3444: {
        if slist_data(entry) == &mut variable1 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(entry) == &variable1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 1 as libc::c_int as libc::c_uint);
    if slist_data(entry) == &mut variable2 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(entry) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3389: {
        if slist_data(entry) == &mut variable2 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(entry) == &variable2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                99 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 2 as libc::c_int as libc::c_uint);
    if slist_data(entry) == &mut variable3 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(entry) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3334: {
        if slist_data(entry) == &mut variable3 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(entry) == &variable3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 3 as libc::c_int as libc::c_uint);
    if slist_data(entry) == &mut variable4 as *mut libc::c_int as SListValue {} else {
        __assert_fail(
            b"slist_data(entry) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3279: {
        if slist_data(entry) == &mut variable4 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_data(entry) == &variable4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                103 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 4 as libc::c_int as libc::c_uint);
    if entry.is_null() {} else {
        __assert_fail(
            b"entry == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3228: {
        if entry.is_null() {} else {
            __assert_fail(
                b"entry == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 400 as libc::c_int as libc::c_uint);
    if entry.is_null() {} else {
        __assert_fail(
            b"entry == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_nth_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_3175: {
        if entry.is_null() {} else {
            __assert_fail(
                b"entry == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_nth_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_nth_data() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
        == &mut variable1 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 0) == &variable1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3773: {
        if slist_nth_data(list, 0 as libc::c_int as libc::c_uint)
            == &mut variable1 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 0) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
        == &mut variable2 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 1) == &variable2\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3723: {
        if slist_nth_data(list, 1 as libc::c_int as libc::c_uint)
            == &mut variable2 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 1) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                123 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
        == &mut variable3 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 2) == &variable3\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3673: {
        if slist_nth_data(list, 2 as libc::c_int as libc::c_uint)
            == &mut variable3 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 2) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                124 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
        == &mut variable4 as *mut libc::c_int as SListValue
    {} else {
        __assert_fail(
            b"slist_nth_data(list, 3) == &variable4\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3623: {
        if slist_nth_data(list, 3 as libc::c_int as libc::c_uint)
            == &mut variable4 as *mut libc::c_int as SListValue
        {} else {
            __assert_fail(
                b"slist_nth_data(list, 3) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (slist_nth_data(list, 4 as libc::c_int as libc::c_uint)).is_null() {} else {
        __assert_fail(
            b"slist_nth_data(list, 4) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            129 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3573: {
        if (slist_nth_data(list, 4 as libc::c_int as libc::c_uint)).is_null() {} else {
            __assert_fail(
                b"slist_nth_data(list, 4) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                129 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (slist_nth_data(list, 400 as libc::c_int as libc::c_uint)).is_null() {} else {
        __assert_fail(
            b"slist_nth_data(list, 400) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_nth_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_3521: {
        if (slist_nth_data(list, 400 as libc::c_int as libc::c_uint)).is_null() {} else {
            __assert_fail(
                b"slist_nth_data(list, 400) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                130 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_nth_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_length() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    if slist_length(list) == 4 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_3943: {
        if slist_length(list) == 4 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_prepend(&mut list, &mut variable1 as *mut libc::c_int as SListValue);
    if slist_length(list) == 5 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 5\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_3891: {
        if slist_length(list) == 5 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 5\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(0 as *mut SListEntry) == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(NULL) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_slist_length(void)\0"))
                .as_ptr(),
        );
    }
    'c_3843: {
        if slist_length(0 as *mut SListEntry) == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_length(NULL) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_slist_length(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_remove_entry() {
    let mut empty_list: *mut SListEntry = 0 as *mut SListEntry;
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut entry: *mut SListEntry = 0 as *mut SListEntry;
    list = generate_list();
    entry = slist_nth_entry(list, 2 as libc::c_int as libc::c_uint);
    if slist_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
        __assert_fail(
            b"slist_remove_entry(&list, entry) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4313: {
        if slist_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
            __assert_fail(
                b"slist_remove_entry(&list, entry) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4269: {
        if slist_length(list) == 3 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entry = slist_nth_entry(list, 0 as libc::c_int as libc::c_uint);
    if slist_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
        __assert_fail(
            b"slist_remove_entry(&list, entry) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4214: {
        if slist_remove_entry(&mut list, entry) != 0 as libc::c_int {} else {
            __assert_fail(
                b"slist_remove_entry(&list, entry) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == 2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4170: {
        if slist_length(list) == 2 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_remove_entry(&mut list, entry) == 0 as libc::c_int {} else {
        __assert_fail(
            b"slist_remove_entry(&list, entry) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4123: {
        if slist_remove_entry(&mut list, entry) == 0 as libc::c_int {} else {
            __assert_fail(
                b"slist_remove_entry(&list, entry) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                180 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_remove_entry(&mut list, 0 as *mut SListEntry) == 0 as libc::c_int {} else {
        __assert_fail(
            b"slist_remove_entry(&list, NULL) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4072: {
        if slist_remove_entry(&mut list, 0 as *mut SListEntry) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"slist_remove_entry(&list, NULL) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                184 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_remove_entry(&mut empty_list, 0 as *mut SListEntry) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"slist_remove_entry(&empty_list, NULL) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_slist_remove_entry(void)\0"))
                .as_ptr(),
        );
    }
    'c_4014: {
        if slist_remove_entry(&mut empty_list, 0 as *mut SListEntry) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"slist_remove_entry(&empty_list, NULL) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                188 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_slist_remove_entry(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_remove_data() {
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
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut i: libc::c_uint = 0;
    list = 0 as *mut SListEntry;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        slist_prepend(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SListValue,
        );
        i = i.wrapping_add(1);
        i;
    }
    val = 0 as libc::c_int;
    if slist_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ) == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"slist_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4808: {
        if slist_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ) == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                211 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 56 as libc::c_int;
    if slist_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ) == 0 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"slist_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4751: {
        if slist_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ) == 0 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_remove_data(&list, int_equal, &val) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                213 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 8 as libc::c_int;
    if slist_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ) == 1 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"slist_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            218 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4694: {
        if slist_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ) == 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                218 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == num_entries.wrapping_sub(1 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"slist_length(list) == num_entries - 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4644: {
        if slist_length(list)
            == num_entries.wrapping_sub(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"slist_length(list) == num_entries - 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 4 as libc::c_int;
    if slist_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ) == 4 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"slist_remove_data(&list, int_equal, &val) == 4\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4587: {
        if slist_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ) == 4 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_remove_data(&list, int_equal, &val) == 4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                224 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == num_entries.wrapping_sub(5 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"slist_length(list) == num_entries - 5\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4537: {
        if slist_length(list)
            == num_entries.wrapping_sub(5 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"slist_length(list) == num_entries - 5\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 89 as libc::c_int;
    if slist_remove_data(
        &mut list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ) == 1 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"slist_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4476: {
        if slist_remove_data(
            &mut list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ) == 1 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"slist_remove_data(&list, int_equal, &val) == 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == num_entries.wrapping_sub(6 as libc::c_int as libc::c_uint)
    {} else {
        __assert_fail(
            b"slist_length(list) == num_entries - 6\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_slist_remove_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_4402: {
        if slist_length(list)
            == num_entries.wrapping_sub(6 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"slist_length(list) == num_entries - 6\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_slist_remove_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_sort() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
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
    list = 0 as *mut SListEntry;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        slist_prepend(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SListValue,
        );
        i = i.wrapping_add(1);
        i;
    }
    slist_sort(
        &mut list,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if slist_length(list) == num_entries {} else {
        __assert_fail(
            b"slist_length(list) == num_entries\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_5095: {
        if slist_length(list) == num_entries {} else {
            __assert_fail(
                b"slist_length(list) == num_entries\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                253 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
        value = slist_nth_data(list, i) as *mut libc::c_int;
        if *value == sorted[i as usize] {} else {
            __assert_fail(
                b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_sort(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4984: {
            if *value == sorted[i as usize] {} else {
                __assert_fail(
                    b"*value == sorted[i]\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-slist.c\0" as *const u8
                        as *const libc::c_char,
                    261 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 27],
                        &[libc::c_char; 27],
                    >(b"void test_slist_sort(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i = i.wrapping_add(1);
        i;
    }
    slist_free(list);
    list = 0 as *mut SListEntry;
    slist_sort(
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
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_slist_sort(void)\0"))
                .as_ptr(),
        );
    }
    'c_4914: {
        if list.is_null() {} else {
            __assert_fail(
                b"list == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                272 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_slist_sort(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_find_data() {
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
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut result: *mut SListEntry = 0 as *mut SListEntry;
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    list = 0 as *mut SListEntry;
    i = 0 as libc::c_int;
    while i < num_entries {
        slist_append(
            &mut list,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SListValue,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < num_entries {
        val = entries[i as usize];
        result = slist_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        );
        if !result.is_null() {} else {
            __assert_fail(
                b"result != NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_find_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5386: {
            if !result.is_null() {} else {
                __assert_fail(
                    b"result != NULL\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-slist.c\0" as *const u8
                        as *const libc::c_char,
                    299 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_slist_find_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        data = slist_data(result) as *mut libc::c_int;
        if *data == val {} else {
            __assert_fail(
                b"*data == val\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_find_data(void)\0"))
                    .as_ptr(),
            );
        }
        'c_5334: {
            if *data == val {} else {
                __assert_fail(
                    b"*data == val\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-slist.c\0" as *const u8
                        as *const libc::c_char,
                    302 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_slist_find_data(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    val = 0 as libc::c_int;
    if (slist_find_data(
        list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_find_data(list, int_equal, &val) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_find_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5264: {
        if (slist_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_find_data(list, int_equal, &val) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_find_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    val = 56 as libc::c_int;
    if (slist_find_data(
        list,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut val as *mut libc::c_int as SListValue,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"slist_find_data(list, int_equal, &val) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            310 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_slist_find_data(void)\0"))
                .as_ptr(),
        );
    }
    'c_5201: {
        if (slist_find_data(
            list,
            Some(
                int_equal
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            &mut val as *mut libc::c_int as SListValue,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"slist_find_data(list, int_equal, &val) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                310 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_slist_find_data(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_to_array() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut array: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    list = generate_list();
    array = slist_to_array(list);
    if *array.offset(0 as libc::c_int as isize)
        == &mut variable1 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[0] == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            323 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_5668: {
        if *array.offset(0 as libc::c_int as isize)
            == &mut variable1 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[0] == &variable1\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                323 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(1 as libc::c_int as isize)
        == &mut variable2 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[1] == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            324 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_5622: {
        if *array.offset(1 as libc::c_int as isize)
            == &mut variable2 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[1] == &variable2\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                324 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(2 as libc::c_int as isize)
        == &mut variable3 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[2] == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            325 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_5576: {
        if *array.offset(2 as libc::c_int as isize)
            == &mut variable3 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[2] == &variable3\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if *array.offset(3 as libc::c_int as isize)
        == &mut variable4 as *mut libc::c_int as *mut libc::c_void
    {} else {
        __assert_fail(
            b"array[3] == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            326 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_slist_to_array(void)\0"))
                .as_ptr(),
        );
    }
    'c_5529: {
        if *array.offset(3 as libc::c_int as isize)
            == &mut variable4 as *mut libc::c_int as *mut libc::c_void
        {} else {
            __assert_fail(
                b"array[3] == &variable4\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                326 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_slist_to_array(void)\0"))
                    .as_ptr(),
            );
        }
    };
    free(array as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_iterate() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut iter: SListIterator = _SListIterator {
        prev_next: 0 as *mut *mut SListEntry,
        current: 0 as *mut SListEntry,
    };
    let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut a: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut counter: libc::c_int = 0;
    list = 0 as *mut SListEntry;
    i = 0 as libc::c_int;
    while i < 50 as libc::c_int {
        slist_prepend(&mut list, &mut a as *mut libc::c_int as SListValue);
        i += 1;
        i;
    }
    counter = 0 as libc::c_int;
    slist_iterate(&mut list, &mut iter);
    slist_iter_remove(&mut iter);
    while slist_iter_has_more(&mut iter) != 0 {
        data = slist_iter_next(&mut iter) as *mut libc::c_int;
        counter += 1;
        counter;
        if counter % 2 as libc::c_int == 0 as libc::c_int {
            slist_iter_remove(&mut iter);
            slist_iter_remove(&mut iter);
        }
    }
    if (slist_iter_next(&mut iter)).is_null() {} else {
        __assert_fail(
            b"slist_iter_next(&iter) == SLIST_NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            381 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_5936: {
        if (slist_iter_next(&mut iter)).is_null() {} else {
            __assert_fail(
                b"slist_iter_next(&iter) == SLIST_NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                381 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_iter_remove(&mut iter);
    if counter == 50 as libc::c_int {} else {
        __assert_fail(
            b"counter == 50\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_5895: {
        if counter == 50 as libc::c_int {} else {
            __assert_fail(
                b"counter == 50\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if slist_length(list) == 25 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"slist_length(list) == 25\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            388 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_5850: {
        if slist_length(list) == 25 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"slist_length(list) == 25\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                388 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
    slist_free(list);
    list = 0 as *mut SListEntry;
    counter = 0 as libc::c_int;
    slist_iterate(&mut list, &mut iter);
    while slist_iter_has_more(&mut iter) != 0 {
        data = slist_iter_next(&mut iter) as *mut libc::c_int;
        counter += 1;
        counter;
        if counter % 2 as libc::c_int == 0 as libc::c_int {
            slist_iter_remove(&mut iter);
        }
    }
    if counter == 0 as libc::c_int {} else {
        __assert_fail(
            b"counter == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
            412 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_slist_iterate(void)\0"))
                .as_ptr(),
        );
    }
    'c_5752: {
        if counter == 0 as libc::c_int {} else {
            __assert_fail(
                b"counter == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-slist.c\0" as *const u8 as *const libc::c_char,
                412 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_slist_iterate(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_slist_iterate_bad_remove() {
    let mut list: *mut SListEntry = 0 as *mut SListEntry;
    let mut iter: SListIterator = _SListIterator {
        prev_next: 0 as *mut *mut SListEntry,
        current: 0 as *mut SListEntry,
    };
    let mut values: [libc::c_int; 49] = [0; 49];
    let mut i: libc::c_int = 0;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    list = 0 as *mut SListEntry;
    i = 0 as libc::c_int;
    while i < 49 as libc::c_int {
        values[i as usize] = i;
        slist_prepend(
            &mut list,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SListValue,
        );
        i += 1;
        i;
    }
    slist_iterate(&mut list, &mut iter);
    while slist_iter_has_more(&mut iter) != 0 {
        val = slist_iter_next(&mut iter) as *mut libc::c_int;
        if *val % 2 as libc::c_int == 0 as libc::c_int {
            if slist_remove_data(
                &mut list,
                Some(
                    int_equal
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> libc::c_int,
                ),
                val as SListValue,
            ) != 0 as libc::c_int as libc::c_uint
            {} else {
                __assert_fail(
                    b"slist_remove_data(&list, int_equal, val) != 0\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-slist.c\0" as *const u8
                        as *const libc::c_char,
                    445 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"void test_slist_iterate_bad_remove(void)\0"))
                        .as_ptr(),
                );
            }
            'c_6096: {
                if slist_remove_data(
                    &mut list,
                    Some(
                        int_equal
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> libc::c_int,
                    ),
                    val as SListValue,
                ) != 0 as libc::c_int as libc::c_uint
                {} else {
                    __assert_fail(
                        b"slist_remove_data(&list, int_equal, val) != 0\0" as *const u8
                            as *const libc::c_char,
                        b"test-without-alloc/test-slist.c\0" as *const u8
                            as *const libc::c_char,
                        445 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 41],
                            &[libc::c_char; 41],
                        >(b"void test_slist_iterate_bad_remove(void)\0"))
                            .as_ptr(),
                    );
                }
            };
            slist_iter_remove(&mut iter);
        }
    }
    slist_free(list);
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
