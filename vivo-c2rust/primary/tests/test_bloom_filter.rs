#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern crate libc;
use primary::bloom_filter::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    pub type _BloomFilter;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn bloom_filter_new(
        table_size: libc::c_uint,
        hash_func: BloomFilterHashFunc,
        num_functions: libc::c_uint,
    ) -> *mut BloomFilter;
    fn bloom_filter_free(bloomfilter: *mut BloomFilter);
    fn bloom_filter_insert(bloomfilter: *mut BloomFilter, value: BloomFilterValue);
    fn bloom_filter_query(
        bloomfilter: *mut BloomFilter,
        value: BloomFilterValue,
    ) -> libc::c_int;
    fn bloom_filter_read(bloomfilter: *mut BloomFilter, array: *mut libc::c_uchar);
    fn bloom_filter_load(bloomfilter: *mut BloomFilter, array: *mut libc::c_uchar);
    fn bloom_filter_union(
        filter1: *mut BloomFilter,
        filter2: *mut BloomFilter,
    ) -> *mut BloomFilter;
    fn bloom_filter_intersection(
        filter1: *mut BloomFilter,
        filter2: *mut BloomFilter,
    ) -> *mut BloomFilter;
    fn string_hash(string: *mut libc::c_void) -> libc::c_uint;
    fn string_nocase_hash(string: *mut libc::c_void) -> libc::c_uint;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type BloomFilter = _BloomFilter;
pub type BloomFilterValue = *mut libc::c_void;
pub type BloomFilterHashFunc = Option::<
    unsafe extern "C" fn(BloomFilterValue) -> libc::c_uint,
>;
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_new_free() {
    let mut filter: *mut BloomFilter = 0 as *mut BloomFilter;
    filter = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        1 as libc::c_int as libc::c_uint,
    );
    if !filter.is_null() {} else {
        __assert_fail(
            b"filter != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_new_free(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter);
    filter = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        64 as libc::c_int as libc::c_uint,
    );
    if !filter.is_null() {} else {
        __assert_fail(
            b"filter != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_new_free(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter);
    filter = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        50000 as libc::c_int as libc::c_uint,
    );
    if filter.is_null() {} else {
        __assert_fail(
            b"filter == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_new_free(void)\0"))
                .as_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_insert_query() {
    let mut filter: *mut BloomFilter = 0 as *mut BloomFilter;
    filter = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    if bloom_filter_query(
        filter,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter, \"test 1\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_insert_query(void)\0"))
                .as_ptr(),
        );
    };
    if bloom_filter_query(
        filter,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter, \"test 2\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_insert_query(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_insert(
        filter,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    bloom_filter_insert(
        filter,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    if bloom_filter_query(
        filter,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter, \"test 1\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_insert_query(void)\0"))
                .as_ptr(),
        );
    };
    if bloom_filter_query(
        filter,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter, \"test 2\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_insert_query(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter);
}
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_read_load() {
    let mut filter1: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut filter2: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut state: [libc::c_uchar; 16] = [0; 16];
    filter1 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_insert(
        filter1,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    bloom_filter_insert(
        filter1,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    bloom_filter_read(filter1, state.as_mut_ptr());
    bloom_filter_free(filter1);
    filter2 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_load(filter2, state.as_mut_ptr());
    if bloom_filter_query(
        filter2,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter2, \"test 1\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_bloom_filter_read_load(void)\0"))
                .as_ptr(),
        );
    };
    if bloom_filter_query(
        filter2,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter2, \"test 2\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_bloom_filter_read_load(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter2);
}
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_intersection() {
    let mut filter1: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut filter2: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut result: *mut BloomFilter = 0 as *mut BloomFilter;
    filter1 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_insert(
        filter1,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    bloom_filter_insert(
        filter1,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    filter2 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_insert(
        filter2,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    if bloom_filter_query(
        filter2,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(filter2, \"test 2\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_intersection(void)\0"))
                .as_ptr(),
        );
    };
    result = bloom_filter_intersection(filter1, filter2);
    if bloom_filter_query(
        result,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(result, \"test 1\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_intersection(void)\0"))
                .as_ptr(),
        );
    };
    if bloom_filter_query(
        result,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(result, \"test 2\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_bloom_filter_intersection(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(result);
}
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_union() {
    let mut filter1: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut filter2: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut result: *mut BloomFilter = 0 as *mut BloomFilter;
    filter1 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_insert(
        filter1,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    filter2 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    bloom_filter_insert(
        filter2,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    );
    result = bloom_filter_union(filter1, filter2);
    if bloom_filter_query(
        result,
        b"test 1\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(result, \"test 1\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_bloom_filter_union(void)\0"))
                .as_ptr(),
        );
    };
    if bloom_filter_query(
        result,
        b"test 2\0" as *const u8 as *const libc::c_char as BloomFilterValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"bloom_filter_query(result, \"test 2\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_bloom_filter_union(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(result);
}
#[no_mangle]
pub unsafe extern "C" fn test_bloom_filter_mismatch() {
    let mut filter1: *mut BloomFilter = 0 as *mut BloomFilter;
    let mut filter2: *mut BloomFilter = 0 as *mut BloomFilter;
    filter1 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    filter2 = bloom_filter_new(
        64 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        4 as libc::c_int as libc::c_uint,
    );
    if (bloom_filter_intersection(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_intersection(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            231 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    if (bloom_filter_union(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_union(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter2);
    filter2 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(
            string_nocase_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint,
        ),
        4 as libc::c_int as libc::c_uint,
    );
    if (bloom_filter_intersection(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_intersection(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    if (bloom_filter_union(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_union(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            239 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter2);
    filter2 = bloom_filter_new(
        128 as libc::c_int as libc::c_uint,
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        32 as libc::c_int as libc::c_uint,
    );
    if (bloom_filter_intersection(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_intersection(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            245 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    if (bloom_filter_union(filter1, filter2)).is_null() {} else {
        __assert_fail(
            b"bloom_filter_union(filter1, filter2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-bloom-filter.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_bloom_filter_mismatch(void)\0"))
                .as_ptr(),
        );
    };
    bloom_filter_free(filter2);
    bloom_filter_free(filter1);
}
static mut tests: [UnitTestFunction; 7] = unsafe {
    [
        Some(test_bloom_filter_new_free as unsafe extern "C" fn() -> ()),
        Some(test_bloom_filter_insert_query as unsafe extern "C" fn() -> ()),
        Some(test_bloom_filter_read_load as unsafe extern "C" fn() -> ()),
        Some(test_bloom_filter_intersection as unsafe extern "C" fn() -> ()),
        Some(test_bloom_filter_union as unsafe extern "C" fn() -> ()),
        Some(test_bloom_filter_mismatch as unsafe extern "C" fn() -> ()),
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
fn s_test_bloom_filter_new_free() {
    unsafe {
        test_bloom_filter_new_free();
    }
}


#[test]
fn s_test_bloom_filter_insert_query() {
    unsafe {
        test_bloom_filter_insert_query();
    }
}


#[test]
fn s_test_bloom_filter_read_load() {
    unsafe {
        test_bloom_filter_read_load();
    }
}


#[test]
fn s_test_bloom_filter_intersection() {
    unsafe {
        test_bloom_filter_intersection();
    }
}


#[test]
fn s_test_bloom_filter_union() {
    unsafe {
        test_bloom_filter_union();
    }
}


#[test]
fn s_test_bloom_filter_mismatch() {
    unsafe {
        test_bloom_filter_mismatch();
    }
}
