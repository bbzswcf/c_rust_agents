#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _SortedArray;
    fn rand() -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn sortedarray_get(
        array: *mut SortedArray,
        i: libc::c_uint,
    ) -> *mut SortedArrayValue;
    fn sortedarray_length(array: *mut SortedArray) -> libc::c_uint;
    fn sortedarray_new(
        length: libc::c_uint,
        equ_func: SortedArrayEqualFunc,
        cmp_func: SortedArrayCompareFunc,
    ) -> *mut SortedArray;
    fn sortedarray_free(sortedarray: *mut SortedArray);
    fn sortedarray_remove(sortedarray: *mut SortedArray, index: libc::c_uint);
    fn sortedarray_remove_range(
        sortedarray: *mut SortedArray,
        index: libc::c_uint,
        length: libc::c_uint,
    );
    fn sortedarray_insert(
        sortedarray: *mut SortedArray,
        data: SortedArrayValue,
    ) -> libc::c_int;
    fn sortedarray_index_of(
        sortedarray: *mut SortedArray,
        data: SortedArrayValue,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type SortedArrayValue = *mut libc::c_void;
pub type SortedArray = _SortedArray;
pub type SortedArrayEqualFunc = Option::<
    unsafe extern "C" fn(SortedArrayValue, SortedArrayValue) -> libc::c_int,
>;
pub type SortedArrayCompareFunc = Option::<
    unsafe extern "C" fn(SortedArrayValue, SortedArrayValue) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn check_sorted_prop(mut sortedarray: *mut SortedArray) {
    let mut i: libc::c_uint = 0;
    i = 1 as libc::c_int as libc::c_uint;
    while i < sortedarray_length(sortedarray) {
        if int_compare(
            sortedarray_get(
                sortedarray,
                i.wrapping_sub(1 as libc::c_int as libc::c_uint),
            ) as *mut libc::c_void,
            sortedarray_get(sortedarray, i) as *mut libc::c_void,
        ) <= 0 as libc::c_int
        {} else {
            __assert_fail(
                b"int_compare( sortedarray_get(sortedarray, i-1), sortedarray_get(sortedarray, i)) <= 0\0"
                    as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void check_sorted_prop(SortedArray *)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn free_sorted_ints(mut sortedarray: *mut SortedArray) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < sortedarray_length(sortedarray) {
        let mut pi: *mut libc::c_int = sortedarray_get(sortedarray, i)
            as *mut libc::c_int;
        alloc_test_free(pi as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    sortedarray_free(sortedarray);
}
#[no_mangle]
pub unsafe extern "C" fn generate_sortedarray_equ(
    mut equ_func: SortedArrayEqualFunc,
) -> *mut SortedArray {
    let mut sortedarray: *mut SortedArray = 0 as *mut SortedArray;
    let mut i: libc::c_uint = 0;
    let mut array: [libc::c_int; 20] = [
        10 as libc::c_int,
        12 as libc::c_int,
        12 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        2 as libc::c_int,
        23 as libc::c_int,
        13 as libc::c_int,
        23 as libc::c_int,
        23 as libc::c_int,
        34 as libc::c_int,
        31 as libc::c_int,
        9 as libc::c_int,
        21 as libc::c_int,
        -(2 as libc::c_int),
        -(12 as libc::c_int),
        -(4 as libc::c_int),
    ];
    sortedarray = sortedarray_new(
        0 as libc::c_int as libc::c_uint,
        equ_func,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 20 as libc::c_int as libc::c_uint {
        let mut pi: *mut libc::c_int = alloc_test_malloc(
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        *pi = array[i as usize];
        sortedarray_insert(sortedarray, pi as SortedArrayValue);
        i = i.wrapping_add(1);
        i;
    }
    return sortedarray;
}
#[no_mangle]
pub unsafe extern "C" fn generate_sortedarray() -> *mut SortedArray {
    return generate_sortedarray_equ(
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_new_free() {
    let mut sortedarray: *mut SortedArray = 0 as *mut SortedArray;
    sortedarray = sortedarray_new(
        0 as libc::c_int as libc::c_uint,
        Some(
            int_equal
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    if !sortedarray.is_null() {} else {
        __assert_fail(
            b"sortedarray != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 37],
                &[libc::c_char; 37],
            >(b"void test_sortedarray_new_free(void)\0"))
                .as_ptr(),
        );
    };
    sortedarray_free(sortedarray);
    sortedarray_free(0 as *mut SortedArray);
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_insert() {
    let mut sortedarray: *mut SortedArray = generate_sortedarray();
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 20 as libc::c_int as libc::c_uint {
        let mut i_0: libc::c_int = (rand() as libc::c_float
            / 2147483647 as libc::c_int as libc::c_float
            * 100 as libc::c_int as libc::c_float) as libc::c_int;
        let mut pi: *mut libc::c_int = alloc_test_malloc(
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        *pi = i_0;
        sortedarray_insert(sortedarray, pi as SortedArrayValue);
        i = i.wrapping_add(1);
        i;
    }
    check_sorted_prop(sortedarray);
    free_sorted_ints(sortedarray);
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_remove() {
    let mut sortedarray: *mut SortedArray = generate_sortedarray();
    let mut ip: *mut libc::c_int = sortedarray_get(
        sortedarray,
        (15 as libc::c_int + 1 as libc::c_int) as libc::c_uint,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = *ip;
    alloc_test_free(
        sortedarray_get(sortedarray, 15 as libc::c_int as libc::c_uint)
            as *mut libc::c_int as *mut libc::c_void,
    );
    sortedarray_remove(sortedarray, 15 as libc::c_int as libc::c_uint);
    if *(sortedarray_get(sortedarray, 15 as libc::c_int as libc::c_uint)
        as *mut libc::c_int) == i
    {} else {
        __assert_fail(
            b"*((int*) sortedarray_get(sortedarray, TEST_REMOVE_EL)) == i\0" as *const u8
                as *const libc::c_char,
            b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_sortedarray_remove(void)\0"))
                .as_ptr(),
        );
    };
    check_sorted_prop(sortedarray);
    free_sorted_ints(sortedarray);
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_remove_range() {
    let mut sortedarray: *mut SortedArray = generate_sortedarray();
    let mut new: [libc::c_int; 4] = [0; 4];
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        new[i
            as usize] = *(sortedarray_get(
            sortedarray,
            ((7 as libc::c_int + 4 as libc::c_int) as libc::c_uint).wrapping_add(i),
        ) as *mut libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        alloc_test_free(
            sortedarray_get(
                sortedarray,
                (7 as libc::c_int as libc::c_uint).wrapping_add(i),
            ) as *mut libc::c_int as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    sortedarray_remove_range(
        sortedarray,
        7 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 4 as libc::c_int as libc::c_uint {
        if *(sortedarray_get(
            sortedarray,
            (7 as libc::c_int as libc::c_uint).wrapping_add(i),
        ) as *mut libc::c_int) == new[i as usize]
        {} else {
            __assert_fail(
                b"*((int*) sortedarray_get(sortedarray, TEST_REMOVE_RANGE + i)) == new[i]\0"
                    as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                161 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[libc::c_char; 41],
                >(b"void test_sortedarray_remove_range(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    check_sorted_prop(sortedarray);
    free_sorted_ints(sortedarray);
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_index_of() {
    let mut sortedarray: *mut SortedArray = generate_sortedarray();
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 20 as libc::c_int as libc::c_uint {
        let mut r: libc::c_int = sortedarray_index_of(
            sortedarray,
            sortedarray_get(sortedarray, i) as SortedArrayValue,
        );
        if r >= 0 as libc::c_int {} else {
            __assert_fail(
                b"r >= 0\0" as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_sortedarray_index_of(void)\0"))
                    .as_ptr(),
            );
        };
        if *(sortedarray_get(sortedarray, r as libc::c_uint) as *mut libc::c_int)
            == *(sortedarray_get(sortedarray, i) as *mut libc::c_int)
        {} else {
            __assert_fail(
                b"*((int*) sortedarray_get(sortedarray,(unsigned int) r)) == *((int*) sortedarray_get(sortedarray, i))\0"
                    as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                177 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 37],
                    &[libc::c_char; 37],
                >(b"void test_sortedarray_index_of(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    free_sorted_ints(sortedarray);
}
unsafe extern "C" fn ptr_equal(
    mut v1: SortedArrayValue,
    mut v2: SortedArrayValue,
) -> libc::c_int {
    return (v1 == v2) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_index_of_equ_key() {
    let mut sortedarray: *mut SortedArray = generate_sortedarray_equ(
        Some(
            ptr_equal
                as unsafe extern "C" fn(
                    SortedArrayValue,
                    SortedArrayValue,
                ) -> libc::c_int,
        ),
    );
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 20 as libc::c_int as libc::c_uint {
        let mut r: libc::c_int = sortedarray_index_of(
            sortedarray,
            sortedarray_get(sortedarray, i) as SortedArrayValue,
        );
        if r >= 0 as libc::c_int {} else {
            __assert_fail(
                b"r >= 0\0" as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                197 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void test_sortedarray_index_of_equ_key(void)\0"))
                    .as_ptr(),
            );
        };
        if i == r as libc::c_uint {} else {
            __assert_fail(
                b"i == (unsigned int) r\0" as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                198 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[libc::c_char; 45],
                >(b"void test_sortedarray_index_of_equ_key(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    free_sorted_ints(sortedarray);
}
#[no_mangle]
pub unsafe extern "C" fn test_sortedarray_get() {
    let mut i: libc::c_uint = 0;
    let mut arr: *mut SortedArray = generate_sortedarray();
    i = 0 as libc::c_int as libc::c_uint;
    while i < sortedarray_length(arr) {
        if sortedarray_get(arr, i) == sortedarray_get(arr, i) {} else {
            __assert_fail(
                b"sortedarray_get(arr, i) == sortedarray_get(arr, i)\0" as *const u8
                    as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                210 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_sortedarray_get(void)\0"))
                    .as_ptr(),
            );
        };
        if *(sortedarray_get(arr, i) as *mut libc::c_int)
            == *(sortedarray_get(arr, i) as *mut libc::c_int)
        {} else {
            __assert_fail(
                b"*((int*) sortedarray_get(arr, i)) == *((int*) sortedarray_get(arr, i))\0"
                    as *const u8 as *const libc::c_char,
                b"test/test-sortedarray.c\0" as *const u8 as *const libc::c_char,
                212 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_sortedarray_get(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    free_sorted_ints(arr);
}
static mut tests: [UnitTestFunction; 8] = unsafe {
    [
        Some(test_sortedarray_new_free as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_insert as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_remove as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_remove_range as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_index_of as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_index_of_equ_key as unsafe extern "C" fn() -> ()),
        Some(test_sortedarray_get as unsafe extern "C" fn() -> ()),
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
