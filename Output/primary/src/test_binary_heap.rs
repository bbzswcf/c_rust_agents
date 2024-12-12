use crate::binary_heap::*;
use crate::compare_int::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 10000 }; }
static mut test_array: Array<i32, {NUM_TEST_VALUES!()}> = arr![0; NUM_TEST_VALUES!()];
#[test]
fn test_binary_heap_new_free() {
    let mut heap: Owned<BinaryHeap<Ptr<i32>>>;
    let mut i: i32;
    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        heap = binary_heap_new(binary_heap_type_min!(), func!(int_compare));
        binary_heap_free(heap.unowned());
    });
}
#[test]
fn test_binary_heap_insert() {
    unsafe {
        let mut heap: Owned<BinaryHeap<Ptr<i32>>>;
        let mut i: i32;
        heap = binary_heap_new(binary_heap_type_min!(), func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            assert!(binary_heap_insert(heap.unowned(), c_ref!(test_array[i])) != 0);
        });
        assert!(binary_heap_num_entries(heap.unowned()) == NUM_TEST_VALUES!());
        binary_heap_free(heap.unowned());
    }
}
#[test]
fn test_min_heap() {
    unsafe {
        let mut heap: Owned<BinaryHeap<Ptr<i32>>>;
        let mut val: Ptr<i32>;
        let mut i: i32;
        heap = binary_heap_new(binary_heap_type_min!(), func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            assert!(binary_heap_insert(heap.unowned(), c_ref!(test_array[i])) != 0);
        });
        i = -1;
        while binary_heap_num_entries(heap.unowned()) > 0 {
            val = binary_heap_pop(heap.unowned());
            assert!(*val == i + 1);
            i = *val;
        }
        assert!(binary_heap_num_entries(heap.unowned()) == 0);
        assert!(binary_heap_pop(heap.unowned()) == binary_heap_null!());
        binary_heap_free(heap.unowned());
    }
}
#[test]
fn test_max_heap() {
    unsafe {
        let mut heap: Owned<BinaryHeap<Ptr<i32>>>;
        let mut val: Ptr<i32>;
        let mut i: i32;
        heap = binary_heap_new(binary_heap_type_max!(), func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            assert!(binary_heap_insert(heap.unowned(), c_ref!(test_array[i])) != 0);
        });
        i = NUM_TEST_VALUES!();
        while binary_heap_num_entries(heap.unowned()) > 0 {
            val = binary_heap_pop(heap.unowned());
            assert!(*val == i - 1);
            i = *val;
        }
        binary_heap_free(heap.unowned());
    }
}
