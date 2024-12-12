use crate::queue::*;
use crate::translation_utils::*;static mut variable1: i32 = 0;
static mut variable2: i32 = 0;
static mut variable3: i32 = 0;
static mut variable4: i32 = 0;
fn generate_queue() -> Manual<Queue<Ptr<i32>>> {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>> = queue_new();
        let mut i: i32;
        c_for!(i = 0; i < 1000; i += 1; {
            queue_push_head(queue, c_ref!(variable1));
            queue_push_head(queue, c_ref!(variable2));
            queue_push_head(queue, c_ref!(variable3));
            queue_push_head(queue, c_ref!(variable4));
        });
        return queue;
    }
}
#[test]
fn test_queue_new_free() {
    unsafe {
        let mut i: i32;
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        queue_free(queue);
        queue = queue_new();
        c_for!(i = 0; i < 1000; i += 1; {
            queue_push_head(queue, c_ref!(variable1));
        });
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_push_head() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        let mut i: i32;
        queue = queue_new();
        c_for!(i = 0; i < 1000; i += 1; {
            queue_push_head(queue, c_ref!(variable1));
            queue_push_head(queue, c_ref!(variable2));
            queue_push_head(queue, c_ref!(variable3));
            queue_push_head(queue, c_ref!(variable4));
        });
        assert!(!queue_is_empty(queue).as_bool());
        assert!(queue_pop_tail(queue) == c_ref!(variable1));
        assert!(queue_pop_tail(queue) == c_ref!(variable2));
        assert!(queue_pop_tail(queue) == c_ref!(variable3));
        assert!(queue_pop_tail(queue) == c_ref!(variable4));
        assert!(queue_pop_head(queue) == c_ref!(variable4));
        assert!(queue_pop_head(queue) == c_ref!(variable3));
        assert!(queue_pop_head(queue) == c_ref!(variable2));
        assert!(queue_pop_head(queue) == c_ref!(variable1));
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_pop_head() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        assert!(queue_pop_head(queue) == null!());
        queue_free(queue);
        queue = generate_queue();
        while !queue_is_empty(queue).as_bool() {
            assert!(queue_pop_head(queue) == c_ref!(variable4));
            assert!(queue_pop_head(queue) == c_ref!(variable3));
            assert!(queue_pop_head(queue) == c_ref!(variable2));
            assert!(queue_pop_head(queue) == c_ref!(variable1));
        }
        assert!(queue_pop_head(queue) == null!());
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_peek_head() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        assert!(queue_peek_head(queue) == null!());
        queue_free(queue);
        queue = generate_queue();
        while !queue_is_empty(queue).as_bool() {
            assert!(queue_peek_head(queue) == c_ref!(variable4));
            assert!(queue_pop_head(queue) == c_ref!(variable4));
            assert!(queue_peek_head(queue) == c_ref!(variable3));
            assert!(queue_pop_head(queue) == c_ref!(variable3));
            assert!(queue_peek_head(queue) == c_ref!(variable2));
            assert!(queue_pop_head(queue) == c_ref!(variable2));
            assert!(queue_peek_head(queue) == c_ref!(variable1));
            assert!(queue_pop_head(queue) == c_ref!(variable1));
        }
        assert!(queue_peek_head(queue) == null!());
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_push_tail() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        let mut i: i32;
        queue = queue_new();
        c_for!(i = 0; i < 1000; i += 1; {
            queue_push_tail(queue, c_ref!(variable1));
            queue_push_tail(queue, c_ref!(variable2));
            queue_push_tail(queue, c_ref!(variable3));
            queue_push_tail(queue, c_ref!(variable4));
        });
        assert!(!queue_is_empty(queue).as_bool());
        assert!(queue_pop_head(queue) == c_ref!(variable1));
        assert!(queue_pop_head(queue) == c_ref!(variable2));
        assert!(queue_pop_head(queue) == c_ref!(variable3));
        assert!(queue_pop_head(queue) == c_ref!(variable4));
        assert!(queue_pop_tail(queue) == c_ref!(variable4));
        assert!(queue_pop_tail(queue) == c_ref!(variable3));
        assert!(queue_pop_tail(queue) == c_ref!(variable2));
        assert!(queue_pop_tail(queue) == c_ref!(variable1));
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_pop_tail() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        assert!(queue_pop_tail(queue) == null!());
        queue_free(queue);
        queue = generate_queue();
        while !queue_is_empty(queue).as_bool() {
            assert!(queue_pop_tail(queue) == c_ref!(variable1));
            assert!(queue_pop_tail(queue) == c_ref!(variable2));
            assert!(queue_pop_tail(queue) == c_ref!(variable3));
            assert!(queue_pop_tail(queue) == c_ref!(variable4));
        }
        assert!(queue_pop_tail(queue) == null!());
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_peek_tail() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        assert!(queue_peek_tail(queue) == null!());
        queue_free(queue);
        queue = generate_queue();
        while !queue_is_empty(queue).as_bool() {
            assert!(queue_peek_tail(queue) == c_ref!(variable1));
            assert!(queue_pop_tail(queue) == c_ref!(variable1));
            assert!(queue_peek_tail(queue) == c_ref!(variable2));
            assert!(queue_pop_tail(queue) == c_ref!(variable2));
            assert!(queue_peek_tail(queue) == c_ref!(variable3));
            assert!(queue_pop_tail(queue) == c_ref!(variable3));
            assert!(queue_peek_tail(queue) == c_ref!(variable4));
            assert!(queue_pop_tail(queue) == c_ref!(variable4));
        }
        assert!(queue_peek_tail(queue) == null!());
        queue_free(queue);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue_is_empty() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        assert!(queue_is_empty(queue).as_bool());
        queue_push_head(queue, c_ref!(variable1));
        assert!(!queue_is_empty(queue).as_bool());
        queue_pop_head(queue);
        assert!(queue_is_empty(queue).as_bool());
        queue_push_tail(queue, c_ref!(variable1));
        assert!(!queue_is_empty(queue).as_bool());
        queue_pop_tail(queue);
        assert!(queue_is_empty(queue).as_bool());
        queue_free(queue);
        test_no_memory_leak!();
    }
}
