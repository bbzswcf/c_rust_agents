#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, label_break_value)]
extern "C" {
    pub type _Queue;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn queue_new() -> *mut Queue;
    fn queue_free(queue: *mut Queue);
    fn queue_push_head(queue: *mut Queue, data: QueueValue) -> libc::c_int;
    fn queue_pop_head(queue: *mut Queue) -> QueueValue;
    fn queue_peek_head(queue: *mut Queue) -> QueueValue;
    fn queue_push_tail(queue: *mut Queue, data: QueueValue) -> libc::c_int;
    fn queue_pop_tail(queue: *mut Queue) -> QueueValue;
    fn queue_peek_tail(queue: *mut Queue) -> QueueValue;
    fn queue_is_empty(queue: *mut Queue) -> libc::c_int;
}
pub type Queue = _Queue;
pub type QueueValue = *mut libc::c_void;
#[no_mangle]
pub static mut variable1: libc::c_int = 0;
#[no_mangle]
pub static mut variable2: libc::c_int = 0;
#[no_mangle]
pub static mut variable3: libc::c_int = 0;
#[no_mangle]
pub static mut variable4: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn generate_queue() -> *mut Queue {
    let mut queue: *mut Queue = 0 as *mut Queue;
    let mut i: libc::c_int = 0;
    queue = queue_new();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        queue_push_head(queue, &mut variable1 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable2 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable3 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable4 as *mut libc::c_int as QueueValue);
        i += 1;
        i;
    }
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_new_free() {
    let mut i: libc::c_int = 0;
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    queue_free(queue);
    queue = queue_new();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        queue_push_head(queue, &mut variable1 as *mut libc::c_int as QueueValue);
        i += 1;
        i;
    }
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_push_head() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    let mut i: libc::c_int = 0;
    queue = queue_new();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        queue_push_head(queue, &mut variable1 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable2 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable3 as *mut libc::c_int as QueueValue);
        queue_push_head(queue, &mut variable4 as *mut libc::c_int as QueueValue);
        i += 1;
        i;
    }
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2129: {
        if queue_is_empty(queue) == 0 {} else {
            __assert_fail(
                b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2083: {
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                68 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2037: {
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                69 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1991: {
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1945: {
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            75 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1899: {
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                75 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1853: {
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            77 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1807: {
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                77 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_1756: {
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                78 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = queue_new();
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_pop_head() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_pop_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2493: {
        if (queue_pop_head(queue)).is_null() {} else {
            __assert_fail(
                b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                104 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2432: {
            if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    104 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2386: {
            if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    105 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                106 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2340: {
            if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    106 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                107 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2294: {
            if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    107 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if (queue_pop_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2240: {
        if (queue_pop_head(queue)).is_null() {} else {
            __assert_fail(
                b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_peek_head() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_peek_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_3000: {
        if (queue_peek_head(queue)).is_null() {} else {
            __assert_fail(
                b"queue_peek_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                122 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_peek_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2939: {
            if queue_peek_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_head(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    132 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                133 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2893: {
            if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    133 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                134 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2847: {
            if queue_peek_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_head(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    134 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2801: {
            if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    135 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                136 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2755: {
            if queue_peek_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_head(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    136 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                137 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2709: {
            if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    137 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                138 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2662: {
            if queue_peek_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_head(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    138 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                139 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
        'c_2616: {
            if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_head(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    139 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_head(void)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if (queue_peek_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_head(void)\0"))
                .as_ptr(),
        );
    }
    'c_2560: {
        if (queue_peek_head(queue)).is_null() {} else {
            __assert_fail(
                b"queue_peek_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_push_tail() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    let mut i: libc::c_int = 0;
    queue = queue_new();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        queue_push_tail(queue, &mut variable1 as *mut libc::c_int as QueueValue);
        queue_push_tail(queue, &mut variable2 as *mut libc::c_int as QueueValue);
        queue_push_tail(queue, &mut variable3 as *mut libc::c_int as QueueValue);
        queue_push_tail(queue, &mut variable4 as *mut libc::c_int as QueueValue);
        i += 1;
        i;
    }
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3447: {
        if queue_is_empty(queue) == 0 {} else {
            __assert_fail(
                b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3401: {
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3355: {
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                167 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3309: {
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                168 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3263: {
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3217: {
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3171: {
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3125: {
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3079: {
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                176 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_push_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = queue_new();
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_pop_tail() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_pop_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3807: {
        if (queue_pop_tail(queue)).is_null() {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                202 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3746: {
            if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    202 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                203 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3700: {
            if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    203 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3654: {
            if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    204 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                205 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3608: {
            if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    205 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 31],
                        &[libc::c_char; 31],
                    >(b"void test_queue_pop_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if (queue_pop_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3556: {
        if (queue_pop_tail(queue)).is_null() {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                208 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_peek_tail() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_peek_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_4313: {
        if (queue_peek_tail(queue)).is_null() {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                220 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_peek_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                230 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4252: {
            if queue_peek_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_tail(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    230 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4206: {
            if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable1\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    231 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                232 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4160: {
            if queue_peek_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_tail(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    232 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                233 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4114: {
            if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable2\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    233 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                234 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4068: {
            if queue_peek_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_tail(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    234 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                235 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_4022: {
            if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable3\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    235 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_peek_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3976: {
            if queue_peek_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_peek_tail(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    236 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                237 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
        'c_3930: {
            if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
            {} else {
                __assert_fail(
                    b"queue_pop_tail(queue) == &variable4\0" as *const u8
                        as *const libc::c_char,
                    b"test-without-alloc/test-queue.c\0" as *const u8
                        as *const libc::c_char,
                    237 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<
                        &[u8; 32],
                        &[libc::c_char; 32],
                    >(b"void test_queue_peek_tail(void)\0"))
                        .as_ptr(),
                );
            }
        };
    }
    if (queue_peek_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_tail(void)\0"))
                .as_ptr(),
        );
    }
    'c_3874: {
        if (queue_peek_tail(queue)).is_null() {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                240 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_is_empty() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if queue_is_empty(queue) != 0 {} else {
        __assert_fail(
            b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            250 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_4567: {
        if queue_is_empty(queue) != 0 {} else {
            __assert_fail(
                b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                250 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_is_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_push_head(queue, &mut variable1 as *mut libc::c_int as QueueValue);
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            254 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_4519: {
        if queue_is_empty(queue) == 0 {} else {
            __assert_fail(
                b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                254 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_is_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_pop_head(queue);
    if queue_is_empty(queue) != 0 {} else {
        __assert_fail(
            b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            258 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_4476: {
        if queue_is_empty(queue) != 0 {} else {
            __assert_fail(
                b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                258 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_is_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_push_tail(queue, &mut variable1 as *mut libc::c_int as QueueValue);
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_4428: {
        if queue_is_empty(queue) == 0 {} else {
            __assert_fail(
                b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                262 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_is_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_pop_tail(queue);
    if queue_is_empty(queue) != 0 {} else {
        __assert_fail(
            b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    }
    'c_4384: {
        if queue_is_empty(queue) != 0 {} else {
            __assert_fail(
                b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
                b"test-without-alloc/test-queue.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_is_empty(void)\0"))
                    .as_ptr(),
            );
        }
    };
    queue_free(queue);
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
