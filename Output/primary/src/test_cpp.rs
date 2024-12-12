use crate::compare_int::*;
use crate::compare_pointer::*;
use crate::compare_string::*;
use crate::hash_int::*;
use crate::hash_pointer::*;
use crate::hash_string::*;
use crate::arraylist::*;
use crate::avl_tree::*;
use crate::binary_heap::*;
use crate::binomial_heap::*;
use crate::bloom_filter::*;
use crate::hash_table::*;
use crate::list::*;
use crate::queue::*;
use crate::set::*;
use crate::slist::*;
use crate::trie::*;
use crate::translation_utils::*;
#[test]
fn test_compare_int() {
    let mut a = 1;
    let mut b = 2;
    assert!(!int_equal(c_ref!(a), c_ref!(b)));
    a = 2;
    b = 2;
    assert!(int_equal(c_ref!(a), c_ref!(b)));
}
#[test]
fn test_compare_pointer() {
    let mut a = 0;
    let mut b = 0;
    let p1 = c_ref!(a);
    let p2 = c_ref!(b);
    assert_eq!(pointer_equal(p1, p2), 0);

    let p1 = c_ref!(a);
    let p2 = c_ref!(a);
    assert!(pointer_equal(p1, p2) != 0);
}
#[test]
fn test_compare_string() {
    let s1 = cstr!("hello");
    let s2 = cstr!("hello");
    let s3 = cstr!("world");
    assert!(!string_equal(s1, s3) != 0);
    assert!(string_equal(s1, s2) != 0);
}
#[test]
fn test_hash_int() {
    let mut a: i32;
    let mut b: i32;
    a = 1; b = 2;
    assert!(int_hash(c_ref!(a)) != int_hash(c_ref!(b)));
    a = 2; b = 2;
    assert!(int_hash(c_ref!(a)) == int_hash(c_ref!(b)));
}
#[test]
fn test_hash_pointer() {
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut p1: *mut i32;
    let mut p2: *mut i32;

    p1 = &mut a;
    p2 = &mut b;
    assert!(pointer_hash(Ptr::from(p1 as *mut _)) != pointer_hash(Ptr::from(p2 as *mut _)));

    p1 = &mut a;
    p2 = &mut a;
    assert!(pointer_hash(Ptr::from(p1 as *mut _)) == pointer_hash(Ptr::from(p2 as *mut _)));
}
#[test]
fn test_hash_string() {
    let mut s1 = cstr!("hello");
    let mut s2 = cstr!("hello");
    let mut s3 = cstr!("world");
    assert!(string_hash(s1) != string_hash(s3));
    assert_eq!(string_hash(s1), string_hash(s2));
}
#[test]
fn test_arraylist() {
    unsafe {
        let mut arraylist: Owned<ArrayList<Ptr<i32>>>;
        arraylist = arraylist_new(0);
        arraylist_free(arraylist.unowned());
    }
}
#[test]
fn test_avl_tree() {
    let mut avl_tree: Owned<AVLTree<CStr>>;
    avl_tree = avl_tree_new(func!(string_compare));
    avl_tree_free(avl_tree.unowned());
}
#[test]
fn test_binary_heap() {
    let mut heap: Owned<BinaryHeap<CStr>>;
    heap = binary_heap_new(binary_heap_type_max!(), func!(string_compare));
    binary_heap_free(heap.unowned());
}
#[test]
fn test_binomial_heap() {
    let mut heap: Owned<BinomialHeap<CStr>>;
    heap = binomial_heap_new(binomial_heap_type_max!(), func!(string_compare));
    binomial_heap_free(heap.unowned());
}
#[test]
fn test_bloom_filter() {
    let mut filter: Owned<BloomFilter<CStr>> = bloom_filter_new(16, FuncPtr::new(string_hash), 10);
    bloom_filter_free(filter.into_unowned());
}
#[test]
fn test_hash_table() {
    let mut hash_table: Owned<HashTable<CStr>>;
    hash_table = hash_table_new(func!(string_hash), func!(string_equal));
    hash_table_free(hash_table.unowned());
}
#[test]
fn test_list() {
    unsafe {
        let mut list: Manual<ListEntry<Ptr<i32>>> = null!();
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;

        assert!(list_prepend(c_ref!(list), c_ref!(a)) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(b)) != null!());
        check_list_integrity(list);
        assert!(list_prepend(c_ref!(list), c_ref!(c)) != null!());
        check_list_integrity(list);

        list_free(list);
        test_no_memory_leak!();
    }
}
#[test]
fn test_queue() {
    unsafe {
        let mut queue: Manual<Queue<Ptr<i32>>>;
        queue = queue_new();
        queue_free(queue);
    }
}
#[test]
fn test_set() {
    let mut set: Owned<Set<CStr>>;
    set = set_new(func!(string_hash), func!(string_equal));
    set_free(set.unowned());
}
#[test]
fn test_slist() {
    unsafe {
        let mut list: Manual<SListEntry<Ptr<i32>>> = null!();
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut c: i32 = 0;

        assert!(slist_prepend(c_ref!(list), c_ref!(a)) != null!());
        assert!(slist_prepend(c_ref!(list), c_ref!(b)) != null!());
        assert!(slist_prepend(c_ref!(list), c_ref!(c)) != null!());

        slist_free(list);
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
