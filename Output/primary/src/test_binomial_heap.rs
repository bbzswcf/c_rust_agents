use crate::binomial_heap::*;
use crate::compare_int::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 10000 }; }

static mut test_array: [i32; NUM_TEST_VALUES!()] = [0; NUM_TEST_VALUES!()];

macro_rules! TEST_VALUE { () => { (NUM_TEST_VALUES!() / 2) }; }
#[test]
fn test_binomial_heap_new_free() {
    let mut heap: Owned<BinomialHeap<Ptr<i32>>>;
    let mut i: i32;
    c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
        heap = binomial_heap_new(binomial_heap_type_min!(), func!(int_compare));
        binomial_heap_free(heap.unowned());
    });
    test_no_memory_leak!();
}
#[test]
fn test_binomial_heap_insert() {
    unsafe {
        let mut heap: Owned<BinomialHeap<Ptr<i32>>>;
        let mut i: i32;
        heap = binomial_heap_new(binomial_heap_type_min!(), func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            assert!(binomial_heap_insert(heap.unowned(), c_ref!(test_array[i])) != 0);
        });
        assert!(binomial_heap_num_entries(heap.unowned()) == NUM_TEST_VALUES!() as u32);
        binomial_heap_free(heap.unowned());
        test_no_memory_leak!();
    }
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
