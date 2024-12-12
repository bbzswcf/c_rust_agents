use crate::translation_utils::*;pub type BinaryHeapType = i32;
#[macro_export]
macro_rules! binary_heap_type_min { () => { 0 } }
#[macro_export]
macro_rules! binary_heap_type_max { () => { 1 } }
pub type BinaryHeapValue<T: GenericValue> = T;
#[macro_export]
macro_rules! binary_heap_null { () => { null!() } }
pub type BinaryHeapCompareFunc<T: GenericValue> = FuncPtr<fn(BinaryHeapValue<T>, BinaryHeapValue<T>) -> i32>;
pub type BinaryHeap<T: GenericValue> = _BinaryHeap<T>;
#[derive(Default)]
pub struct _BinaryHeap<T: GenericValue> {
    pub heap_type: BinaryHeapType,
    pub values: Vector<BinaryHeapValue<T>>,
    pub num_values: u32,
    pub alloced_size: u32,
    pub compare_func: BinaryHeapCompareFunc<T>,
}
pub fn binary_heap_cmp<T: GenericValue>(heap: Unowned<BinaryHeap<T>>, data1: BinaryHeapValue<T>, data2: BinaryHeapValue<T>) -> i32 {
    if heap.heap_type == binary_heap_type_min!() {
        return (heap.compare_func)(data1, data2);
    } else {
        return -(heap.compare_func)(data1, data2);
    }
}
pub fn binary_heap_new<T: GenericValue>(heap_type: BinaryHeapType, compare_func: BinaryHeapCompareFunc<T>) -> Owned<BinaryHeap<T>> {
    let mut heap: Owned<BinaryHeap<T>>;
    heap = c_malloc!(c_sizeof!(BinaryHeap<T>));
    if heap == null!() {
        return null!();
    }
    heap.heap_type = heap_type;
    heap.num_values = 0;
    heap.compare_func = compare_func;
    heap.alloced_size = 16;
    heap.values = c_malloc!(c_sizeof!(BinaryHeapValue<T>) * heap.alloced_size as usize);
    if heap.values == null!() {
        c_free!(heap);
        return null!();
    }
    return heap;
}
pub fn binary_heap_free<T: GenericValue>(mut heap: Unowned<BinaryHeap<T>>) {
    c_free!(heap.values);
    c_free!(heap);
}
pub fn binary_heap_insert<T: GenericValue>(mut heap: Unowned<BinaryHeap<T>>, value: BinaryHeapValue<T>) -> i32 {
    let mut new_values: Vector<BinaryHeapValue<T>>;
    let mut index: u32;
    let mut new_size: u32;
    let mut parent: u32;
    if heap.num_values >= heap.alloced_size {
        new_size = heap.alloced_size * 2;
        new_values = c_realloc!(heap.values, c_sizeof!(BinaryHeapValue<T>) * new_size as usize);
        if new_values == null!() {
            return 0;
        }
        heap.alloced_size = new_size;
        heap.values = new_values;
    }
    index = heap.num_values;
    heap.num_values += 1;
    while index > 0 {
        parent = (index - 1) / 2;
        if binary_heap_cmp(heap, heap.values[parent], value) < 0 {
            break;
        } else {
            heap.values[index] = heap.values[parent];
            index = parent;
        }
    }
    heap.values[index] = value;
    return 1;
}
pub fn binary_heap_pop<T: GenericValue>(mut heap: Unowned<BinaryHeap<T>>) -> BinaryHeapValue<T> {
    let mut result: BinaryHeapValue<T>;
    let mut new_value: BinaryHeapValue<T>;
    let mut index: u32;
    let mut next_index: u32;
    let mut child1: u32;
    let mut child2: u32;
    if heap.num_values == 0 {
        return binary_heap_null!();
    }
    result = heap.values[0];
    new_value = heap.values[heap.num_values - 1];
    heap.num_values -= 1;
    index = 0;
    loop {
        child1 = index * 2 + 1;
        child2 = index * 2 + 2;
        if child1 < heap.num_values && binary_heap_cmp(heap, new_value, heap.values[child1]) > 0 {
            if child2 < heap.num_values && binary_heap_cmp(heap, heap.values[child1], heap.values[child2]) > 0 {
                next_index = child2;
            } else {
                next_index = child1;
            }
        } else if child2 < heap.num_values && binary_heap_cmp(heap, new_value, heap.values[child2]) > 0 {
            next_index = child2;
        } else {
            heap.values[index] = new_value;
            break;
        }
        heap.values[index] = heap.values[next_index];
        index = next_index;
    }
    return result;
}
pub fn binary_heap_num_entries<T: GenericValue>(heap: Unowned<BinaryHeap<T>>) -> u32 {
    return heap.num_values;
}
