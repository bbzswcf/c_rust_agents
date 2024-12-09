import os
TMP_EXTRACT_FILE_PATH = "Tool/test_extract_tmp.rs"
TMP_REMOVE_FILE_PATH = "Tool/test_remove_tmp.rs"
EMPTY_PATH = "Tool/test_not_exist.rs"


test_extract_content = """
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _BinomialHeap;
    fn binomial_heap_new(
        heap_type: BinomialHeapType,
        compare_func: BinomialHeapCompareFunc,
    ) -> *mut BinomialHeap;
    fn binomial_heap_free(heap: *mut BinomialHeap);
    fn binomial_heap_insert(
        heap: *mut BinomialHeap,
        value: BinomialHeapValue,
    ) -> libc::c_int;
}
pub struct _ArrayList {
    pub data: *mut ArrayListValue,
    pub length: libc::c_uint,
    pub _alloced: libc::c_uint,
}
pub type BinomialHeapType = libc::c_uint;
pub static mut test_array: [libc::c_int; 10000] = [0; 10000];
pub type BinomialHeapCompareFunc = Option::<
    unsafe extern "C" fn(BinomialHeapValue, BinomialHeapValue) -> libc::c_int,
>;
pub static mut variable1: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn test_binomial_heap_new_free() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10000 as libc::c_int {
        heap = binomial_heap_new(
            BINOMIAL_HEAP_TYPE_MIN,
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        binomial_heap_free(heap);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn test_pop_out_of_memory() {
    let mut heap: *mut BinomialHeap = 0 as *mut BinomialHeap;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        heap = generate_heap();
        i += 1;
        i;
    }
}
"""

test_remove_content = """
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn arraylist_new(length: libc::c_uint) -> *mut ArrayList;
    fn arraylist_free(arraylist: *mut ArrayList);
    fn arraylist_append(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_prepend(arraylist: *mut ArrayList, data: ArrayListValue) -> libc::c_int;
    fn arraylist_remove(arraylist: *mut ArrayList, index: libc::c_uint);
    fn arraylist_remove_range(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        length: libc::c_uint,
    );
    fn arraylist_insert(
        arraylist: *mut ArrayList,
        index: libc::c_uint,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_index_of(
        arraylist: *mut ArrayList,
        callback: ArrayListEqualFunc,
        data: ArrayListValue,
    ) -> libc::c_int;
    fn arraylist_clear(arraylist: *mut ArrayList);
    fn arraylist_sort(arraylist: *mut ArrayList, compare_func: ArrayListCompareFunc);
    fn int_equal(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type ArrayListValue = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ArrayList {
    pub data: *mut ArrayListValue,
    pub length: libc::c_uint,
    pub _alloced: libc::c_uint,
}
pub type ArrayList = _ArrayList;
pub type ArrayListEqualFunc = Option::<
    unsafe extern "C" fn(ArrayListValue, ArrayListValue) -> libc::c_int,
>;
pub type ArrayListCompareFunc = Option::<
    unsafe extern "C" fn(ArrayListValue, ArrayListValue) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn generate_arraylist() -> *mut ArrayList {
    let mut arraylist: *mut ArrayList = 0 as *mut ArrayList;
    let mut i: libc::c_int = 0;
    arraylist = arraylist_new(0 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        arraylist_append(
            arraylist,
            &mut variable1 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable2 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable3 as *mut libc::c_int as ArrayListValue,
        );
        arraylist_append(
            arraylist,
            &mut variable4 as *mut libc::c_int as ArrayListValue,
        );
        i += 1;
        i;
    }
    return arraylist;
}
static mut tests: [UnitTestFunction; 10] = unsafe {
    [
        Some(test_arraylist_new_free as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_append as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_prepend as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_insert as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_remove as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_remove_range as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_index_of as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_clear as unsafe extern "C" fn() -> ()),
        Some(test_arraylist_sort as unsafe extern "C" fn() -> ()),
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
"""


def pytest_sessionstart(session):
    print(f"{'-'*40}\nCreating files for test before tests\n{'-'*40}")
    with open(TMP_EXTRACT_FILE_PATH, "w") as file:
        file.write(test_extract_content)
    with open(TMP_REMOVE_FILE_PATH, "w") as file:
        file.write(test_remove_content)

def pytest_sessionfinish(session, exitstatus):
    print(f"{'-'*40}\nRemoving files for test after tests\n{'-'*40}")
    os.remove(TMP_EXTRACT_FILE_PATH)
    os.remove(TMP_REMOVE_FILE_PATH)