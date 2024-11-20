#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern crate libc;
use primary::set::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    pub type _Set;
    pub type _SetEntry;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn alloc_test_strdup(string: *const libc::c_char) -> *mut libc::c_char;
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn set_new(hash_func: SetHashFunc, equal_func: SetEqualFunc) -> *mut Set;
    fn set_free(set: *mut Set);
    fn set_register_free_function(set: *mut Set, free_func: SetFreeFunc);
    fn set_insert(set: *mut Set, data: SetValue) -> libc::c_int;
    fn set_remove(set: *mut Set, data: SetValue) -> libc::c_int;
    fn set_query(set: *mut Set, data: SetValue) -> libc::c_int;
    fn set_num_entries(set: *mut Set) -> libc::c_uint;
    fn set_to_array(set: *mut Set) -> *mut SetValue;
    fn set_union(set1: *mut Set, set2: *mut Set) -> *mut Set;
    fn set_intersection(set1: *mut Set, set2: *mut Set) -> *mut Set;
    fn set_iterate(set: *mut Set, iter: *mut SetIterator);
    fn set_iter_has_more(iterator: *mut SetIterator) -> libc::c_int;
    fn set_iter_next(iterator: *mut SetIterator) -> SetValue;
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_hash(location: *mut libc::c_void) -> libc::c_uint;
    fn pointer_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn pointer_hash(location: *mut libc::c_void) -> libc::c_uint;
    fn string_equal(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_hash(string: *mut libc::c_void) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type Set = _Set;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SetIterator {
    pub set: *mut Set,
    pub next_entry: *mut SetEntry,
    pub next_chain: libc::c_uint,
}
pub type SetEntry = _SetEntry;
pub type SetIterator = _SetIterator;
pub type SetValue = *mut libc::c_void;
pub type SetHashFunc = Option::<unsafe extern "C" fn(SetValue) -> libc::c_uint>;
pub type SetEqualFunc = Option::<
    unsafe extern "C" fn(SetValue, SetValue) -> libc::c_int,
>;
pub type SetFreeFunc = Option::<unsafe extern "C" fn(SetValue) -> ()>;
#[no_mangle]
pub static mut allocated_values: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn generate_set() -> *mut Set {
    let mut set: *mut Set = 0 as *mut Set;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_uint = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    set = set_new(
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            string_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 10000 as libc::c_int as libc::c_uint {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        value = alloc_test_strdup(buf.as_mut_ptr());
        set_insert(set, value as SetValue);
        if set_num_entries(set) == i.wrapping_add(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"set_num_entries(set) == i + 1\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                57 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 24],
                    &[libc::c_char; 24],
                >(b"Set *generate_set(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    set_register_free_function(
        set,
        Some(alloc_test_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    return set;
}
#[no_mangle]
pub unsafe extern "C" fn test_set_new_free() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    set = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    set_register_free_function(
        set,
        Some(alloc_test_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    if !set.is_null() {} else {
        __assert_fail(
            b"set != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_set_new_free(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        value = alloc_test_malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as *mut libc::c_int;
        *value = i;
        set_insert(set, value as SetValue);
        i += 1;
        i;
    }
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_insert() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut numbers1: [libc::c_int; 6] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
    ];
    let mut numbers2: [libc::c_int; 6] = [
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    set = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        set_insert(
            set,
            &mut *numbers1.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        set_insert(
            set,
            &mut *numbers2.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    if set_num_entries(set) == 10 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"set_num_entries(set) == 10\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_set_insert(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_query() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_int = 0;
    set = generate_set();
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if set_query(set, buf.as_mut_ptr() as SetValue) != 0 as libc::c_int {} else {
            __assert_fail(
                b"set_query(set, buf) != 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_set_query(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    if set_query(set, b"-1\0" as *const u8 as *const libc::c_char as SetValue)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"set_query(set, \"-1\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_set_query(void)\0"))
                .as_ptr(),
        );
    };
    if set_query(set, b"100001\0" as *const u8 as *const libc::c_char as SetValue)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"set_query(set, \"100001\") == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_set_query(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_remove() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_int = 0;
    let mut num_entries: libc::c_uint = 0;
    set = generate_set();
    num_entries = set_num_entries(set);
    if num_entries == 10000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"num_entries == 10000\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void test_set_remove(void)\0"))
                .as_ptr(),
        );
    };
    i = 4000 as libc::c_int;
    while i < 6000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if set_query(set, buf.as_mut_ptr() as SetValue) != 0 as libc::c_int {} else {
            __assert_fail(
                b"set_query(set, buf) != 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                170 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        if set_remove(set, buf.as_mut_ptr() as SetValue) != 0 as libc::c_int {} else {
            __assert_fail(
                b"set_remove(set, buf) != 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        if set_num_entries(set)
            == num_entries.wrapping_sub(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"set_num_entries(set) == num_entries - 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                178 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        if set_query(set, buf.as_mut_ptr() as SetValue) == 0 as libc::c_int {} else {
            __assert_fail(
                b"set_query(set, buf) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                182 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        num_entries = num_entries.wrapping_sub(1);
        num_entries;
        i += 1;
        i;
    }
    i = -(1000 as libc::c_int);
    while i < -(500 as libc::c_int) {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if set_remove(set, buf.as_mut_ptr() as SetValue) == 0 as libc::c_int {} else {
            __assert_fail(
                b"set_remove(set, buf) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                192 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        if set_num_entries(set) == num_entries {} else {
            __assert_fail(
                b"set_num_entries(set) == num_entries\0" as *const u8
                    as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = 50000 as libc::c_int;
    while i < 51000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if set_remove(set, buf.as_mut_ptr() as SetValue) == 0 as libc::c_int {} else {
            __assert_fail(
                b"set_remove(set, buf) == 0\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                199 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        if set_num_entries(set) == num_entries {} else {
            __assert_fail(
                b"set_num_entries(set) == num_entries\0" as *const u8
                    as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                200 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 27],
                    &[libc::c_char; 27],
                >(b"void test_set_remove(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_union() {
    let mut numbers1: [libc::c_int; 7] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
    ];
    let mut numbers2: [libc::c_int; 7] = [
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let mut result: [libc::c_int; 11] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut set1: *mut Set = 0 as *mut Set;
    let mut set2: *mut Set = 0 as *mut Set;
    let mut result_set: *mut Set = 0 as *mut Set;
    let mut allocated: size_t = 0;
    set1 = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        set_insert(
            set1,
            &mut *numbers1.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    set2 = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        set_insert(
            set2,
            &mut *numbers2.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    result_set = set_union(set1, set2);
    if set_num_entries(result_set) == 11 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"set_num_entries(result_set) == 11\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 26],
                &[libc::c_char; 26],
            >(b"void test_set_union(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        if set_query(
            result_set,
            &mut *result.as_mut_ptr().offset(i as isize) as *mut libc::c_int as SetValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"set_query(result_set, &result[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"void test_set_union(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    set_free(result_set);
    set_free(set1);
    set_free(set2);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_intersection() {
    let mut numbers1: [libc::c_int; 7] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        4 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
    ];
    let mut numbers2: [libc::c_int; 7] = [
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
    ];
    let mut result: [libc::c_int; 3] = [
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut set1: *mut Set = 0 as *mut Set;
    let mut set2: *mut Set = 0 as *mut Set;
    let mut result_set: *mut Set = 0 as *mut Set;
    let mut allocated: size_t = 0;
    set1 = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        set_insert(
            set1,
            &mut *numbers1.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    set2 = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        set_insert(
            set2,
            &mut *numbers2.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as SetValue,
        );
        i += 1;
        i;
    }
    result_set = set_intersection(set1, set2);
    if set_num_entries(result_set) == 3 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"set_num_entries(result_set) == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_set_intersection(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if set_query(
            result_set,
            &mut *result.as_mut_ptr().offset(i as isize) as *mut libc::c_int as SetValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"set_query(result_set, &result[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                303 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_set_intersection(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    set_free(set1);
    set_free(set2);
    set_free(result_set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_to_array() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut values: [libc::c_int; 100] = [0; 100];
    let mut array: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0;
    set = set_new(
        Some(pointer_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            pointer_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        values[i as usize] = 1 as libc::c_int;
        set_insert(
            set,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int as SetValue,
        );
        i += 1;
        i;
    }
    array = set_to_array(set) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < 100 as libc::c_int {
        if **array.offset(i as isize) == 1 as libc::c_int {} else {
            __assert_fail(
                b"*array[i] == 1\0" as *const u8 as *const libc::c_char,
                b"test/test-set.c\0" as *const u8 as *const libc::c_char,
                345 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_set_to_array(void)\0"))
                    .as_ptr(),
            );
        };
        **array.offset(i as isize) = 0 as libc::c_int;
        i += 1;
        i;
    }
    alloc_test_free(array as *mut libc::c_void);
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_iterating() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut iterator: SetIterator = _SetIterator {
        set: 0 as *mut Set,
        next_entry: 0 as *mut SetEntry,
        next_chain: 0,
    };
    let mut count: libc::c_int = 0;
    set = generate_set();
    count = 0 as libc::c_int;
    set_iterate(set, &mut iterator);
    while set_iter_has_more(&mut iterator) != 0 {
        set_iter_next(&mut iterator);
        count += 1;
        count;
    }
    if (set_iter_next(&mut iterator)).is_null() {} else {
        __assert_fail(
            b"set_iter_next(&iterator) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            380 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_set_iterating(void)\0"))
                .as_ptr(),
        );
    };
    if count == 10000 as libc::c_int {} else {
        __assert_fail(
            b"count == 10000\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_set_iterating(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
    set = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    set_iterate(set, &mut iterator);
    if set_iter_has_more(&mut iterator) == 0 as libc::c_int {} else {
        __assert_fail(
            b"set_iter_has_more(&iterator) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            394 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_set_iterating(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn test_set_iterating_remove() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut iterator: SetIterator = _SetIterator {
        set: 0 as *mut Set,
        next_entry: 0 as *mut SetEntry,
        next_chain: 0,
    };
    let mut count: libc::c_int = 0;
    let mut removed: libc::c_uint = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    set = generate_set();
    count = 0 as libc::c_int;
    removed = 0 as libc::c_int as libc::c_uint;
    set_iterate(set, &mut iterator);
    while set_iter_has_more(&mut iterator) != 0 {
        value = set_iter_next(&mut iterator) as *mut libc::c_char;
        if atoi(value) % 100 as libc::c_int == 0 as libc::c_int {
            set_remove(set, value as SetValue);
            removed = removed.wrapping_add(1);
            removed;
        }
        count += 1;
        count;
    }
    if count == 10000 as libc::c_int {} else {
        __assert_fail(
            b"count == 10000\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_set_iterating_remove(void)\0"))
                .as_ptr(),
        );
    };
    if removed == 100 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"removed == 100\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_set_iterating_remove(void)\0"))
                .as_ptr(),
        );
    };
    if set_num_entries(set)
        == (10000 as libc::c_int as libc::c_uint).wrapping_sub(removed)
    {} else {
        __assert_fail(
            b"set_num_entries(set) == 10000 - removed\0" as *const u8
                as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_set_iterating_remove(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
}
#[no_mangle]
pub unsafe extern "C" fn new_value(mut value: libc::c_int) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = 0 as *mut libc::c_int;
    result = alloc_test_malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    *result = value;
    allocated_values += 1;
    allocated_values;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn free_value(mut value: *mut libc::c_void) {
    alloc_test_free(value);
    allocated_values -= 1;
    allocated_values;
}
#[no_mangle]
pub unsafe extern "C" fn test_set_free_function() {
    let mut set: *mut Set = 0 as *mut Set;
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    set = set_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    set_register_free_function(
        set,
        Some(free_value as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    allocated_values = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        value = new_value(i);
        set_insert(set, value as SetValue);
        i += 1;
        i;
    }
    if allocated_values == 1000 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == 1000\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            484 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_set_free_function(void)\0"))
                .as_ptr(),
        );
    };
    i = 500 as libc::c_int;
    set_remove(set, &mut i as *mut libc::c_int as SetValue);
    if allocated_values == 999 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == 999\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            491 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_set_free_function(void)\0"))
                .as_ptr(),
        );
    };
    set_free(set);
    if allocated_values == 0 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-set.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_set_free_function(void)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_set_out_of_memory() {}
static mut tests: [UnitTestFunction; 12] = unsafe {
    [
        Some(test_set_new_free as unsafe extern "C" fn() -> ()),
        Some(test_set_insert as unsafe extern "C" fn() -> ()),
        Some(test_set_query as unsafe extern "C" fn() -> ()),
        Some(test_set_remove as unsafe extern "C" fn() -> ()),
        Some(test_set_intersection as unsafe extern "C" fn() -> ()),
        Some(test_set_union as unsafe extern "C" fn() -> ()),
        Some(test_set_iterating as unsafe extern "C" fn() -> ()),
        Some(test_set_iterating_remove as unsafe extern "C" fn() -> ()),
        Some(test_set_to_array as unsafe extern "C" fn() -> ()),
        Some(test_set_free_function as unsafe extern "C" fn() -> ()),
        Some(test_set_out_of_memory as unsafe extern "C" fn() -> ()),
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




















