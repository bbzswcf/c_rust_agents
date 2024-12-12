use crate::compare_int::*;
use crate::sortedarray::*;
use crate::translation_utils::*;macro_rules! test_size { () => { 20 }; }
macro_rules! test_array { () => { arr![10, 12, 12, 1, 2, 3, 6, 7, 2, 23, 13, 23, 23, 34, 31, 9, 21, -2, -12, -4] }; }
macro_rules! test_remove_el { () => { 15 }; }
macro_rules! test_remove_range { () => { 7 }; }
macro_rules! test_remove_range_length { () => { 4 }; }
fn check_sorted_prop(mut sortedarray: Unowned<SortedArray<Ptr<i32>>>) {
    let mut i: u32;
    c_for!(i = 1; i < sortedarray_length(sortedarray); i += 1; {
        assert!(int_compare(sortedarray_get(sortedarray, i - 1), sortedarray_get(sortedarray, i)) <= 0);
    });
}
fn free_sorted_ints(mut sortedarray: Owned<SortedArray<Ptr<i32>>>) {
    let mut i: u32;
    c_for!(i = 0; i < sortedarray_length(sortedarray.unowned()); i += 1; {
        let mut pi: Ptr<i32> = sortedarray_get(sortedarray.unowned(), i);
        c_free!(pi);
    });
    sortedarray_free(sortedarray.unowned());
}
fn generate_sortedarray_equ(mut equ_func: SortedArrayEqualFunc<Ptr<i32>>) -> Owned<SortedArray<Ptr<i32>>> {
    let mut sortedarray: Owned<SortedArray<Ptr<i32>>>;
    let mut i: u32;
    let array: Array<i32, {test_size!()}> = test_array!();
    sortedarray = sortedarray_new(0, equ_func, func!(int_compare));
    c_for!(i = 0; i < test_size!(); i += 1; {
        let mut pi: Ptr<i32> = c_malloc!(c_sizeof!(i32));
        *pi = array[i];
        sortedarray_insert(sortedarray.unowned(), pi);
    });
    sortedarray
}
fn generate_sortedarray() -> Owned<SortedArray<Ptr<i32>>> {
    generate_sortedarray_equ(func!(int_equal))
}
#[test]
fn test_sortedarray_new_free() {
    let mut sortedarray: Owned<SortedArray<Ptr<i32>>>;
    sortedarray = sortedarray_new(0, func!(int_equal), func!(int_compare));
    assert!(sortedarray != null!());
    sortedarray_free(sortedarray.unowned());
    sortedarray_free::<Ptr<i32>>(null!());
}
#[test]
fn test_sortedarray_insert() {
    let mut sortedarray: Owned<SortedArray<Ptr<i32>>> = generate_sortedarray();
    let mut i: u32;
    c_for!(i = 0; i < 20; i += 1; {
        let i: i32 = unsafe { (rand() as f32 / RAND_MAX as f32 * 100.0) as i32 };
        let mut pi: Ptr<i32> = c_malloc!(c_sizeof!(i32));
        *pi = i;
        sortedarray_insert(sortedarray.unowned(), pi);
    });
    check_sorted_prop(sortedarray.unowned());
    free_sorted_ints(sortedarray);
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
#[test]
fn test_sortedarray_index_of() {
    let mut sortedarray: Owned<SortedArray<Ptr<i32>>> = generate_sortedarray();
    let mut i: u32;
    c_for!(i = 0; i < test_size!(); i += 1; {
        let mut r: i32 = sortedarray_index_of(sortedarray.unowned(), sortedarray_get(sortedarray.unowned(), i));
        assert!(r >= 0);
        assert!(*sortedarray_get(sortedarray.unowned(), r as u32) == *sortedarray_get(sortedarray.unowned(), i));
    });
    free_sorted_ints(sortedarray);
}
fn ptr_equal<T: GenericValue>(mut v1: SortedArrayValue<T>, mut v2: SortedArrayValue<T>) -> i32 {
    (v1 == v2) as i32
}
#[test]
fn test_sortedarray_index_of_equ_key() {
    let mut sortedarray: Owned<SortedArray<Ptr<i32>>> = generate_sortedarray_equ(func!(ptr_equal));
    let mut i: u32;
    c_for!(i = 0; i < test_size!(); i += 1; {
        let mut r: i32 = sortedarray_index_of(sortedarray.unowned(), sortedarray_get(sortedarray.unowned(), i));
        assert!(r >= 0);
        assert!(i == r as u32);
    });
    free_sorted_ints(sortedarray);
}
#[test]
fn test_sortedarray_get() {
    let mut i: u32;
    let mut arr: Owned<SortedArray<Ptr<i32>>> = generate_sortedarray();
    c_for!(i = 0; i < sortedarray_length(arr.unowned()); i += 1; {
        assert!(sortedarray_get(arr.unowned(), i) == sortedarray_get(arr.unowned(), i));
        assert!(*sortedarray_get(arr.unowned(), i) == *sortedarray_get(arr.unowned(), i));
    });
    free_sorted_ints(arr);
}
