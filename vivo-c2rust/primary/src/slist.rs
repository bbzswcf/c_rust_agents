#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SListEntry {
    pub data: SListValue,
    pub next: *mut SListEntry,
}
pub type SListEntry = _SListEntry;
pub type SListValue = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SListIterator {
    pub prev_next: *mut *mut SListEntry,
    pub current: *mut SListEntry,
}
pub type SListIterator = _SListIterator;
pub type SListCompareFunc = Option::<
    unsafe extern "C" fn(SListValue, SListValue) -> libc::c_int,
>;
pub type SListEqualFunc = Option::<
    unsafe extern "C" fn(SListValue, SListValue) -> libc::c_int,
>;
#[no_mangle]
pub extern "C" fn slist_free(mut list: *mut SListEntry) {
    let mut entry: *mut SListEntry = list;
    while !entry.is_null() {
        let next: *mut SListEntry = unsafe { (*entry).next };
        unsafe { alloc_test_free(entry as *mut libc::c_void) };
        entry = next;
    }
}
#[no_mangle]
pub extern "C" fn slist_prepend(
    mut list: *mut *mut SListEntry,
    mut data: SListValue,
) -> *mut SListEntry {
    let mut newentry: *mut SListEntry = 0 as *mut SListEntry;
    unsafe {
        newentry = alloc_test_malloc(::core::mem::size_of::<SListEntry>() as libc::c_ulong)
            as *mut SListEntry;
        if newentry.is_null() {
            return 0 as *mut SListEntry;
        }
        (*newentry).data = data;
        (*newentry).next = *list;
        *list = newentry;
    }
    newentry
}
#[no_mangle]
pub extern "C" fn slist_append(
    mut list: *mut *mut SListEntry,
    mut data: SListValue,
) -> *mut SListEntry {
    let mut rover: *mut SListEntry = std::ptr::null_mut();
    let mut newentry: *mut SListEntry = std::ptr::null_mut();

    unsafe {
        newentry = alloc_test_malloc(std::mem::size_of::<SListEntry>() as size_t) as *mut SListEntry;
        if newentry.is_null() {
            return std::ptr::null_mut();
        }
        (*newentry).data = data;
        (*newentry).next = std::ptr::null_mut();

        if (*list).is_null() {
            *list = newentry;
        } else {
            rover = *list;
            while !(*rover).next.is_null() {
                rover = (*rover).next;
            }
            (*rover).next = newentry;
        }
    }

    newentry
}
#[no_mangle]
pub extern "C" fn slist_data(mut listentry: *mut SListEntry) -> SListValue {
    unsafe {
        return (*listentry).data;
    }
}
#[no_mangle]
pub extern "C" fn slist_set_data(
    mut listentry: *mut SListEntry,
    mut data: SListValue,
) {
    if !listentry.is_null() {
        unsafe {
            (*listentry).data = data;
        }
    }
}
#[no_mangle]
pub extern "C" fn slist_next(mut listentry: *mut SListEntry) -> *mut SListEntry {
    unsafe {
        (*listentry).next
    }
}
#[no_mangle]
pub extern "C" fn slist_nth_entry(
    mut list: *mut SListEntry,
    mut n: libc::c_uint,
) -> *mut SListEntry {
    let mut entry: *mut SListEntry = list;
    let mut i: libc::c_uint = 0;

    while i < n {
        if entry.is_null() {
            return std::ptr::null_mut();
        }
        entry = unsafe { (*entry).next };
        i = i.wrapping_add(1);
    }

    entry
}
#[no_mangle]
pub extern "C" fn slist_nth_data(
    mut list: *mut SListEntry,
    mut n: libc::c_uint,
) -> SListValue {
    let entry = slist_nth_entry(list, n);
    if entry.is_null() {
        0 as *mut libc::c_void
    } else {
        unsafe { (*entry).data }
    }
}
#[no_mangle]
pub extern "C" fn slist_length(mut list: *mut SListEntry) -> libc::c_uint {
    let mut entry: *mut SListEntry = list;
    let mut length: libc::c_uint = 0;

    while !entry.is_null() {
        length = length.wrapping_add(1);
        entry = unsafe { (*entry).next };
    }

    length
}
#[no_mangle]
pub extern "C" fn slist_to_array(mut list: *mut SListEntry) -> *mut SListValue {
    let mut rover: *mut SListEntry = std::ptr::null_mut();
    let mut array: *mut SListValue = std::ptr::null_mut();
    let mut length: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;

    length = slist_length(list);
    unsafe {
        array = alloc_test_malloc(
            (std::mem::size_of::<SListValue>() as libc::c_ulong)
                .wrapping_mul(length as libc::c_ulong),
        ) as *mut SListValue;
    }
    if array.is_null() {
        return std::ptr::null_mut();
    }

    rover = list;
    i = 0;
    while i < length {
        unsafe {
            *array.offset(i as isize) = (*rover).data;
            rover = (*rover).next;
        }
        i = i.wrapping_add(1);
    }

    array
}
#[no_mangle]
pub extern "C" fn slist_remove_entry(
    mut list: *mut *mut SListEntry,
    mut entry: *mut SListEntry,
) -> libc::c_int {
    unsafe {
        if (*list).is_null() || entry.is_null() {
            return 0 as libc::c_int;
        }
        if *list == entry {
            *list = (*entry).next;
        } else {
            let mut rover = *list;
            while !rover.is_null() && (*rover).next != entry {
                rover = (*rover).next;
            }
            if rover.is_null() {
                return 0 as libc::c_int;
            } else {
                (*rover).next = (*entry).next;
            }
        }
        alloc_test_free(entry as *mut libc::c_void);
        return 1 as libc::c_int;
    }
}
#[no_mangle]
pub extern "C" fn slist_remove_data(
    mut list: *mut *mut SListEntry,
    mut callback: SListEqualFunc,
    mut data: SListValue,
) -> libc::c_uint {
    let mut rover: *mut *mut SListEntry = std::ptr::null_mut();
    let mut next: *mut SListEntry = std::ptr::null_mut();
    let mut entries_removed: libc::c_uint = 0;
    entries_removed = 0;
    rover = list;
    while !unsafe { (*rover).is_null() } {
        if unsafe { callback.expect("non-null function pointer")((**rover).data, data) != 0 } {
            next = unsafe { (**rover).next };
            unsafe { alloc_test_free(*rover as *mut libc::c_void) };
            unsafe { *rover = next };
            entries_removed = entries_removed.wrapping_add(1);
        } else {
            rover = unsafe { &mut (**rover).next };
        }
    }
    entries_removed
}
extern "C" fn slist_sort_internal(
    mut list: *mut *mut SListEntry,
    mut compare_func: SListCompareFunc,
) -> *mut SListEntry {
    unsafe {
        let mut pivot: *mut SListEntry = std::ptr::null_mut();
        let mut rover: *mut SListEntry = std::ptr::null_mut();
        let mut less_list: *mut SListEntry = std::ptr::null_mut();
        let mut more_list: *mut SListEntry = std::ptr::null_mut();
        let mut less_list_end: *mut SListEntry = std::ptr::null_mut();
        let mut more_list_end: *mut SListEntry = std::ptr::null_mut();

        if (*list).is_null() || ((**list).next).is_null() {
            return *list;
        }

        pivot = *list;
        less_list = std::ptr::null_mut();
        more_list = std::ptr::null_mut();
        rover = (**list).next;

        while !rover.is_null() {
            let next: *mut SListEntry = (*rover).next;
            if compare_func.expect("non-null function pointer")((*rover).data, (*pivot).data) < 0 {
                (*rover).next = less_list;
                less_list = rover;
            } else {
                (*rover).next = more_list;
                more_list = rover;
            }
            rover = next;
        }

        less_list_end = slist_sort_internal(&mut less_list, compare_func);
        more_list_end = slist_sort_internal(&mut more_list, compare_func);

        *list = less_list;
        if less_list.is_null() {
            *list = pivot;
        } else {
            (*less_list_end).next = pivot;
        }
        (*pivot).next = more_list;

        if more_list.is_null() {
            pivot
        } else {
            more_list_end
        }
    }
}
#[no_mangle]
pub extern "C" fn slist_sort(
    mut list: *mut *mut SListEntry,
    mut compare_func: SListCompareFunc,
) {
    unsafe {
        slist_sort_internal(list, compare_func);
    }
}
#[no_mangle]
pub extern "C" fn slist_find_data(
    mut list: *mut SListEntry,
    mut callback: SListEqualFunc,
    mut data: SListValue,
) -> *mut SListEntry {
    let mut rover: *mut SListEntry = std::ptr::null_mut();
    rover = list;
    while !rover.is_null() {
        if unsafe { callback.expect("non-null function pointer")((*rover).data, data) != 0 } {
            return rover;
        }
        rover = unsafe { (*rover).next };
    }
    return std::ptr::null_mut();
}
#[no_mangle]
pub extern "C" fn slist_iterate(
    mut list: *mut *mut SListEntry,
    mut iter: *mut SListIterator,
) {
    unsafe {
        (*iter).prev_next = list;
        (*iter).current = std::ptr::null_mut();
    }
}
#[no_mangle]
pub extern "C" fn slist_iter_has_more(mut iter: *mut SListIterator) -> libc::c_int {
    if unsafe { ((*iter).current).is_null() || (*iter).current != *(*iter).prev_next } {
        return (unsafe { *(*iter).prev_next != std::ptr::null_mut::<SListEntry>() }) as libc::c_int;
    } else {
        return (unsafe { (*(*iter).current).next != std::ptr::null_mut::<SListEntry>() }) as libc::c_int;
    }
}
#[no_mangle]
pub extern "C" fn slist_iter_next(mut iter: *mut SListIterator) -> SListValue {
    unsafe {
        if ((*iter).current).is_null() || (*iter).current != *(*iter).prev_next {
            (*iter).current = *(*iter).prev_next;
        } else {
            (*iter).prev_next = &mut (*(*iter).current).next;
            (*iter).current = (*(*iter).current).next;
        }
        if ((*iter).current).is_null() {
            return 0 as *mut libc::c_void
        } else {
            return (*(*iter).current).data
        };
    }
}
#[no_mangle]
pub extern "C" fn slist_iter_remove(mut iter: *mut SListIterator) {
    unsafe {
        if !((*iter).current).is_null() && (*iter).current == *(*iter).prev_next {
            *(*iter).prev_next = (*(*iter).current).next;
            alloc_test_free((*iter).current as *mut libc::c_void);
            (*iter).current = std::ptr::null_mut();
        }
    }
}
