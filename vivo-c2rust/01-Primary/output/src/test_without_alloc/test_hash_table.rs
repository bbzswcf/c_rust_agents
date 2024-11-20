#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _HashTable;
    pub type _HashTableEntry;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn hash_table_new(
        hash_func: HashTableHashFunc,
        equal_func: HashTableEqualFunc,
    ) -> *mut HashTable;
    fn hash_table_free(hash_table: *mut HashTable);
    fn hash_table_register_free_functions(
        hash_table: *mut HashTable,
        key_free_func: HashTableKeyFreeFunc,
        value_free_func: HashTableValueFreeFunc,
    );
    fn hash_table_insert(
        hash_table: *mut HashTable,
        key: HashTableKey,
        value: HashTableValue,
    ) -> libc::c_int;
    fn hash_table_lookup(
        hash_table: *mut HashTable,
        key: HashTableKey,
    ) -> HashTableValue;
    fn hash_table_remove(hash_table: *mut HashTable, key: HashTableKey) -> libc::c_int;
    fn hash_table_num_entries(hash_table: *mut HashTable) -> libc::c_uint;
    fn hash_table_iterate(hash_table: *mut HashTable, iter: *mut HashTableIterator);
    fn hash_table_iter_has_more(iterator: *mut HashTableIterator) -> libc::c_int;
    fn hash_table_iter_next(iterator: *mut HashTableIterator) -> HashTablePair;
    fn int_hash(location: *mut libc::c_void) -> libc::c_uint;
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn string_hash(string: *mut libc::c_void) -> libc::c_uint;
    fn string_equal(
        string1: *mut libc::c_void,
        string2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type HashTable = _HashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTableIterator {
    pub hash_table: *mut HashTable,
    pub next_entry: *mut HashTableEntry,
    pub next_chain: libc::c_uint,
}
pub type HashTableEntry = _HashTableEntry;
pub type HashTableIterator = _HashTableIterator;
pub type HashTableKey = *mut libc::c_void;
pub type HashTableValue = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTablePair {
    pub key: HashTableKey,
    pub value: HashTableValue,
}
pub type HashTablePair = _HashTablePair;
pub type HashTableHashFunc = Option::<
    unsafe extern "C" fn(HashTableKey) -> libc::c_uint,
>;
pub type HashTableEqualFunc = Option::<
    unsafe extern "C" fn(HashTableKey, HashTableKey) -> libc::c_int,
>;
pub type HashTableKeyFreeFunc = Option::<unsafe extern "C" fn(HashTableKey) -> ()>;
pub type HashTableValueFreeFunc = Option::<unsafe extern "C" fn(HashTableValue) -> ()>;
#[no_mangle]
pub static mut value1: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut value2: libc::c_int = 2 as libc::c_int;
#[no_mangle]
pub static mut value3: libc::c_int = 3 as libc::c_int;
#[no_mangle]
pub static mut value4: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub static mut allocated_keys: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut allocated_values: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn generate_hash_table() -> *mut HashTable {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    hash_table = hash_table_new(
        Some(string_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            string_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        value = strdup(buf.as_mut_ptr());
        hash_table_insert(hash_table, value as HashTableKey, value as HashTableValue);
        i += 1;
        i;
    }
    hash_table_register_free_functions(
        hash_table,
        None,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    return hash_table;
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_new_free() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    hash_table = hash_table_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if !hash_table.is_null() {} else {
        __assert_fail(
            b"hash_table != NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void test_hash_table_new_free(void)\0"))
                .as_ptr(),
        );
    }
    'c_2153: {
        if !hash_table.is_null() {} else {
            __assert_fail(
                b"hash_table != NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void test_hash_table_new_free(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_insert(
        hash_table,
        &mut value1 as *mut libc::c_int as HashTableKey,
        &mut value1 as *mut libc::c_int as HashTableValue,
    );
    hash_table_insert(
        hash_table,
        &mut value2 as *mut libc::c_int as HashTableKey,
        &mut value2 as *mut libc::c_int as HashTableValue,
    );
    hash_table_insert(
        hash_table,
        &mut value3 as *mut libc::c_int as HashTableKey,
        &mut value3 as *mut libc::c_int as HashTableValue,
    );
    hash_table_insert(
        hash_table,
        &mut value4 as *mut libc::c_int as HashTableKey,
        &mut value4 as *mut libc::c_int as HashTableValue,
    );
    hash_table_free(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_insert_lookup() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    hash_table = generate_hash_table();
    if hash_table_num_entries(hash_table) == 10000 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void test_hash_table_insert_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2529: {
        if hash_table_num_entries(hash_table) == 10000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_hash_table_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        value = hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)
            as *mut libc::c_char;
        if strcmp(value, buf.as_mut_ptr()) == 0 as libc::c_int {} else {
            __assert_fail(
                b"strcmp(value, buf) == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                86 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_hash_table_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2450: {
            if strcmp(value, buf.as_mut_ptr()) == 0 as libc::c_int {} else {
                __assert_fail(
                    b"strcmp(value, buf) == 0\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-hash-table.c\0" as *const u8
                        as *const libc::c_char,
                    86 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 41],
                        &[libc::c_char; 41],
                    >(b"void test_hash_table_insert_lookup(void)\0"))
                        .as_ptr(),
                );
            }
        };
        i += 1;
        i;
    }
    sprintf(
        buf.as_mut_ptr(),
        b"%i\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
    {} else {
        __assert_fail(
            b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            92 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void test_hash_table_insert_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2382: {
        if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
        {} else {
            __assert_fail(
                b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                92 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_hash_table_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(
        buf.as_mut_ptr(),
        b"%i\0" as *const u8 as *const libc::c_char,
        10000 as libc::c_int,
    );
    if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
    {} else {
        __assert_fail(
            b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            94 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void test_hash_table_insert_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2321: {
        if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
        {} else {
            __assert_fail(
                b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                94 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_hash_table_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(
        buf.as_mut_ptr(),
        b"%i\0" as *const u8 as *const libc::c_char,
        12345 as libc::c_int,
    );
    hash_table_insert(
        hash_table,
        buf.as_mut_ptr() as HashTableKey,
        strdup(b"hello world\0" as *const u8 as *const libc::c_char) as HashTableValue,
    );
    value = hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)
        as *mut libc::c_char;
    if strcmp(value, b"hello world\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"strcmp(value, \"hello world\") == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[libc::c_char; 41],
            >(b"void test_hash_table_insert_lookup(void)\0"))
                .as_ptr(),
        );
    }
    'c_2233: {
        if strcmp(value, b"hello world\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {} else {
            __assert_fail(
                b"strcmp(value, \"hello world\") == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                101 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_hash_table_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_free(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_remove() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut buf: [libc::c_char; 10] = [0; 10];
    hash_table = generate_hash_table();
    if hash_table_num_entries(hash_table) == 10000 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            112 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_hash_table_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_2829: {
        if hash_table_num_entries(hash_table) == 10000 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                112 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_hash_table_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(
        buf.as_mut_ptr(),
        b"%i\0" as *const u8 as *const libc::c_char,
        5000 as libc::c_int,
    );
    if !(hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
    {} else {
        __assert_fail(
            b"hash_table_lookup(hash_table, buf) != NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            114 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_hash_table_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_2769: {
        if !(hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
        {} else {
            __assert_fail(
                b"hash_table_lookup(hash_table, buf) != NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                114 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_hash_table_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_remove(hash_table, buf.as_mut_ptr() as HashTableKey);
    if hash_table_num_entries(hash_table) == 9999 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"hash_table_num_entries(hash_table) == 9999\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_hash_table_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_2717: {
        if hash_table_num_entries(hash_table) == 9999 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"hash_table_num_entries(hash_table) == 9999\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_hash_table_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
    {} else {
        __assert_fail(
            b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_hash_table_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_2665: {
        if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey)).is_null()
        {} else {
            __assert_fail(
                b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_hash_table_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    sprintf(
        buf.as_mut_ptr(),
        b"%i\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    hash_table_remove(hash_table, buf.as_mut_ptr() as HashTableKey);
    if hash_table_num_entries(hash_table) == 9999 as libc::c_int as libc::c_uint
    {} else {
        __assert_fail(
            b"hash_table_num_entries(hash_table) == 9999\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_hash_table_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_2602: {
        if hash_table_num_entries(hash_table) == 9999 as libc::c_int as libc::c_uint
        {} else {
            __assert_fail(
                b"hash_table_num_entries(hash_table) == 9999\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_hash_table_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_free(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_iterating() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut iterator: HashTableIterator = _HashTableIterator {
        hash_table: 0 as *mut HashTable,
        next_entry: 0 as *mut HashTableEntry,
        next_chain: 0,
    };
    let mut count: libc::c_int = 0;
    hash_table = generate_hash_table();
    count = 0 as libc::c_int;
    hash_table_iterate(hash_table, &mut iterator);
    while hash_table_iter_has_more(&mut iterator) != 0 {
        hash_table_iter_next(&mut iterator);
        count += 1;
        count;
    }
    if count == 10000 as libc::c_int {} else {
        __assert_fail(
            b"count == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            157 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_hash_table_iterating(void)\0"))
                .as_ptr(),
        );
    }
    'c_3017: {
        if count == 10000 as libc::c_int {} else {
            __assert_fail(
                b"count == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                157 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_hash_table_iterating(void)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut pair: HashTablePair = hash_table_iter_next(&mut iterator);
    if (pair.value).is_null() {} else {
        __assert_fail(
            b"pair.value == HASH_TABLE_NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_hash_table_iterating(void)\0"))
                .as_ptr(),
        );
    }
    'c_2967: {
        if (pair.value).is_null() {} else {
            __assert_fail(
                b"pair.value == HASH_TABLE_NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_hash_table_iterating(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_free(hash_table);
    hash_table = hash_table_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hash_table_iterate(hash_table, &mut iterator);
    if hash_table_iter_has_more(&mut iterator) == 0 as libc::c_int {} else {
        __assert_fail(
            b"hash_table_iter_has_more(&iterator) == 0\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            172 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_hash_table_iterating(void)\0"))
                .as_ptr(),
        );
    }
    'c_2901: {
        if hash_table_iter_has_more(&mut iterator) == 0 as libc::c_int {} else {
            __assert_fail(
                b"hash_table_iter_has_more(&iterator) == 0\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_hash_table_iterating(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_free(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_iterating_remove() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut iterator: HashTableIterator = _HashTableIterator {
        hash_table: 0 as *mut HashTable,
        next_entry: 0 as *mut HashTableEntry,
        next_chain: 0,
    };
    let mut buf: [libc::c_char; 10] = [0; 10];
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pair: HashTablePair = _HashTablePair {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
    };
    let mut count: libc::c_int = 0;
    let mut removed: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    hash_table = generate_hash_table();
    count = 0 as libc::c_int;
    removed = 0 as libc::c_int as libc::c_uint;
    hash_table_iterate(hash_table, &mut iterator);
    while hash_table_iter_has_more(&mut iterator) != 0 {
        pair = hash_table_iter_next(&mut iterator);
        val = pair.value as *mut libc::c_char;
        if atoi(val) % 100 as libc::c_int == 0 as libc::c_int {
            hash_table_remove(hash_table, val as HashTableKey);
            removed = removed.wrapping_add(1);
            removed;
        }
        count += 1;
        count;
    }
    if removed == 100 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"removed == 100\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_hash_table_iterating_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3339: {
        if removed == 100 as libc::c_int as libc::c_uint {} else {
            __assert_fail(
                b"removed == 100\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                215 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void test_hash_table_iterating_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if count == 10000 as libc::c_int {} else {
        __assert_fail(
            b"count == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_hash_table_iterating_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3302: {
        if count == 10000 as libc::c_int {} else {
            __assert_fail(
                b"count == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                216 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void test_hash_table_iterating_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if hash_table_num_entries(hash_table)
        == (10000 as libc::c_int as libc::c_uint).wrapping_sub(removed)
    {} else {
        __assert_fail(
            b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES - removed\0"
                as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            219 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void test_hash_table_iterating_remove(void)\0"))
                .as_ptr(),
        );
    }
    'c_3250: {
        if hash_table_num_entries(hash_table)
            == (10000 as libc::c_int as libc::c_uint).wrapping_sub(removed)
        {} else {
            __assert_fail(
                b"hash_table_num_entries(hash_table) == NUM_TEST_VALUES - removed\0"
                    as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                219 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void test_hash_table_iterating_remove(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        sprintf(buf.as_mut_ptr(), b"%i\0" as *const u8 as *const libc::c_char, i);
        if i % 100 as libc::c_int == 0 as libc::c_int {
            if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey))
                .is_null()
            {} else {
                __assert_fail(
                    b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-hash-table.c\0" as *const u8
                        as *const libc::c_char,
                    227 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"void test_hash_table_iterating_remove(void)\0"))
                        .as_ptr(),
                );
            }
            'c_3176: {
                if (hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey))
                    .is_null()
                {} else {
                    __assert_fail(
                        b"hash_table_lookup(hash_table, buf) == NULL\0" as *const u8
                            as *const libc::c_char,
                        b"test-without-alloc/test-hash-table.c\0" as *const u8
                            as *const libc::c_char,
                        227 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"void test_hash_table_iterating_remove(void)\0"))
                            .as_ptr(),
                    );
                }
            };
        } else {
            if !(hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey))
                .is_null()
            {} else {
                __assert_fail(
                    b"hash_table_lookup(hash_table, buf) != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-hash-table.c\0" as *const u8
                        as *const libc::c_char,
                    229 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 44],
                        &[libc::c_char; 44],
                    >(b"void test_hash_table_iterating_remove(void)\0"))
                        .as_ptr(),
                );
            }
            'c_3121: {
                if !(hash_table_lookup(hash_table, buf.as_mut_ptr() as HashTableKey))
                    .is_null()
                {} else {
                    __assert_fail(
                        b"hash_table_lookup(hash_table, buf) != NULL\0" as *const u8
                            as *const libc::c_char,
                        b"test-without-alloc/test-hash-table.c\0" as *const u8
                            as *const libc::c_char,
                        229 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 44],
                            &[libc::c_char; 44],
                        >(b"void test_hash_table_iterating_remove(void)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        i += 1;
        i;
    }
    hash_table_free(hash_table);
}
#[no_mangle]
pub unsafe extern "C" fn new_key(mut value: libc::c_int) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = 0 as *mut libc::c_int;
    result = malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    *result = value;
    allocated_keys += 1;
    allocated_keys;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn free_key(mut key: *mut libc::c_void) {
    free(key);
    allocated_keys -= 1;
    allocated_keys;
}
#[no_mangle]
pub unsafe extern "C" fn new_value(mut value: libc::c_int) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = 0 as *mut libc::c_int;
    result = malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as *mut libc::c_int;
    *result = value;
    allocated_values += 1;
    allocated_values;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn free_value(mut value: *mut libc::c_void) {
    free(value);
    allocated_values -= 1;
    allocated_values;
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_table_free_functions() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut key: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    hash_table = hash_table_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hash_table_register_free_functions(
        hash_table,
        Some(free_key as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        Some(free_value as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    allocated_values = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        key = new_key(i);
        value = new_value(99 as libc::c_int);
        hash_table_insert(hash_table, key as HashTableKey, value as HashTableValue);
        i += 1;
        i;
    }
    if allocated_keys == 10000 as libc::c_int {} else {
        __assert_fail(
            b"allocated_keys == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3947: {
        if allocated_keys == 10000 as libc::c_int {} else {
            __assert_fail(
                b"allocated_keys == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                291 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if allocated_values == 10000 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            292 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3911: {
        if allocated_values == 10000 as libc::c_int {} else {
            __assert_fail(
                b"allocated_values == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                292 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 10000 as libc::c_int / 2 as libc::c_int;
    hash_table_remove(hash_table, &mut i as *mut libc::c_int as HashTableKey);
    if allocated_keys == 10000 as libc::c_int - 1 as libc::c_int {} else {
        __assert_fail(
            b"allocated_keys == NUM_TEST_VALUES - 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            299 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3857: {
        if allocated_keys == 10000 as libc::c_int - 1 as libc::c_int {} else {
            __assert_fail(
                b"allocated_keys == NUM_TEST_VALUES - 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                299 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if allocated_values == 10000 as libc::c_int - 1 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == NUM_TEST_VALUES - 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            300 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3817: {
        if allocated_values == 10000 as libc::c_int - 1 as libc::c_int {} else {
            __assert_fail(
                b"allocated_values == NUM_TEST_VALUES - 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                300 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    key = new_key(10000 as libc::c_int / 3 as libc::c_int);
    value = new_value(999 as libc::c_int);
    if allocated_keys == 10000 as libc::c_int {} else {
        __assert_fail(
            b"allocated_keys == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            307 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3766: {
        if allocated_keys == 10000 as libc::c_int {} else {
            __assert_fail(
                b"allocated_keys == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                307 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if allocated_values == 10000 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == NUM_TEST_VALUES\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            308 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3730: {
        if allocated_values == 10000 as libc::c_int {} else {
            __assert_fail(
                b"allocated_values == NUM_TEST_VALUES\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                308 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_insert(hash_table, key as HashTableKey, value as HashTableValue);
    if allocated_keys == 10000 as libc::c_int - 1 as libc::c_int {} else {
        __assert_fail(
            b"allocated_keys == NUM_TEST_VALUES - 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            312 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3676: {
        if allocated_keys == 10000 as libc::c_int - 1 as libc::c_int {} else {
            __assert_fail(
                b"allocated_keys == NUM_TEST_VALUES - 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                312 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if allocated_values == 10000 as libc::c_int - 1 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == NUM_TEST_VALUES - 1\0" as *const u8
                as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            313 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3635: {
        if allocated_values == 10000 as libc::c_int - 1 as libc::c_int {} else {
            __assert_fail(
                b"allocated_values == NUM_TEST_VALUES - 1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                313 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    hash_table_free(hash_table);
    if allocated_keys == 0 as libc::c_int {} else {
        __assert_fail(
            b"allocated_keys == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            319 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3592: {
        if allocated_keys == 0 as libc::c_int {} else {
            __assert_fail(
                b"allocated_keys == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                319 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if allocated_values == 0 as libc::c_int {} else {
        __assert_fail(
            b"allocated_values == 0\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-hash-table.c\0" as *const u8
                as *const libc::c_char,
            320 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 42],
                &[libc::c_char; 42],
            >(b"void test_hash_table_free_functions(void)\0"))
                .as_ptr(),
        );
    }
    'c_3554: {
        if allocated_values == 0 as libc::c_int {} else {
            __assert_fail(
                b"allocated_values == 0\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                320 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 42],
                    &[libc::c_char; 42],
                >(b"void test_hash_table_free_functions(void)\0"))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_iterator_key_pair() {
    let mut hash_table: *mut HashTable = 0 as *mut HashTable;
    let mut iterator: HashTableIterator = _HashTableIterator {
        hash_table: 0 as *mut HashTable,
        next_entry: 0 as *mut HashTableEntry,
        next_chain: 0,
    };
    let mut pair: HashTablePair = _HashTablePair {
        key: 0 as *mut libc::c_void,
        value: 0 as *mut libc::c_void,
    };
    hash_table = hash_table_new(
        Some(int_hash as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_uint),
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    hash_table_insert(
        hash_table,
        &mut value1 as *mut libc::c_int as HashTableKey,
        &mut value1 as *mut libc::c_int as HashTableValue,
    );
    hash_table_insert(
        hash_table,
        &mut value2 as *mut libc::c_int as HashTableKey,
        &mut value2 as *mut libc::c_int as HashTableValue,
    );
    hash_table_iterate(hash_table, &mut iterator);
    while hash_table_iter_has_more(&mut iterator) != 0 {
        pair = hash_table_iter_next(&mut iterator);
        let mut key: *mut libc::c_int = pair.key as *mut libc::c_int;
        let mut val: *mut libc::c_int = pair.value as *mut libc::c_int;
        if *key == *val {} else {
            __assert_fail(
                b"*key == *val\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-hash-table.c\0" as *const u8
                    as *const libc::c_char,
                345 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_hash_iterator_key_pair()\0"))
                    .as_ptr(),
            );
        }
        'c_4066: {
            if *key == *val {} else {
                __assert_fail(
                    b"*key == *val\0" as *const u8 as *const libc::c_char,
                    b"test-without-alloc/test-hash-table.c\0" as *const u8
                        as *const libc::c_char,
                    345 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 35],
                        &[libc::c_char; 35],
                    >(b"void test_hash_iterator_key_pair()\0"))
                        .as_ptr(),
                );
            }
        };
    }
    hash_table_free(hash_table);
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
