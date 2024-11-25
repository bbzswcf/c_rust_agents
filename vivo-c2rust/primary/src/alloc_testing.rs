#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type size_t = libc::c_ulong;
pub type BlockHeader = _BlockHeader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _BlockHeader {
    pub magic_number: libc::c_uint,
    pub bytes: size_t,
}
static mut allocated_bytes: size_t = 0 as libc::c_int as size_t;
#[no_mangle]
pub static mut allocation_limit: libc::c_int = -(1 as libc::c_int);
extern "C" fn alloc_test_get_header(mut ptr: *mut libc::c_void) -> *mut BlockHeader {
    let result: *mut BlockHeader;
    unsafe {
        result = (ptr as *mut BlockHeader).offset(-(1 as libc::c_int as isize));
        if (*result).magic_number != 0x72ec82d2 as libc::c_int as libc::c_uint {
            __assert_fail(
                b"result->magic_number == ALLOC_TEST_MAGIC\0" as *const u8 as *const libc::c_char,
                b"test/alloc-testing.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[libc::c_char; 43],
                >(b"BlockHeader *alloc_test_get_header(void *)\0")).as_ptr(),
            );
        }
    }
    result
}
extern "C" fn alloc_test_overwrite(
    mut ptr: *mut libc::c_void,
    mut length: size_t,
    mut pattern: libc::c_uint,
) {
    let mut byte_ptr: *mut libc::c_uchar = ptr as *mut libc::c_uchar;
    let mut i: size_t = 0;
    while i < length {
        let pattern_seq = (i & 3) as libc::c_int;
        let b = ((pattern >> (8 * pattern_seq)) & 0xff) as libc::c_uchar;
        unsafe {
            *byte_ptr.offset(i as isize) = b;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub extern "C" fn alloc_test_malloc(mut bytes: size_t) -> *mut libc::c_void {
    if unsafe { allocation_limit == 0 } {
        return std::ptr::null_mut();
    }
    let header_size = std::mem::size_of::<BlockHeader>() as libc::c_ulong;
    let total_size = header_size.wrapping_add(bytes);
    let header = unsafe { malloc(total_size) as *mut BlockHeader };
    if header.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        (*header).magic_number = 0x72ec82d2 as libc::c_uint;
        (*header).bytes = bytes;
    }
    let ptr = unsafe { header.offset(1) as *mut libc::c_void };
    alloc_test_overwrite(ptr, bytes, 0xbaadf00d as libc::c_uint);
    unsafe {
        allocated_bytes = allocated_bytes.wrapping_add(bytes);
        if allocation_limit > 0 {
            allocation_limit -= 1;
        }
    }
    ptr
}
#[no_mangle]
pub extern "C" fn alloc_test_free(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let header = alloc_test_get_header(ptr);
    let block_size = unsafe { (*header).bytes };
    unsafe {
        if allocated_bytes >= block_size {
            alloc_test_overwrite(ptr, block_size, 0xdeadbeef as libc::c_uint);
            (*header).magic_number = 0 as libc::c_int as libc::c_uint;
            free(header as *mut libc::c_void);
            allocated_bytes = allocated_bytes.wrapping_sub(block_size);
        } else {
            __assert_fail(
                b"allocated_bytes >= block_size\0" as *const u8 as *const libc::c_char,
                b"test/alloc-testing.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 29],
                    &[libc::c_char; 29],
                >(b"void alloc_test_free(void *)\0"))
                    .as_ptr(),
            );
        }
    }
}
#[no_mangle]
pub extern "C" fn alloc_test_realloc(
    mut ptr: *mut libc::c_void,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut header: *mut BlockHeader = std::ptr::null_mut();
    let mut new_ptr: *mut libc::c_void = std::ptr::null_mut();
    let mut bytes_to_copy: size_t = 0;

    new_ptr = alloc_test_malloc(bytes);
    if new_ptr.is_null() {
        return std::ptr::null_mut();
    }

    if !ptr.is_null() {
        unsafe {
            header = alloc_test_get_header(ptr);
            bytes_to_copy = (*header).bytes;
            if bytes_to_copy > bytes {
                bytes_to_copy = bytes;
            }
            memcpy(new_ptr, ptr, bytes_to_copy);
            alloc_test_free(ptr);
        }
    }

    new_ptr
}
#[no_mangle]
pub extern "C" fn alloc_test_calloc(
    mut nmemb: size_t,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = std::ptr::null_mut();
    let mut total_bytes: size_t = nmemb * bytes;
    result = alloc_test_malloc(total_bytes);
    if result.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        memset(result, 0, total_bytes);
    }
    result
}
#[no_mangle]
pub extern "C" fn alloc_test_strdup(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = std::ptr::null_mut();
    let len = unsafe { strlen(string) };
    result = alloc_test_malloc(len.wrapping_add(1)) as *mut libc::c_char;
    if result.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { strcpy(result, string) };
    result
}
#[no_mangle]
pub extern "C" fn alloc_test_set_limit(mut alloc_count: libc::c_int) {
    unsafe {
        allocation_limit = alloc_count;
    }
}
#[no_mangle]
pub extern "C" fn alloc_test_get_allocated() -> size_t {
    unsafe { allocated_bytes }
}
