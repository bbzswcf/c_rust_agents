#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn alloc_test_realloc(ptr: *mut libc::c_void, bytes: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type SortedArrayValue = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _SortedArray {
    pub data: *mut SortedArrayValue,
    pub length: libc::c_uint,
    pub _alloced: libc::c_uint,
    pub equ_func: SortedArrayEqualFunc,
    pub cmp_func: SortedArrayCompareFunc,
}
pub type SortedArrayCompareFunc = Option::<
    unsafe extern "C" fn(SortedArrayValue, SortedArrayValue) -> libc::c_int,
>;
pub type SortedArrayEqualFunc = Option::<
    unsafe extern "C" fn(SortedArrayValue, SortedArrayValue) -> libc::c_int,
>;
pub type SortedArray = _SortedArray;
extern "C" fn sortedarray_first_index(
    mut sortedarray: *mut SortedArray,
    mut data: SortedArrayValue,
    mut left: libc::c_uint,
    mut right: libc::c_uint,
) -> libc::c_uint {
    let mut index: libc::c_uint = left;
    while left < right {
        index = left.wrapping_add(right).wrapping_div(2);
        let order: libc::c_int = unsafe {
            ((*sortedarray).cmp_func).expect("non-null function pointer")(
                data,
                *((*sortedarray).data).offset(index as isize),
            )
        };
        if order > 0 {
            left = index.wrapping_add(1);
        } else {
            right = index;
        }
    }
    index
}
extern "C" fn sortedarray_last_index(
    mut sortedarray: *mut SortedArray,
    mut data: SortedArrayValue,
    mut left: libc::c_uint,
    mut right: libc::c_uint,
) -> libc::c_uint {
    let mut index: libc::c_uint = right;
    while left < right {
        index = left.wrapping_add(right).wrapping_div(2);
        let order: libc::c_int = unsafe {
            ((*sortedarray).cmp_func).expect("non-null function pointer")(
                data,
                *((*sortedarray).data).offset(index as isize),
            )
        };
        if order <= 0 {
            left = index.wrapping_add(1);
        } else {
            right = index;
        }
    }
    index
}
#[no_mangle]
pub extern "C" fn sortedarray_get(
    mut array: *mut SortedArray,
    mut i: libc::c_uint,
) -> *mut SortedArrayValue {
    if array.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        return *((*array).data).offset(i as isize) as *mut SortedArrayValue;
    }
}
#[no_mangle]
pub extern "C" fn sortedarray_length(
    array: *mut SortedArray,
) -> libc::c_uint {
    unsafe {
        (*array).length
    }
}
#[no_mangle]
pub extern "C" fn sortedarray_new(
    mut length: libc::c_uint,
    mut equ_func: SortedArrayEqualFunc,
    mut cmp_func: SortedArrayCompareFunc,
) -> *mut SortedArray {
    if equ_func.is_none() || cmp_func.is_none() {
        return std::ptr::null_mut();
    }
    if length == 0 {
        length = 16;
    }
    let array: *mut SortedArrayValue = unsafe {
        alloc_test_malloc(
            (std::mem::size_of::<SortedArrayValue>() as libc::c_ulong)
                .wrapping_mul(length as libc::c_ulong),
        ) as *mut SortedArrayValue
    };
    if array.is_null() {
        return std::ptr::null_mut();
    }
    let sortedarray: *mut SortedArray = unsafe {
        alloc_test_malloc(std::mem::size_of::<SortedArray>() as libc::c_ulong) as *mut SortedArray
    };
    if sortedarray.is_null() {
        unsafe { alloc_test_free(array as *mut libc::c_void) };
        return std::ptr::null_mut();
    }
    unsafe {
        (*sortedarray).data = array;
        (*sortedarray).length = 0;
        (*sortedarray)._alloced = length;
        (*sortedarray).equ_func = equ_func;
        (*sortedarray).cmp_func = cmp_func;
    }
    sortedarray
}
#[no_mangle]
pub extern "C" fn sortedarray_free(mut sortedarray: *mut SortedArray) {
    if !sortedarray.is_null() {
        unsafe {
            alloc_test_free((*sortedarray).data as *mut libc::c_void);
            alloc_test_free(sortedarray as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub extern "C" fn sortedarray_remove_range(
    mut sortedarray: *mut SortedArray,
    mut index: libc::c_uint,
    mut length: libc::c_uint,
) {
    if index > unsafe { (*sortedarray).length }
        || index.wrapping_add(length) > unsafe { (*sortedarray).length }
    {
        return;
    }
    unsafe {
        memmove(
            &mut *((*sortedarray).data).offset(index as isize) as *mut SortedArrayValue as *mut libc::c_void,
            &mut *((*sortedarray).data).offset(index.wrapping_add(length) as isize) as *mut SortedArrayValue as *const libc::c_void,
            ((*sortedarray).length).wrapping_sub(index.wrapping_add(length)) as libc::c_ulong * std::mem::size_of::<SortedArrayValue>() as libc::c_ulong,
        );
        (*sortedarray).length = (*sortedarray).length.wrapping_sub(length);
    }
}
#[no_mangle]
pub extern "C" fn sortedarray_remove(
    mut sortedarray: *mut SortedArray,
    mut index: libc::c_uint,
) {
    sortedarray_remove_range(sortedarray, index, 1);
}
#[no_mangle]
pub extern "C" fn sortedarray_insert(
    mut sortedarray: *mut SortedArray,
    mut data: SortedArrayValue,
) -> libc::c_int {
    let mut left: libc::c_uint = 0;
    let mut right: libc::c_uint = unsafe { (*sortedarray).length };
    let mut index: libc::c_uint = 0;
    right = if right > 1 { right } else { 0 };
    while left != right {
        index = left.wrapping_add(right).wrapping_div(2);
        let order: libc::c_int = unsafe {
            ((*sortedarray).cmp_func).expect("non-null function pointer")(
                data,
                *((*sortedarray).data).offset(index as isize),
            )
        };
        if order < 0 {
            right = index;
        } else {
            if order > 0 {
                left = index.wrapping_add(1);
            } else {
                break;
            }
        }
    }
    if unsafe { (*sortedarray).length > 0 }
        && unsafe {
            ((*sortedarray).cmp_func).expect("non-null function pointer")(
                data,
                *((*sortedarray).data).offset(index as isize),
            ) > 0
        }
    {
        index = index.wrapping_add(1);
    }
    if unsafe { (*sortedarray).length.wrapping_add(1) > (*sortedarray)._alloced } {
        let newsize: libc::c_uint = unsafe { (*sortedarray)._alloced.wrapping_mul(2) };
        let data_0: *mut SortedArrayValue = unsafe {
            alloc_test_realloc(
                (*sortedarray).data as *mut libc::c_void,
                (std::mem::size_of::<SortedArrayValue>() as libc::c_ulong)
                    .wrapping_mul(newsize as libc::c_ulong),
            ) as *mut SortedArrayValue
        };
        if data_0.is_null() {
            return 0;
        } else {
            unsafe {
                (*sortedarray).data = data_0;
                (*sortedarray)._alloced = newsize;
            }
        }
    }
    unsafe {
        memmove(
            &mut *((*sortedarray).data)
                .offset(index.wrapping_add(1) as isize)
                as *mut SortedArrayValue as *mut libc::c_void,
            &mut *((*sortedarray).data).offset(index as isize) as *mut SortedArrayValue
                as *const libc::c_void,
            ((*sortedarray).length.wrapping_sub(index) as libc::c_ulong)
                .wrapping_mul(std::mem::size_of::<SortedArrayValue>() as libc::c_ulong),
        );
        let ref mut fresh0 = *((*sortedarray).data).offset(index as isize);
        *fresh0 = data;
        (*sortedarray).length = (*sortedarray).length.wrapping_add(1);
    }
    return 1;
}
#[no_mangle]
pub extern "C" fn sortedarray_index_of(
    mut sortedarray: *mut SortedArray,
    mut data: SortedArrayValue,
) -> libc::c_int {
    if sortedarray.is_null() {
        return -(1 as libc::c_int);
    }
    let mut left: libc::c_uint = 0;
    let mut right: libc::c_uint = unsafe { (*sortedarray).length };
    let mut index: libc::c_uint = 0;
    right = if right > 1 { right } else { 0 };
    while left != right {
        index = left.wrapping_add(right).wrapping_div(2);
        let order: libc::c_int = unsafe {
            ((*sortedarray).cmp_func).expect("non-null function pointer")(
                data,
                *((*sortedarray).data).offset(index as isize),
            )
        };
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = index.wrapping_add(1);
        } else {
            left = unsafe { sortedarray_first_index(sortedarray, data, left, index) };
            right = unsafe { sortedarray_last_index(sortedarray, data, index, right) };
            index = left;
            while index <= right {
                if unsafe {
                    ((*sortedarray).equ_func).expect("non-null function pointer")(
                        data,
                        *((*sortedarray).data).offset(index as isize),
                    ) != 0
                } {
                    return index as libc::c_int;
                }
                index = index.wrapping_add(1);
            }
            return -(1 as libc::c_int);
        }
    }
    -(1 as libc::c_int)
}
#[no_mangle]
pub extern "C" fn sortedarray_clear(mut sortedarray: *mut SortedArray) {
    unsafe {
        (*sortedarray).length = 0 as libc::c_int as libc::c_uint;
    }
}
