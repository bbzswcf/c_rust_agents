#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern crate libc;
use primary::binary_heap::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    pub type _BinaryHeap;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn binary_heap_new(
        heap_type: BinaryHeapType,
        compare_func: BinaryHeapCompareFunc,
    ) -> *mut BinaryHeap;
    fn binary_heap_free(heap: *mut BinaryHeap);
    fn binary_heap_insert(heap: *mut BinaryHeap, value: BinaryHeapValue) -> libc::c_int;
    fn binary_heap_pop(heap: *mut BinaryHeap) -> BinaryHeapValue;
    fn binary_heap_num_entries(heap: *mut BinaryHeap) -> libc::c_uint;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type BinaryHeapType = libc::c_uint;
pub const BINARY_HEAP_TYPE_MAX: BinaryHeapType = 1;
pub const BINARY_HEAP_TYPE_MIN: BinaryHeapType = 0;
pub type BinaryHeapValue = *mut libc::c_void;
pub type BinaryHeapCompareFunc = Option::<
    unsafe extern "C" fn(BinaryHeapValue, BinaryHeapValue) -> libc::c_int,
>;
pub type BinaryHeap = _BinaryHeap;
#[no_mangle]
pub static mut test_array: [libc::c_int; 10000] = [0; 10000];
#[no_mangle]
pub unsafe extern "C" fn test_binary_heap_new_free() {
    let mut heap: *mut BinaryHeap = 0 as *mut BinaryHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        heap = binary_heap_new(
            BINARY_HEAP_TYPE_MIN,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        binary_heap_free(heap);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_binary_heap_insert() {
    let mut heap: *mut BinaryHeap = 0 as *mut BinaryHeap;
    let mut i: libc::c_int = 0;
    heap = binary_heap_new(
        BINARY_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binary_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinaryHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binary_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[libc::c_char; 35],
                >(b"void test_binary_heap_insert(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    if binary_heap_num_entries(heap) == 10000 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"binary_heap_num_entries(heap) == NUM_TEST_VALUES\0" as *const u8
                as *const libc::c_char,
            b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 35],
                &[libc::c_char; 35],
            >(b"void test_binary_heap_insert(void)\0"))
                .as_ptr(),
        );
    };
    binary_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn test_min_heap() {
    let mut heap: *mut BinaryHeap = 0 as *mut BinaryHeap;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    heap = binary_heap_new(
        BINARY_HEAP_TYPE_MIN,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binary_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinaryHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binary_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_min_heap(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = -(1 as libc::c_int);
    while binary_heap_num_entries(heap) > 0 as libc::c_int as libc::c_uint {
        val = binary_heap_pop(heap) as *mut libc::c_int;
        if *val == i + 1 as libc::c_int {} else {
            __assert_fail(
                b"*val == i + 1\0" as *const u8 as *const libc::c_char,
                b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_min_heap(void)\0"))
                    .as_ptr(),
            );
        };
        i = *val;
    }
    if binary_heap_num_entries(heap) == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"binary_heap_num_entries(heap) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_min_heap(void)\0"))
                .as_ptr(),
        );
    };
    if (binary_heap_pop(heap)).is_null() {} else {
        __assert_fail(
            b"binary_heap_pop(heap) == BINARY_HEAP_NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 25],
                &[libc::c_char; 25],
            >(b"void test_min_heap(void)\0"))
                .as_ptr(),
        );
    };
    binary_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn test_max_heap() {
    let mut heap: *mut BinaryHeap = 0 as *mut BinaryHeap;
    let mut val: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    heap = binary_heap_new(
        BINARY_HEAP_TYPE_MAX,
        Some(
            int_compare
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        test_array[i as usize] = i;
        if binary_heap_insert(
            heap,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as BinaryHeapValue,
        ) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"binary_heap_insert(heap, &test_array[i]) != 0\0" as *const u8
                    as *const libc::c_char,
                b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
                117 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_max_heap(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = 10000 as libc::c_int;
    while binary_heap_num_entries(heap) > 0 as libc::c_int as libc::c_uint {
        val = binary_heap_pop(heap) as *mut libc::c_int;
        if *val == i - 1 as libc::c_int {} else {
            __assert_fail(
                b"*val == i - 1\0" as *const u8 as *const libc::c_char,
                b"test/test-binary-heap.c\0" as *const u8 as *const libc::c_char,
                126 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 25],
                    &[libc::c_char; 25],
                >(b"void test_max_heap(void)\0"))
                    .as_ptr(),
            );
        };
        i = *val;
    }
    binary_heap_free(heap);
}
#[no_mangle]
pub unsafe extern "C" fn test_out_of_memory() {}
static mut tests: [UnitTestFunction; 6] = unsafe {
    [
        Some(test_binary_heap_new_free as unsafe extern "C" fn() -> ()),
        Some(test_binary_heap_insert as unsafe extern "C" fn() -> ()),
        Some(test_min_heap as unsafe extern "C" fn() -> ()),
        Some(test_max_heap as unsafe extern "C" fn() -> ()),
        Some(test_out_of_memory as unsafe extern "C" fn() -> ()),
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








