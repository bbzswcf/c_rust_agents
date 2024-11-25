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
unsafe extern "C" fn alloc_test_get_header(
    mut ptr: *mut libc::c_void,
) -> *mut BlockHeader {
    let mut result: *mut BlockHeader = 0 as *mut BlockHeader;
    result = (ptr as *mut BlockHeader).offset(-(1 as libc::c_int as isize));
    if (*result).magic_number == 0x72ec82d2 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"result->magic_number == ALLOC_TEST_MAGIC\0" as *const u8
                as *const libc::c_char,
            b"test/alloc-testing.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[libc::c_char; 43],
            >(b"BlockHeader *alloc_test_get_header(void *)\0"))
                .as_ptr(),
        );
    };
    return result;
}
unsafe extern "C" fn alloc_test_overwrite(
    mut ptr: *mut libc::c_void,
    mut length: size_t,
    mut pattern: libc::c_uint,
) {
    let mut byte_ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pattern_seq: libc::c_int = 0;
    let mut b: libc::c_uchar = 0;
    let mut i: size_t = 0;
    byte_ptr = ptr as *mut libc::c_uchar;
    i = 0 as libc::c_int as size_t;
    while i < length {
        pattern_seq = (i & 3 as libc::c_int as size_t) as libc::c_int;
        b = (pattern >> 8 as libc::c_int * pattern_seq
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *byte_ptr.offset(i as isize) = b;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_malloc(mut bytes: size_t) -> *mut libc::c_void {
    let mut header: *mut BlockHeader = 0 as *mut BlockHeader;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if allocation_limit == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    header = malloc(
        (::core::mem::size_of::<BlockHeader>() as libc::c_ulong).wrapping_add(bytes),
    ) as *mut BlockHeader;
    if header.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*header).magic_number = 0x72ec82d2 as libc::c_int as libc::c_uint;
    (*header).bytes = bytes;
    ptr = header.offset(1 as libc::c_int as isize) as *mut libc::c_void;
    alloc_test_overwrite(ptr, bytes, 0xbaadf00d as libc::c_uint);
    allocated_bytes = allocated_bytes.wrapping_add(bytes);
    if allocation_limit > 0 as libc::c_int {
        allocation_limit -= 1;
        allocation_limit;
    }
    return header.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_free(mut ptr: *mut libc::c_void) {
    let mut header: *mut BlockHeader = 0 as *mut BlockHeader;
    let mut block_size: size_t = 0;
    if ptr.is_null() {
        return;
    }
    header = alloc_test_get_header(ptr);
    block_size = (*header).bytes;
    if allocated_bytes >= block_size {} else {
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
    };
    alloc_test_overwrite(ptr, (*header).bytes, 0xdeadbeef as libc::c_uint);
    (*header).magic_number = 0 as libc::c_int as libc::c_uint;
    free(header as *mut libc::c_void);
    allocated_bytes = allocated_bytes.wrapping_sub(block_size);
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_realloc(
    mut ptr: *mut libc::c_void,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut header: *mut BlockHeader = 0 as *mut BlockHeader;
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bytes_to_copy: size_t = 0;
    new_ptr = alloc_test_malloc(bytes);
    if new_ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    if !ptr.is_null() {
        header = alloc_test_get_header(ptr);
        bytes_to_copy = (*header).bytes;
        if bytes_to_copy > bytes {
            bytes_to_copy = bytes;
        }
        memcpy(new_ptr, ptr, bytes_to_copy);
        alloc_test_free(ptr);
    }
    return new_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_calloc(
    mut nmemb: size_t,
    mut bytes: size_t,
) -> *mut libc::c_void {
    let mut result: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut total_bytes: size_t = nmemb * bytes;
    result = alloc_test_malloc(total_bytes);
    if result.is_null() {
        return 0 as *mut libc::c_void;
    }
    memset(result, 0 as libc::c_int, total_bytes);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_strdup(
    mut string: *const libc::c_char,
) -> *mut libc::c_char {
    let mut result: *mut libc::c_char = 0 as *mut libc::c_char;
    result = alloc_test_malloc(
        (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if result.is_null() {
        return 0 as *mut libc::c_char;
    }
    strcpy(result, string);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_set_limit(mut alloc_count: libc::c_int) {
    allocation_limit = alloc_count;
}
#[no_mangle]
pub unsafe extern "C" fn alloc_test_get_allocated() -> size_t {
    return allocated_bytes;
}
