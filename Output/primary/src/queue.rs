use crate::translation_utils::*;pub type Queue<T: GenericValue> = _Queue<T>;
pub type QueueValue<T: GenericValue> = T;

#[macro_export]
macro_rules! queue_null { () => { null!() } }

pub type QueueEntry<T: GenericValue> = _QueueEntry<T>;

#[derive(Default)]
pub struct _QueueEntry<T: GenericValue> {
    pub data: QueueValue<T>,
    pub prev: Manual<QueueEntry<T>>,
    pub next: Manual<QueueEntry<T>>,
}

#[derive(Default)]
pub struct _Queue<T: GenericValue> {
    pub head: Manual<QueueEntry<T>>,
    pub tail: Manual<QueueEntry<T>>,
}
pub fn queue_new<T: GenericValue>() -> Manual<Queue<T>> {
    let mut queue: Manual<Queue<T>>;
    queue = c_malloc!(c_sizeof!(Queue<T>));
    if queue == null!() {
        return null!();
    }
    queue.head = null!();
    queue.tail = null!();
    return queue;
}
pub fn queue_free<T: GenericValue>(mut queue: Manual<Queue<T>>) {
    while !queue_is_empty(queue).as_bool() {
        queue_pop_head(queue);
    }
    c_free!(queue);
}
pub fn queue_push_head<T: GenericValue>(mut queue: Manual<Queue<T>>, mut data: QueueValue<T>) -> i32 {
    let mut new_entry: Manual<QueueEntry<T>>;
    new_entry = c_malloc!(c_sizeof!(QueueEntry<T>));
    if new_entry == null!() {
        return 0;
    }
    new_entry.data = data;
    new_entry.prev = null!();
    new_entry.next = queue.head;
    if queue.head == null!() {
        queue.head = new_entry;
        queue.tail = new_entry;
    } else {
        queue.head.prev = new_entry;
        queue.head = new_entry;
    }
    return 1;
}
pub fn queue_pop_head<T: GenericValue>(mut queue: Manual<Queue<T>>) -> QueueValue<T> {
    let mut entry: Manual<QueueEntry<T>>;
    let mut result: QueueValue<T>;
    if queue_is_empty(queue).as_bool() {
        return queue_null!();
    }
    entry = queue.head;
    queue.head = entry.next;
    result = entry.data;
    if queue.head == null!() {
        queue.tail = null!();
    } else {
        queue.head.prev = null!();
    }
    c_free!(entry);
    return result;
}
pub fn queue_peek_head<T: GenericValue>(mut queue: Manual<Queue<T>>) -> QueueValue<T> {
    if queue_is_empty(queue).as_bool() {
        return queue_null!();
    } else {
        return queue.head.data;
    }
}
pub fn queue_push_tail<T: GenericValue>(mut queue: Manual<Queue<T>>, mut data: QueueValue<T>) -> i32 {
    let mut new_entry: Manual<QueueEntry<T>>;
    new_entry = c_malloc!(c_sizeof!(QueueEntry<T>));
    if new_entry == null!() {
        return 0;
    }
    new_entry.data = data;
    new_entry.prev = queue.tail;
    new_entry.next = null!();
    if queue.tail == null!() {
        queue.head = new_entry;
        queue.tail = new_entry;
    } else {
        queue.tail.next = new_entry;
        queue.tail = new_entry;
    }
    return 1;
}
pub fn queue_pop_tail<T: GenericValue>(mut queue: Manual<Queue<T>>) -> QueueValue<T> {
    let mut entry: Manual<QueueEntry<T>>;
    let mut result: QueueValue<T>;
    if queue_is_empty(queue).as_bool() {
        return queue_null!();
    }
    entry = queue.tail;
    queue.tail = entry.prev;
    result = entry.data;
    if queue.tail == null!() {
        queue.head = null!();
    } else {
        queue.tail.next = null!();
    }
    c_free!(entry);
    return result;
}
pub fn queue_peek_tail<T: GenericValue>(mut queue: Manual<Queue<T>>) -> QueueValue<T> {
    if queue_is_empty(queue).as_bool() {
        return queue_null!();
    } else {
        return queue.tail.data;
    }
}
pub fn queue_is_empty<T: GenericValue>(mut queue: Manual<Queue<T>>) -> i32 {
    return (queue.head == null!()) as i32;
}
