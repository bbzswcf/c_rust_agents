#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _Trie;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn trie_new() -> *mut Trie;
    fn trie_free(trie: *mut Trie);
    fn trie_insert(
        trie: *mut Trie,
        key: *mut libc::c_char,
        value: TrieValue,
    ) -> libc::c_int;
    fn trie_insert_binary(
        trie: *mut Trie,
        key: *mut libc::c_uchar,
        key_length: libc::c_int,
        value: TrieValue,
    ) -> libc::c_int;
    fn trie_lookup(trie: *mut Trie, key: *mut libc::c_char) -> TrieValue;
    fn trie_lookup_binary(
        trie: *mut Trie,
        key: *mut libc::c_uchar,
        key_length: libc::c_int,
    ) -> TrieValue;
    fn trie_remove(trie: *mut Trie, key: *mut libc::c_char) -> libc::c_int;
    fn trie_remove_binary(
        trie: *mut Trie,
        key: *mut libc::c_uchar,
        key_length: libc::c_int,
    ) -> libc::c_int;
    fn trie_num_entries(trie: *mut Trie) -> libc::c_uint;
}
pub type size_t = libc::c_ulong;
pub type Trie = _Trie;
pub type TrieValue = *mut libc::c_void;
#[no_mangle]
pub static mut test_array: [libc::c_int; 10000] = [0; 10000];
#[no_mangle]
pub static mut test_strings: [[libc::c_char; 10]; 10000] = [[0; 10]; 10000];
#[no_mangle]
pub static mut bin_key: [libc::c_uchar; 7] = [
    'a' as i32 as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    'c' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut bin_key2: [libc::c_uchar; 8] = [
    'a' as i32 as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    'c' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    0xff as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub static mut bin_key3: [libc::c_uchar; 3] = [
    'a' as i32 as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    'c' as i32 as libc::c_uchar,
];
#[no_mangle]
pub static mut bin_key4: [libc::c_uchar; 4] = [
    'z' as i32 as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    'z' as i32 as libc::c_uchar,
    'z' as i32 as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn generate_trie() -> *mut Trie {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut i: libc::c_int = 0;
    let mut entries: libc::c_uint = 0;
    trie = trie_new();
    entries = 0 as libc::c_int as libc::c_uint;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        sprintf(
            (test_strings[i as usize]).as_mut_ptr(),
            b"%i\0" as *const u8 as *const libc::c_char,
            i,
        );
        if trie_insert(
            trie,
            (test_strings[i as usize]).as_mut_ptr(),
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, test_strings[i], &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"Trie *generate_trie(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2048: {
            if trie_insert(
                trie,
                (test_strings[i as usize]).as_mut_ptr(),
                &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                    as TrieValue,
            ) != 0 as libc::c_int
            {} else {
                __assert_fail(
                    b"trie_insert(trie, test_strings[i], &test_array[i]) != 0\0"
                        as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-trie.c\0" as *const u8
                        as *const libc::c_char,
                    36 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"Trie *generate_trie(void)\0"))
                        .as_ptr(),
                );
            }
        };
        entries = entries.wrapping_add(1);
        entries;
        if trie_num_entries(trie) == entries {} else {
            __assert_fail(
                b"trie_num_entries(trie) == entries\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                40 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 26],
                    &[libc::c_char; 26],
                >(b"Trie *generate_trie(void)\0"))
                    .as_ptr(),
            );
        }
        'c_1995: {
            if trie_num_entries(trie) == entries {} else {
                __assert_fail(
                    b"trie_num_entries(trie) == entries\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-trie.c\0" as *const u8
                        as *const libc::c_char,
                    40 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 26],
                        &[libc::c_char; 26],
                    >(b"Trie *generate_trie(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    return trie;
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_new_free() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    trie = trie_new();
    if !trie.is_null() {} else {
        __assert_fail(
            b"trie != NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2534: {
        if !trie.is_null() {} else {
            __assert_fail(
                b"trie != NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                53 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
    trie = trie_new();
    if trie_insert(
        trie,
        b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"there\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"hello\", \"there\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2470: {
        if trie_insert(
            trie,
            b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"there\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"hello\", \"there\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                61 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_insert(
        trie,
        b"hell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"testing\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"hell\", \"testing\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2414: {
        if trie_insert(
            trie,
            b"hell\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"testing\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"hell\", \"testing\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_insert(
        trie,
        b"testing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"testing\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"testing\", \"testing\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2358: {
        if trie_insert(
            trie,
            b"testing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"testing\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"testing\", \"testing\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                63 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_insert(
        trie,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"asfasf\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"\", \"asfasf\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2301: {
        if trie_insert(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"asfasf\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"\", \"asfasf\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
    trie = trie_new();
    if trie_insert(
        trie,
        b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"there\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"hello\", \"there\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2236: {
        if trie_insert(
            trie,
            b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"there\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"hello\", \"there\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                72 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove(
        trie,
        b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove(trie, \"hello\") != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_trie_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2184: {
        if trie_remove(
            trie,
            b"hello\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove(trie, \"hello\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void test_trie_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_insert() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut entries: libc::c_uint = 0;
    let mut allocated: size_t = 0;
    trie = generate_trie();
    entries = trie_num_entries(trie);
    if trie_insert(
        trie,
        b"hello world\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"hello world\", NULL) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_2638: {
        if trie_insert(
            trie,
            b"hello world\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"hello world\", NULL) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_num_entries(trie) == entries {} else {
        __assert_fail(
            b"trie_num_entries(trie) == entries\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_insert(void)\0"))
                .as_ptr(),
        );
    }
    'c_2591: {
        if trie_num_entries(trie) == entries {} else {
            __assert_fail(
                b"trie_num_entries(trie) == entries\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_insert(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_lookup() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    trie = generate_trie();
    if (trie_lookup(
        trie,
        b"000000000000000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup(trie, \"000000000000000\") == TRIE_NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2857: {
        if (trie_lookup(
            trie,
            b"000000000000000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup(trie, \"000000000000000\") == TRIE_NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                108 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (trie_lookup(
        trie,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup(trie, \"\") == TRIE_NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2804: {
        if (trie_lookup(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup(trie, \"\") == TRIE_NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                109 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        val = trie_lookup(trie, buf.as_mut_ptr()) as *mut libc::c_int;
        if *val == i {} else {
            __assert_fail(
                b"*val == i\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                119 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_lookup(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2731: {
            if *val == i {} else {
                __assert_fail(
                    b"*val == i\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-trie.c\0" as *const u8
                        as *const libc::c_char,
                    119 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"void test_trie_lookup(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_remove() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut i: libc::c_int = 0;
    let mut entries: libc::c_uint = 0;
    trie = generate_trie();
    if trie_remove(
        trie,
        b"000000000000000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove(trie, \"000000000000000\") == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3148: {
        if trie_remove(
            trie,
            b"000000000000000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove(trie, \"000000000000000\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove(trie, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove(trie, \"\") == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            136 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3100: {
        if trie_remove(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove(trie, \"\") == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                136 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    entries = trie_num_entries(trie);
    if entries == 10000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"entries == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_trie_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3054: {
        if entries == 10000 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"entries == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                140 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if trie_remove(trie, buf.as_mut_ptr()) != 0 as libc::c_int {} else {
            __assert_fail(
                b"trie_remove(trie, buf) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                150 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_remove(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2990: {
            if trie_remove(trie, buf.as_mut_ptr()) != 0 as libc::c_int {} else {
                __assert_fail(
                    b"trie_remove(trie, buf) != 0\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-trie.c\0" as *const u8
                        as *const libc::c_char,
                    150 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"void test_trie_remove(void)\0"))
                        .as_ptr(),
                );
            }
        };
        entries = entries.wrapping_sub(1);
        entries;
        if trie_num_entries(trie) == entries {} else {
            __assert_fail(
                b"trie_num_entries(trie) == entries\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                152 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 28],
                    &[libc::c_char; 28],
                >(b"void test_trie_remove(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2943: {
            if trie_num_entries(trie) == entries {} else {
                __assert_fail(
                    b"trie_num_entries(trie) == entries\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-trie.c\0" as *const u8
                        as *const libc::c_char,
                    152 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 28],
                        &[libc::c_char; 28],
                    >(b"void test_trie_remove(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_replace() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    trie = generate_trie();
    val = malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    *val = 999 as libc::c_int;
    if trie_insert(
        trie,
        b"999\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        val as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"999\", val) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_trie_replace(void)\0"))
                .as_ptr(),
        );
    }
    'c_3325: {
        if trie_insert(
            trie,
            b"999\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"999\", val) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_trie_replace(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_num_entries(trie) == 10000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"trie_num_entries(trie) == NUM_TEST_VALUES\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_trie_replace(void)\0"))
                .as_ptr(),
        );
    }
    'c_3281: {
        if trie_num_entries(trie) == 10000 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"trie_num_entries(trie) == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_trie_replace(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_lookup(
        trie,
        b"999\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == val as TrieValue
    {} else {
        __assert_fail(
            b"trie_lookup(trie, \"999\") == val\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            171 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_trie_replace(void)\0"))
                .as_ptr(),
        );
    }
    'c_3227: {
        if trie_lookup(
            trie,
            b"999\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == val as TrieValue
        {} else {
            __assert_fail(
                b"trie_lookup(trie, \"999\") == val\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void test_trie_replace(void)\0"))
                    .as_ptr(),
            );
        }
    };
    free(val as *mut libc::c_void);
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_insert_empty() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut buf: [libc::c_char; 10] = [0; 10];
    trie = trie_new();
    if trie_insert(
        trie,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        buf.as_mut_ptr() as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, \"\", buf) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            184 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_trie_insert_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_3603: {
        if trie_insert(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            buf.as_mut_ptr() as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, \"\", buf) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                184 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_trie_insert_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_num_entries(trie) != 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"trie_num_entries(trie) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_trie_insert_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_3559: {
        if trie_num_entries(trie) != 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"trie_num_entries(trie) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                185 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_trie_insert_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_lookup(trie, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == buf.as_mut_ptr() as TrieValue
    {} else {
        __assert_fail(
            b"trie_lookup(trie, \"\") == buf\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_trie_insert_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_3506: {
        if trie_lookup(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == buf.as_mut_ptr() as TrieValue
        {} else {
            __assert_fail(
                b"trie_lookup(trie, \"\") == buf\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_trie_insert_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove(trie, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove(trie, \"\") != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_trie_insert_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_3458: {
        if trie_remove(
            trie,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove(trie, \"\") != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                187 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_trie_insert_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_num_entries(trie) == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"trie_num_entries(trie) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_trie_insert_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_3414: {
        if trie_num_entries(trie) == 0 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"trie_num_entries(trie) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                189 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_trie_insert_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_free_long() {
    let mut long_string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut trie: *mut Trie = 0 as *mut Trie;
    long_string = malloc(4096 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    memset(
        long_string as *mut libc::c_void,
        'A' as i32,
        4096 as libc::c_int as libc::c_ulong,
    );
    *long_string
        .offset(
            (4096 as libc::c_int - 1 as libc::c_int) as isize,
        ) = '\0' as i32 as libc::c_char;
    trie = trie_new();
    trie_insert(trie, long_string, long_string as TrieValue);
    trie_free(trie);
    free(long_string as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_negative_keys() {
    let mut my_key: [libc::c_char; 6] = [
        'a' as i32 as libc::c_char,
        'b' as i32 as libc::c_char,
        'c' as i32 as libc::c_char,
        -(50 as libc::c_int) as libc::c_char,
        -(20 as libc::c_int) as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    trie = trie_new();
    if trie_insert(
        trie,
        my_key.as_mut_ptr(),
        b"hello world\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert(trie, my_key, \"hello world\") != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_negative_keys(void)\0"))
                .as_ptr(),
        );
    }
    'c_3965: {
        if trie_insert(
            trie,
            my_key.as_mut_ptr(),
            b"hello world\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert(trie, my_key, \"hello world\") != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                221 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_negative_keys(void)\0"))
                    .as_ptr(),
            );
        }
    };
    value = trie_lookup(trie, my_key.as_mut_ptr());
    if strcmp(
        value as *const libc::c_char,
        b"hello world\0" as *const u8 as *const libc::c_char,
    ) == 0
    {} else {
        __assert_fail(
            b"!strcmp(value, \"hello world\")\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_negative_keys(void)\0"))
                .as_ptr(),
        );
    }
    'c_3908: {
        if strcmp(
            value as *const libc::c_char,
            b"hello world\0" as *const u8 as *const libc::c_char,
        ) == 0
        {} else {
            __assert_fail(
                b"!strcmp(value, \"hello world\")\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_negative_keys(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove(trie, my_key.as_mut_ptr()) != 0 as libc::c_int {} else {
        __assert_fail(
            b"trie_remove(trie, my_key) != 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            227 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_negative_keys(void)\0"))
                .as_ptr(),
        );
    }
    'c_3862: {
        if trie_remove(trie, my_key.as_mut_ptr()) != 0 as libc::c_int {} else {
            __assert_fail(
                b"trie_remove(trie, my_key) != 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                227 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_negative_keys(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove(trie, my_key.as_mut_ptr()) == 0 as libc::c_int {} else {
        __assert_fail(
            b"trie_remove(trie, my_key) == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_negative_keys(void)\0"))
                .as_ptr(),
        );
    }
    'c_3816: {
        if trie_remove(trie, my_key.as_mut_ptr()) == 0 as libc::c_int {} else {
            __assert_fail(
                b"trie_remove(trie, my_key) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                228 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_negative_keys(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (trie_lookup(trie, my_key.as_mut_ptr())).is_null() {} else {
        __assert_fail(
            b"trie_lookup(trie, my_key) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            229 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_negative_keys(void)\0"))
                .as_ptr(),
        );
    }
    'c_3748: {
        if (trie_lookup(trie, my_key.as_mut_ptr())).is_null() {} else {
            __assert_fail(
                b"trie_lookup(trie, my_key) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                229 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_negative_keys(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn generate_binary_trie() -> *mut Trie {
    let mut trie: *mut Trie = 0 as *mut Trie;
    trie = trie_new();
    if trie_insert_binary(
        trie,
        bin_key2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
        b"goodbye world\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert_binary(trie, bin_key2, sizeof(bin_key2), \"goodbye world\") != 0\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"Trie *generate_binary_trie(void)\0"))
                .as_ptr(),
        );
    }
    'c_4096: {
        if trie_insert_binary(
            trie,
            bin_key2.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
            b"goodbye world\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert_binary(trie, bin_key2, sizeof(bin_key2), \"goodbye world\") != 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                243 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"Trie *generate_binary_trie(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_insert_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
        b"hello world\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert_binary(trie, bin_key, sizeof(bin_key), \"hello world\") != 0\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"Trie *generate_binary_trie(void)\0"))
                .as_ptr(),
        );
    }
    'c_4033: {
        if trie_insert_binary(
            trie,
            bin_key.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
            b"hello world\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert_binary(trie, bin_key, sizeof(bin_key), \"hello world\") != 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                246 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"Trie *generate_binary_trie(void)\0"))
                    .as_ptr(),
            );
        }
    };
    return trie;
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_insert_binary() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    trie = generate_binary_trie();
    if trie_insert_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
        b"hi world\0" as *const u8 as *const libc::c_char as TrieValue,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert_binary(trie, bin_key, sizeof(bin_key), \"hi world\") != 0\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_insert_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4365: {
        if trie_insert_binary(
            trie,
            bin_key.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
            b"hi world\0" as *const u8 as *const libc::c_char as TrieValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert_binary(trie, bin_key, sizeof(bin_key), \"hi world\") != 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_insert_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_insert_binary(
        trie,
        bin_key3.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong as libc::c_int,
        0 as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_insert_binary(trie, bin_key3, sizeof(bin_key3), NULL) == 0\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_insert_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4304: {
        if trie_insert_binary(
            trie,
            bin_key3.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong as libc::c_int,
            0 as *mut libc::c_void,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_insert_binary(trie, bin_key3, sizeof(bin_key3), NULL) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_insert_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    value = trie_lookup_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    if strcmp(value, b"hi world\0" as *const u8 as *const libc::c_char) == 0 {} else {
        __assert_fail(
            b"!strcmp(value, \"hi world\")\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_insert_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4243: {
        if strcmp(value, b"hi world\0" as *const u8 as *const libc::c_char) == 0
        {} else {
            __assert_fail(
                b"!strcmp(value, \"hi world\")\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                271 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_insert_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    value = trie_lookup_binary(
        trie,
        bin_key2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
    ) as *mut libc::c_char;
    if strcmp(value, b"goodbye world\0" as *const u8 as *const libc::c_char) == 0
    {} else {
        __assert_fail(
            b"!strcmp(value, \"goodbye world\")\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_insert_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4181: {
        if strcmp(value, b"goodbye world\0" as *const u8 as *const libc::c_char) == 0
        {} else {
            __assert_fail(
                b"!strcmp(value, \"goodbye world\")\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                274 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_insert_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
}
#[no_mangle]
pub unsafe extern "C" fn test_trie_remove_binary() {
    let mut trie: *mut Trie = 0 as *mut Trie;
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    trie = generate_binary_trie();
    value = trie_lookup_binary(
        trie,
        bin_key3.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong as libc::c_int,
    );
    if value.is_null() {} else {
        __assert_fail(
            b"value == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4903: {
        if value.is_null() {} else {
            __assert_fail(
                b"value == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                288 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove_binary(
        trie,
        bin_key3.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove_binary(trie, bin_key3, sizeof(bin_key3)) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4849: {
        if trie_remove_binary(
            trie,
            bin_key3.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 3]>() as libc::c_ulong as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove_binary(trie, bin_key3, sizeof(bin_key3)) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                290 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (trie_lookup_binary(
        trie,
        bin_key4.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup_binary(trie, bin_key4, sizeof(bin_key4)) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4793: {
        if (trie_lookup_binary(
            trie,
            bin_key4.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup_binary(trie, bin_key4, sizeof(bin_key4)) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                292 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove_binary(
        trie,
        bin_key4.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
    ) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove_binary(trie, bin_key4, sizeof(bin_key4)) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4739: {
        if trie_remove_binary(
            trie,
            bin_key4.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 4]>() as libc::c_ulong as libc::c_int,
        ) == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove_binary(trie, bin_key4, sizeof(bin_key4)) == 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                293 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove_binary(
        trie,
        bin_key2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove_binary(trie, bin_key2, sizeof(bin_key2)) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4684: {
        if trie_remove_binary(
            trie,
            bin_key2.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove_binary(trie, bin_key2, sizeof(bin_key2)) != 0\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                297 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (trie_lookup_binary(
        trie,
        bin_key2.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup_binary(trie, bin_key2, sizeof(bin_key2)) == NULL\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            298 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4625: {
        if (trie_lookup_binary(
            trie,
            bin_key2.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 8]>() as libc::c_ulong as libc::c_int,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup_binary(trie, bin_key2, sizeof(bin_key2)) == NULL\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                298 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(trie_lookup_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup_binary(trie, bin_key, sizeof(bin_key)) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4567: {
        if !(trie_lookup_binary(
            trie,
            bin_key.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup_binary(trie, bin_key, sizeof(bin_key)) != NULL\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                299 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if trie_remove_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
    ) != 0 as libc::c_int
    {} else {
        __assert_fail(
            b"trie_remove_binary(trie, bin_key, sizeof(bin_key)) != 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4512: {
        if trie_remove_binary(
            trie,
            bin_key.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"trie_remove_binary(trie, bin_key, sizeof(bin_key)) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                301 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (trie_lookup_binary(
        trie,
        bin_key.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
    ))
        .is_null()
    {} else {
        __assert_fail(
            b"trie_lookup_binary(trie, bin_key, sizeof(bin_key)) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
            302 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_trie_remove_binary(void)\0"))
                .as_ptr(),
        );
    }
    'c_4453: {
        if (trie_lookup_binary(
            trie,
            bin_key.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_uchar; 7]>() as libc::c_ulong as libc::c_int,
        ))
            .is_null()
        {} else {
            __assert_fail(
                b"trie_lookup_binary(trie, bin_key, sizeof(bin_key)) == NULL\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-trie.c\0" as *const u8 as *const libc::c_char,
                302 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_trie_remove_binary(void)\0"))
                    .as_ptr(),
            );
        }
    };
    trie_free(trie);
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
