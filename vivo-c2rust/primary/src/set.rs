#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn alloc_test_calloc(nmemb: size_t, bytes: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _Set {
    pub table: *mut *mut SetEntry,
    pub entries: libc::c_uint,
    pub table_size: libc::c_uint,
    pub prime_index: libc::c_uint,
    pub hash_func: SetHashFunc,
    pub equal_func: SetEqualFunc,
    pub free_func: SetFreeFunc,
}
pub type SetFreeFunc = Option::<unsafe extern "C" fn(SetValue) -> ()>;
pub type SetValue = *mut libc::c_void;
pub type SetEqualFunc = Option::<
    unsafe extern "C" fn(SetValue, SetValue) -> libc::c_int,
>;
pub type SetHashFunc = Option::<unsafe extern "C" fn(SetValue) -> libc::c_uint>;
pub type SetEntry = _SetEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SetEntry {
    pub data: SetValue,
    pub next: *mut SetEntry,
}
pub type Set = _Set;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SetIterator {
    pub set: *mut Set,
    pub next_entry: *mut SetEntry,
    pub next_chain: libc::c_uint,
}
pub type SetIterator = _SetIterator;
static mut set_primes: [libc::c_uint; 24] = [
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
static mut set_num_primes: libc::c_uint = 0;
extern "C" fn set_allocate_table(mut set: *mut Set) -> libc::c_int {
    unsafe {
        if (*set).prime_index < set_num_primes {
            (*set).table_size = set_primes[(*set).prime_index as usize];
        } else {
            (*set).table_size = (*set).entries.wrapping_mul(10);
        }
        (*set).table = alloc_test_calloc(
            (*set).table_size as size_t,
            std::mem::size_of::<*mut SetEntry>() as libc::c_ulong,
        ) as *mut *mut SetEntry;
        ((*set).table != std::ptr::null_mut()) as libc::c_int
    }
}
extern "C" fn set_free_entry(mut set: *mut Set, mut entry: *mut SetEntry) {
    unsafe {
        if ((*set).free_func).is_some() {
            ((*set).free_func).expect("non-null function pointer")((*entry).data);
        }
        alloc_test_free(entry as *mut libc::c_void);
    }
}
#[no_mangle]
pub extern "C" fn set_new(
    mut hash_func: SetHashFunc,
    mut equal_func: SetEqualFunc,
) -> *mut Set {
    let mut new_set: *mut Set = std::ptr::null_mut();
    unsafe {
        new_set = alloc_test_malloc(::core::mem::size_of::<Set>() as libc::c_ulong) as *mut Set;
        if new_set.is_null() {
            return std::ptr::null_mut();
        }
        (*new_set).hash_func = hash_func;
        (*new_set).equal_func = equal_func;
        (*new_set).entries = 0 as libc::c_int as libc::c_uint;
        (*new_set).prime_index = 0 as libc::c_int as libc::c_uint;
        (*new_set).free_func = None;
        if set_allocate_table(new_set) == 0 {
            alloc_test_free(new_set as *mut libc::c_void);
            return std::ptr::null_mut();
        }
    }
    new_set
}
#[no_mangle]
pub extern "C" fn set_free(mut set: *mut Set) {
    let mut rover: *mut SetEntry = std::ptr::null_mut();
    let mut next: *mut SetEntry = std::ptr::null_mut();
    let mut i: libc::c_uint = 0;
    i = 0;
    while i < unsafe { (*set).table_size } {
        rover = unsafe { *((*set).table).offset(i as isize) };
        while !rover.is_null() {
            next = unsafe { (*rover).next };
            set_free_entry(set, rover);
            rover = next;
        }
        i = i.wrapping_add(1);
    }
    unsafe {
        alloc_test_free((*set).table as *mut libc::c_void);
        alloc_test_free(set as *mut libc::c_void);
    }
}
#[no_mangle]
pub extern "C" fn set_register_free_function(
    mut set: *mut Set,
    mut free_func: SetFreeFunc,
) {
    unsafe {
        (*set).free_func = free_func;
    }
}
extern "C" fn set_enlarge(mut set: *mut Set) -> libc::c_int {
    let mut rover: *mut SetEntry = std::ptr::null_mut();
    let mut next: *mut SetEntry = std::ptr::null_mut();
    let mut old_table: *mut *mut SetEntry = std::ptr::null_mut();
    let mut old_table_size: libc::c_uint = 0;
    let mut old_prime_index: libc::c_uint = 0;
    let mut index: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;

    unsafe {
        old_table = (*set).table;
        old_table_size = (*set).table_size;
        old_prime_index = (*set).prime_index;
        (*set).prime_index = (*set).prime_index.wrapping_add(1);

        if set_allocate_table(set) == 0 {
            (*set).table = old_table;
            (*set).table_size = old_table_size;
            (*set).prime_index = old_prime_index;
            return 0;
        }

        i = 0;
        while i < old_table_size {
            rover = *old_table.offset(i as isize);
            while !rover.is_null() {
                next = (*rover).next;
                index = ((*set).hash_func.expect("non-null function pointer")((*rover).data)).wrapping_rem((*set).table_size);
                (*rover).next = *((*set).table).offset(index as isize);
                *((*set).table).offset(index as isize) = rover;
                rover = next;
            }
            i = i.wrapping_add(1);
        }

        alloc_test_free(old_table as *mut libc::c_void);
    }

    return 1;
}
#[no_mangle]
pub extern "C" fn set_insert(
    mut set: *mut Set,
    mut data: SetValue,
) -> libc::c_int {
    unsafe {
        if ((*set).entries * 3) / (*set).table_size > 0 {
            if set_enlarge(set) == 0 {
                return 0;
            }
        }

        let index = ((*set).hash_func.expect("non-null function pointer")(data)) % (*set).table_size;
        let mut rover = *((*set).table).offset(index as isize);

        while !rover.is_null() {
            if (*set).equal_func.expect("non-null function pointer")(data, (*rover).data) != 0 {
                return 0;
            }
            rover = (*rover).next;
        }

        let newentry = alloc_test_malloc(std::mem::size_of::<SetEntry>() as size_t) as *mut SetEntry;
        if newentry.is_null() {
            return 0;
        }

        (*newentry).data = data;
        (*newentry).next = *((*set).table).offset(index as isize);
        *((*set).table).offset(index as isize) = newentry;
        (*set).entries += 1;

        return 1;
    }
}
#[no_mangle]
pub extern "C" fn set_remove(
    mut set: *mut Set,
    mut data: SetValue,
) -> libc::c_int {
    unsafe {
        let mut rover: *mut *mut SetEntry = std::ptr::null_mut();
        let mut entry: *mut SetEntry = std::ptr::null_mut();
        let mut index: libc::c_uint = 0;
        index = ((*set).hash_func).expect("non-null function pointer")(data).wrapping_rem((*set).table_size);
        rover = &mut *((*set).table).offset(index as isize) as *mut *mut SetEntry;
        while !(*rover).is_null() {
            if ((*set).equal_func).expect("non-null function pointer")(data, (**rover).data) != 0 {
                entry = *rover;
                *rover = (*entry).next;
                (*set).entries = (*set).entries.wrapping_sub(1);
                set_free_entry(set, entry);
                return 1;
            }
            rover = &mut (**rover).next;
        }
        return 0;
    }
}
#[no_mangle]
pub extern "C" fn set_query(
    mut set: *mut Set,
    mut data: SetValue,
) -> libc::c_int {
    let mut rover: *mut SetEntry = std::ptr::null_mut();
    let mut index: libc::c_uint = 0;

    unsafe {
        index = ((*set).hash_func).expect("non-null function pointer")(data).wrapping_rem((*set).table_size);
        rover = *((*set).table).offset(index as isize);
    }

    while !rover.is_null() {
        if unsafe { ((*set).equal_func).expect("non-null function pointer")(data, (*rover).data) } != 0 {
            return 1;
        }
        rover = unsafe { (*rover).next };
    }

    return 0;
}
#[no_mangle]
pub extern "C" fn set_num_entries(set: *mut Set) -> libc::c_uint {
    unsafe {
        (*set).entries
    }
}
#[no_mangle]
pub extern "C" fn set_to_array(mut set: *mut Set) -> *mut SetValue {
    let mut array: *mut SetValue = 0 as *mut SetValue;
    let mut array_counter: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    let mut rover: *mut SetEntry = 0 as *mut SetEntry;

    unsafe {
        array = alloc_test_malloc(
            (std::mem::size_of::<SetValue>() as libc::c_ulong)
                .wrapping_mul((*set).entries as libc::c_ulong),
        ) as *mut SetValue;
        if array.is_null() {
            return 0 as *mut SetValue;
        }
    }

    array_counter = 0 as libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;

    while i < unsafe { (*set).table_size } {
        rover = unsafe { *((*set).table).offset(i as isize) };
        while !rover.is_null() {
            unsafe {
                let ref mut fresh2 = *array.offset(array_counter as isize);
                *fresh2 = (*rover).data;
            }
            array_counter += 1;
            rover = unsafe { (*rover).next };
        }
        i = i.wrapping_add(1);
    }

    array
}
#[no_mangle]
pub extern "C" fn set_iterate(mut set: *mut Set, mut iter: *mut SetIterator) {
    unsafe {
        (*iter).set = set;
        (*iter).next_entry = std::ptr::null_mut();
        let mut chain: libc::c_uint = 0;
        while chain < (*set).table_size {
            if !(*((*set).table).offset(chain as isize)).is_null() {
                (*iter).next_entry = *((*set).table).offset(chain as isize);
                break;
            }
            chain = chain.wrapping_add(1);
        }
        (*iter).next_chain = chain;
    }
}
#[no_mangle]
pub extern "C" fn set_iter_next(mut iterator: *mut SetIterator) -> SetValue {
    let set: *mut Set;
    let mut result: SetValue = std::ptr::null_mut();
    let mut current_entry: *mut SetEntry;
    let mut chain: libc::c_uint;

    unsafe {
        set = (*iterator).set;
        if (*iterator).next_entry.is_null() {
            return std::ptr::null_mut();
        }
        current_entry = (*iterator).next_entry;
        result = (*current_entry).data;
        if !(*current_entry).next.is_null() {
            (*iterator).next_entry = (*current_entry).next;
        } else {
            (*iterator).next_entry = std::ptr::null_mut();
            chain = (*iterator).next_chain.wrapping_add(1);
            while chain < (*set).table_size {
                if !(*((*set).table).offset(chain as isize)).is_null() {
                    (*iterator).next_entry = *((*set).table).offset(chain as isize);
                    break;
                } else {
                    chain = chain.wrapping_add(1);
                }
            }
            (*iterator).next_chain = chain;
        }
    }
    result
}
#[no_mangle]
pub extern "C" fn set_iter_has_more(
    mut iterator: *mut SetIterator,
) -> libc::c_int {
    unsafe {
        ((*iterator).next_entry != std::ptr::null_mut()) as libc::c_int
    }
}
#[no_mangle]
pub extern "C" fn set_union(mut set1: *mut Set, mut set2: *mut Set) -> *mut Set {
    let mut iterator: SetIterator = _SetIterator {
        set: std::ptr::null_mut(),
        next_entry: std::ptr::null_mut(),
        next_chain: 0,
    };
    let mut new_set: *mut Set = std::ptr::null_mut();
    let mut value: SetValue = std::ptr::null_mut();

    unsafe {
        new_set = set_new((*set1).hash_func, (*set1).equal_func);
        if new_set.is_null() {
            return std::ptr::null_mut();
        }
    }

    set_iterate(set1, &mut iterator);
    while set_iter_has_more(&mut iterator) != 0 {
        value = set_iter_next(&mut iterator);
        if set_insert(new_set, value) == 0 {
            set_free(new_set);
            return std::ptr::null_mut();
        }
    }

    set_iterate(set2, &mut iterator);
    while set_iter_has_more(&mut iterator) != 0 {
        value = set_iter_next(&mut iterator);
        if set_query(new_set, value) == 0 {
            if set_insert(new_set, value) == 0 {
                set_free(new_set);
                return std::ptr::null_mut();
            }
        }
    }

    new_set
}
#[no_mangle]
pub extern "C" fn set_intersection(
    mut set1: *mut Set,
    mut set2: *mut Set,
) -> *mut Set {
    let mut new_set: *mut Set = std::ptr::null_mut();
    let mut iterator: SetIterator = _SetIterator {
        set: std::ptr::null_mut(),
        next_entry: std::ptr::null_mut(),
        next_chain: 0,
    };
    let mut value: SetValue = std::ptr::null_mut();

    unsafe {
        new_set = set_new((*set1).hash_func, (*set2).equal_func);
        if new_set.is_null() {
            return std::ptr::null_mut();
        }
    }

    set_iterate(set1, &mut iterator);
    while set_iter_has_more(&mut iterator) != 0 {
        value = set_iter_next(&mut iterator);
        if set_query(set2, value) != 0 {
            if set_insert(new_set, value) == 0 {
                set_free(new_set);
                return std::ptr::null_mut();
            }
        }
    }

    new_set
}
extern "C" fn run_static_initializers() {
    unsafe {
        set_num_primes = (::core::mem::size_of::<[libc::c_uint; 24]>() / ::core::mem::size_of::<libc::c_int>()) as libc::c_uint;
    }
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
