#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn alloc_test_calloc(nmemb: size_t, bytes: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTable {
    pub table: *mut *mut HashTableEntry,
    pub table_size: libc::c_uint,
    pub hash_func: HashTableHashFunc,
    pub equal_func: HashTableEqualFunc,
    pub key_free_func: HashTableKeyFreeFunc,
    pub value_free_func: HashTableValueFreeFunc,
    pub entries: libc::c_uint,
    pub prime_index: libc::c_uint,
}
pub type HashTableValueFreeFunc = Option::<unsafe extern "C" fn(HashTableValue) -> ()>;
pub type HashTableValue = *mut libc::c_void;
pub type HashTableKeyFreeFunc = Option::<unsafe extern "C" fn(HashTableKey) -> ()>;
pub type HashTableKey = *mut libc::c_void;
pub type HashTableEqualFunc = Option::<
    unsafe extern "C" fn(HashTableKey, HashTableKey) -> libc::c_int,
>;
pub type HashTableHashFunc = Option::<
    unsafe extern "C" fn(HashTableKey) -> libc::c_uint,
>;
pub type HashTableEntry = _HashTableEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTableEntry {
    pub pair: HashTablePair,
    pub next: *mut HashTableEntry,
}
pub type HashTablePair = _HashTablePair;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTablePair {
    pub key: HashTableKey,
    pub value: HashTableValue,
}
pub type HashTable = _HashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _HashTableIterator {
    pub hash_table: *mut HashTable,
    pub next_entry: *mut HashTableEntry,
    pub next_chain: libc::c_uint,
}
pub type HashTableIterator = _HashTableIterator;
static mut hash_table_primes: [libc::c_uint; 24] = [
    193 as libc::c_int as libc::c_uint,
    389 as libc::c_int as libc::c_uint,
    769 as libc::c_int as libc::c_uint,
    1543 as libc::c_int as libc::c_uint,
    3079 as libc::c_int as libc::c_uint,
    6151 as libc::c_int as libc::c_uint,
    12289 as libc::c_int as libc::c_uint,
    24593 as libc::c_int as libc::c_uint,
    49157 as libc::c_int as libc::c_uint,
    98317 as libc::c_int as libc::c_uint,
    196613 as libc::c_int as libc::c_uint,
    393241 as libc::c_int as libc::c_uint,
    786433 as libc::c_int as libc::c_uint,
    1572869 as libc::c_int as libc::c_uint,
    3145739 as libc::c_int as libc::c_uint,
    6291469 as libc::c_int as libc::c_uint,
    12582917 as libc::c_int as libc::c_uint,
    25165843 as libc::c_int as libc::c_uint,
    50331653 as libc::c_int as libc::c_uint,
    100663319 as libc::c_int as libc::c_uint,
    201326611 as libc::c_int as libc::c_uint,
    402653189 as libc::c_int as libc::c_uint,
    805306457 as libc::c_int as libc::c_uint,
    1610612741 as libc::c_int as libc::c_uint,
];
static mut hash_table_num_primes: libc::c_uint = 0;
extern "C" fn hash_table_allocate_table(
    mut hash_table: *mut HashTable,
) -> libc::c_int {
    let mut new_table_size: libc::c_uint = 0;
    unsafe {
        if (*hash_table).prime_index < hash_table_num_primes {
            new_table_size = hash_table_primes[(*hash_table).prime_index as usize];
        } else {
            new_table_size = ((*hash_table).entries).wrapping_mul(10);
        }
        (*hash_table).table_size = new_table_size;
        (*hash_table).table = alloc_test_calloc(
            (*hash_table).table_size as size_t,
            ::core::mem::size_of::<*mut HashTableEntry>() as libc::c_ulong,
        ) as *mut *mut HashTableEntry;
    }
    unsafe {
        ((*hash_table).table != 0 as *mut libc::c_void as *mut *mut HashTableEntry) as libc::c_int
    }
}
extern "C" fn hash_table_free_entry(
    hash_table: *mut HashTable,
    entry: *mut HashTableEntry,
) {
    let pair: *mut HashTablePair = unsafe { &mut (*entry).pair };
    if unsafe { ((*hash_table).key_free_func).is_some() } {
        unsafe { ((*hash_table).key_free_func).expect("non-null function pointer")((*pair).key) };
    }
    if unsafe { ((*hash_table).value_free_func).is_some() } {
        unsafe { ((*hash_table).value_free_func).expect("non-null function pointer")((*pair).value) };
    }
    unsafe { alloc_test_free(entry as *mut libc::c_void) };
}
#[no_mangle]
pub extern "C" fn hash_table_new(
    mut hash_func: HashTableHashFunc,
    mut equal_func: HashTableEqualFunc,
) -> *mut HashTable {
    let mut hash_table: *mut HashTable = std::ptr::null_mut();
    unsafe {
        hash_table = alloc_test_malloc(::core::mem::size_of::<HashTable>() as libc::c_ulong)
            as *mut HashTable;
        if hash_table.is_null() {
            return std::ptr::null_mut();
        }
        (*hash_table).hash_func = hash_func;
        (*hash_table).equal_func = equal_func;
        (*hash_table).key_free_func = None;
        (*hash_table).value_free_func = None;
        (*hash_table).entries = 0 as libc::c_int as libc::c_uint;
        (*hash_table).prime_index = 0 as libc::c_int as libc::c_uint;
        if hash_table_allocate_table(hash_table) == 0 {
            alloc_test_free(hash_table as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    hash_table
}
#[no_mangle]
pub extern "C" fn hash_table_free(mut hash_table: *mut HashTable) {
    let mut rover: *mut HashTableEntry;
    let mut next: *mut HashTableEntry;
    let mut i: libc::c_uint = 0;

    while i < unsafe { (*hash_table).table_size } {
        rover = unsafe { *((*hash_table).table).offset(i as isize) };
        while !rover.is_null() {
            next = unsafe { (*rover).next };
            hash_table_free_entry(hash_table, rover);
            rover = next;
        }
        i = i.wrapping_add(1);
    }
    unsafe {
        alloc_test_free((*hash_table).table as *mut libc::c_void);
        alloc_test_free(hash_table as *mut libc::c_void);
    }
}
#[no_mangle]
pub extern "C" fn hash_table_register_free_functions(
    mut hash_table: *mut HashTable,
    mut key_free_func: HashTableKeyFreeFunc,
    mut value_free_func: HashTableValueFreeFunc,
) {
    unsafe {
        (*hash_table).key_free_func = key_free_func;
        (*hash_table).value_free_func = value_free_func;
    }
}
extern "C" fn hash_table_enlarge(mut hash_table: *mut HashTable) -> libc::c_int {
    let mut old_table: *mut *mut HashTableEntry = std::ptr::null_mut();
    let mut old_table_size: libc::c_uint = 0;
    let mut old_prime_index: libc::c_uint = 0;
    let mut rover: *mut HashTableEntry = std::ptr::null_mut();
    let mut pair: *mut HashTablePair = std::ptr::null_mut();
    let mut next: *mut HashTableEntry = std::ptr::null_mut();
    let mut index: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;

    unsafe {
        old_table = (*hash_table).table;
        old_table_size = (*hash_table).table_size;
        old_prime_index = (*hash_table).prime_index;
        (*hash_table).prime_index = ((*hash_table).prime_index).wrapping_add(1);

        if hash_table_allocate_table(hash_table) == 0 {
            (*hash_table).table = old_table;
            (*hash_table).table_size = old_table_size;
            (*hash_table).prime_index = old_prime_index;
            return 0;
        }

        i = 0;
        while i < old_table_size {
            rover = *old_table.offset(i as isize);
            while !rover.is_null() {
                next = (*rover).next;
                pair = &mut (*rover).pair;
                index = ((*hash_table).hash_func.expect("non-null function pointer")((*pair).key)).wrapping_rem((*hash_table).table_size);
                (*rover).next = *((*hash_table).table).offset(index as isize);
                *((*hash_table).table).offset(index as isize) = rover;
                rover = next;
            }
            i = i.wrapping_add(1);
        }

        alloc_test_free(old_table as *mut libc::c_void);
    }

    return 1;
}
#[no_mangle]
pub extern "C" fn hash_table_insert(
    mut hash_table: *mut HashTable,
    mut key: HashTableKey,
    mut value: HashTableValue,
) -> libc::c_int {
    let mut rover: *mut HashTableEntry = std::ptr::null_mut();
    let mut pair: *mut HashTablePair = std::ptr::null_mut();
    let mut newentry: *mut HashTableEntry = std::ptr::null_mut();
    let mut index: libc::c_uint = 0;

    if unsafe {
        ((*hash_table).entries).wrapping_mul(3).wrapping_div((*hash_table).table_size) > 0
    } {
        if hash_table_enlarge(hash_table) == 0 {
            return 0;
        }
    }

    index = unsafe {
        ((*hash_table).hash_func.expect("non-null function pointer")(key))
            .wrapping_rem((*hash_table).table_size)
    };
    rover = unsafe { *((*hash_table).table).offset(index as isize) };

    while !rover.is_null() {
        pair = unsafe { &mut (*rover).pair };
        if unsafe {
            ((*hash_table).equal_func.expect("non-null function pointer")((*pair).key, key)) != 0
        } {
            if unsafe { (*hash_table).value_free_func.is_some() } {
                unsafe {
                    (*hash_table).value_free_func.expect("non-null function pointer")((*pair).value)
                };
            }
            if unsafe { (*hash_table).key_free_func.is_some() } {
                unsafe {
                    (*hash_table).key_free_func.expect("non-null function pointer")((*pair).key)
                };
            }
            unsafe {
                (*pair).key = key;
                (*pair).value = value;
            }
            return 1;
        }
        rover = unsafe { (*rover).next };
    }

    newentry = unsafe {
        alloc_test_malloc(std::mem::size_of::<HashTableEntry>().try_into().unwrap())
            as *mut HashTableEntry
    };
    if newentry.is_null() {
        return 0;
    }

    unsafe {
        (*newentry).pair.key = key;
        (*newentry).pair.value = value;
        (*newentry).next = *((*hash_table).table).offset(index as isize);
        let ref mut fresh1 = *((*hash_table).table).offset(index as isize);
        *fresh1 = newentry;
        (*hash_table).entries = (*hash_table).entries.wrapping_add(1);
    }

    return 1;
}
#[no_mangle]
pub extern "C" fn hash_table_lookup(
    mut hash_table: *mut HashTable,
    mut key: HashTableKey,
) -> HashTableValue {
    let mut rover: *mut HashTableEntry = std::ptr::null_mut();
    let mut pair: *mut HashTablePair = std::ptr::null_mut();
    let mut index: libc::c_uint = 0;
    index = (unsafe { ((*hash_table).hash_func).expect("non-null function pointer")(key) })
        .wrapping_rem(unsafe { (*hash_table).table_size });
    rover = unsafe { *((*hash_table).table).offset(index as isize) };
    while !rover.is_null() {
        pair = unsafe { &mut (*rover).pair };
        if unsafe { ((*hash_table).equal_func).expect("non-null function pointer")(key, (*pair).key) } != 0 {
            return unsafe { (*pair).value };
        }
        rover = unsafe { (*rover).next };
    }
    return std::ptr::null_mut();
}
#[no_mangle]
pub extern "C" fn hash_table_remove(
    mut hash_table: *mut HashTable,
    mut key: HashTableKey,
) -> libc::c_int {
    let mut rover: *mut *mut HashTableEntry = std::ptr::null_mut();
    let mut entry: *mut HashTableEntry = std::ptr::null_mut();
    let mut pair: *mut HashTablePair = std::ptr::null_mut();
    let mut index: libc::c_uint = 0;
    let mut result: libc::c_int = 0;

    unsafe {
        index = ((*hash_table).hash_func).expect("non-null function pointer")(key)
            .wrapping_rem((*hash_table).table_size);
        result = 0;
        rover = &mut *((*hash_table).table).offset(index as isize) as *mut *mut HashTableEntry;

        while !(*rover).is_null() {
            pair = &mut (**rover).pair;
            if ((*hash_table).equal_func)
                .expect("non-null function pointer")(key, (*pair).key) != 0
            {
                entry = *rover;
                *rover = (*entry).next;
                hash_table_free_entry(hash_table, entry);
                (*hash_table).entries = (*hash_table).entries.wrapping_sub(1);
                result = 1;
                break;
            } else {
                rover = &mut (**rover).next;
            }
        }
    }

    result
}
#[no_mangle]
pub extern "C" fn hash_table_num_entries(
    hash_table: *mut HashTable,
) -> libc::c_uint {
    unsafe {
        (*hash_table).entries
    }
}
#[no_mangle]
pub extern "C" fn hash_table_iterate(
    mut hash_table: *mut HashTable,
    mut iterator: *mut HashTableIterator,
) {
    unsafe {
        (*iterator).hash_table = hash_table;
        (*iterator).next_entry = std::ptr::null_mut();
        (*iterator).next_chain = 0;

        let mut chain: libc::c_uint = 0;
        while chain < (*hash_table).table_size {
            if !(*((*hash_table).table).offset(chain as isize)).is_null() {
                (*iterator).next_entry = *((*hash_table).table).offset(chain as isize);
                (*iterator).next_chain = chain;
                break;
            }
            chain = chain.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub extern "C" fn hash_table_iter_has_more(
    mut iterator: *mut HashTableIterator,
) -> libc::c_int {
    unsafe {
        ((*iterator).next_entry != std::ptr::null_mut::<HashTableEntry>()) as libc::c_int
    }
}
#[no_mangle]
pub extern "C" fn hash_table_iter_next(
    mut iterator: *mut HashTableIterator,
) -> HashTablePair {
    let mut current_entry: *mut HashTableEntry = std::ptr::null_mut();
    let mut hash_table: *mut HashTable = std::ptr::null_mut();
    let mut pair: HashTablePair = HashTablePair {
        key: std::ptr::null_mut(),
        value: std::ptr::null_mut(),
    };
    let mut chain: libc::c_uint = 0;

    unsafe {
        hash_table = (*iterator).hash_table;
        if ((*iterator).next_entry).is_null() {
            return pair;
        }
        current_entry = (*iterator).next_entry;
        pair = (*current_entry).pair;
        if !((*current_entry).next).is_null() {
            (*iterator).next_entry = (*current_entry).next;
        } else {
            chain = ((*iterator).next_chain).wrapping_add(1);
            (*iterator).next_entry = std::ptr::null_mut();
            while chain < (*hash_table).table_size {
                if !(*((*hash_table).table).offset(chain as isize)).is_null() {
                    (*iterator).next_entry = *((*hash_table).table).offset(chain as isize);
                    break;
                } else {
                    chain = chain.wrapping_add(1);
                }
            }
            (*iterator).next_chain = chain;
        }
    }
    pair
}
extern "C" fn run_static_initializers() {
    unsafe {
        hash_table_num_primes = (::core::mem::size_of::<[libc::c_uint; 24]>() / ::core::mem::size_of::<libc::c_int>()) as libc::c_uint;
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
