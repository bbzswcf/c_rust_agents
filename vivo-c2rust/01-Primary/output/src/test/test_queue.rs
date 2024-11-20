#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type _Queue;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn run_tests(tests_0: *mut UnitTestFunction);
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
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
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
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_head(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_pop_head() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_pop_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_head(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                143 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                145 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_head(void)\0"))
                    .as_ptr(),
            );
        };
    }
    if (queue_pop_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_head(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_head(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_peek_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                171 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                172 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                173 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                175 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                176 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                177 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_head(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                178 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_head(void)\0"))
                    .as_ptr(),
            );
        };
    }
    if (queue_peek_head(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_head(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_head(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            207 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_head(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_head(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            209 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable4\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable3\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable2\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
    {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == &variable1\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_push_tail(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
}
#[no_mangle]
pub unsafe extern "C" fn test_queue_pop_tail() {
    let mut queue: *mut Queue = 0 as *mut Queue;
    queue = queue_new();
    if (queue_pop_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_tail(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                247 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                249 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                250 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_queue_pop_tail(void)\0"))
                    .as_ptr(),
            );
        };
    }
    if (queue_pop_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_pop_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            253 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_pop_tail(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            266 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_tail(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
    queue = generate_queue();
    while queue_is_empty(queue) == 0 {
        if queue_peek_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                276 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable1 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                277 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                278 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable2 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable2\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                279 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                280 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable3 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable3\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                281 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_peek_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_peek_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                282 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
        if queue_pop_tail(queue) == &mut variable4 as *mut libc::c_int as QueueValue
        {} else {
            __assert_fail(
                b"queue_pop_tail(queue) == &variable4\0" as *const u8
                    as *const libc::c_char,
                b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_queue_peek_tail(void)\0"))
                    .as_ptr(),
            );
        };
    }
    if (queue_peek_tail(queue)).is_null() {} else {
        __assert_fail(
            b"queue_peek_tail(queue) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            286 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_queue_peek_tail(void)\0"))
                .as_ptr(),
        );
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
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    };
    queue_push_head(queue, &mut variable1 as *mut libc::c_int as QueueValue);
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    };
    queue_pop_head(queue);
    if queue_is_empty(queue) != 0 {} else {
        __assert_fail(
            b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            305 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    };
    queue_push_tail(queue, &mut variable1 as *mut libc::c_int as QueueValue);
    if queue_is_empty(queue) == 0 {} else {
        __assert_fail(
            b"!queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    };
    queue_pop_tail(queue);
    if queue_is_empty(queue) != 0 {} else {
        __assert_fail(
            b"queue_is_empty(queue)\0" as *const u8 as *const libc::c_char,
            b"test/test-queue.c\0" as *const u8 as *const libc::c_char,
            313 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_queue_is_empty(void)\0"))
                .as_ptr(),
        );
    };
    queue_free(queue);
}
static mut tests: [UnitTestFunction; 9] = unsafe {
    [
        Some(test_queue_new_free as unsafe extern "C" fn() -> ()),
        Some(test_queue_push_head as unsafe extern "C" fn() -> ()),
        Some(test_queue_pop_head as unsafe extern "C" fn() -> ()),
        Some(test_queue_peek_head as unsafe extern "C" fn() -> ()),
        Some(test_queue_push_tail as unsafe extern "C" fn() -> ()),
        Some(test_queue_pop_tail as unsafe extern "C" fn() -> ()),
        Some(test_queue_peek_tail as unsafe extern "C" fn() -> ()),
        Some(test_queue_is_empty as unsafe extern "C" fn() -> ()),
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
