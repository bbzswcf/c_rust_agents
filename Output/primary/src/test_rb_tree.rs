use crate::rb_tree::*;
use crate::compare_int::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 1000 }; }
static mut test_array: Array<i32, {NUM_TEST_VALUES!()}> = arr![0; NUM_TEST_VALUES!()];
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
fn validate_tree<T: GenericValue>(mut tree: Unowned<RBTree<T>>) {
}
fn create_tree() -> Owned<RBTree<Ptr<i32>>> {
    unsafe {
        let mut tree: Owned<RBTree<Ptr<i32>>>;
        let mut i: i32;
        tree = rb_tree_new(func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            rb_tree_insert(tree.unowned(), c_ref!(test_array[i]), c_ref!(test_array[i]));
        });
        return tree;
    }
}
#[test]
fn test_rb_tree_new() {
    let mut tree: Owned<RBTree<Ptr<i32>>>;
    tree = rb_tree_new(func!(int_compare));
    assert!(tree != null!());
    assert!(rb_tree_root_node(tree.unowned()) == null!());
    assert!(rb_tree_num_entries(tree.unowned()) == 0);
    rb_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_rb_tree_insert_lookup() {
    unsafe {
        let mut tree: Owned<RBTree<Ptr<i32>>>;
        let mut node: Manual<RBTreeNode<Ptr<i32>>>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = rb_tree_new(func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            test_array[i] = i;
            rb_tree_insert(tree.unowned(), c_ref!(test_array[i]), c_ref!(test_array[i]));
            assert!(rb_tree_num_entries(tree.unowned()) == i + 1);
            validate_tree(tree.unowned());
        });
        assert!(rb_tree_root_node(tree.unowned()) != null!());
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            node = rb_tree_lookup_node(tree.unowned(), c_ref!(i));
            assert!(node != null!());
            value = rb_tree_node_key(node);
            assert!(*value == i);
            value = rb_tree_node_value(node);
            assert!(*value == i);
        });
        i = -1;
        assert!(rb_tree_lookup_node(tree.unowned(), c_ref!(i)) == null!());
        i = NUM_TEST_VALUES!() + 100;
        assert!(rb_tree_lookup_node(tree.unowned(), c_ref!(i)) == null!());
        rb_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_rb_tree_child() {
    let mut tree: Owned<RBTree<Ptr<i32>>>;
    let mut root: Manual<RBTreeNode<Ptr<i32>>>;
    let mut left: Manual<RBTreeNode<Ptr<i32>>>;
    let mut right: Manual<RBTreeNode<Ptr<i32>>>;
    let mut values: Array<i32, 3> = arr![1, 2, 3];
    let mut p: Ptr<i32>;
    let mut i: i32;

    tree = rb_tree_new(func!(int_compare));

    c_for!(i = 0; i < 3; i += 1; {
        rb_tree_insert(tree.unowned(), c_ref!(values[i]), c_ref!(values[i]));
    });

    root = rb_tree_root_node(tree.unowned());
    p = rb_tree_node_value(root);
    assert!(*p == 2);

    left = rb_tree_node_child(root, rb_tree_node_left!());
    p = rb_tree_node_value(left);
    assert!(*p == 1);

    right = rb_tree_node_child(root, rb_tree_node_right!());
    p = rb_tree_node_value(right);
    assert!(*p == 3);

    assert!(rb_tree_node_child(root, 10000) == null!());
    assert!(rb_tree_node_child(root, 2) == null!());

    rb_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_rb_tree_free() {
    let mut tree: Owned<RBTree<Ptr<i32>>>;
    tree = rb_tree_new(func!(int_compare));
    rb_tree_free(tree.unowned());
    tree = create_tree();
    rb_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_rb_tree_lookup() {
    unsafe {
        let mut tree: Owned<RBTree<Ptr<i32>>>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = create_tree();
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            value = rb_tree_lookup(tree.unowned(), c_ref!(i));
            assert!(value != null!());
            assert!(*value == i);
        });
        i = -1;
        assert!(rb_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        i = NUM_TEST_VALUES!() + 1;
        assert!(rb_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        i = 8724897;
        assert!(rb_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        rb_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_rb_tree_remove() {
    unsafe {
        let mut tree: Owned<RBTree<Ptr<i32>>>;
        let mut i: i32;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut value: i32;
        let mut expected_entries: i32;

        tree = create_tree();
        i = NUM_TEST_VALUES!() + 100;
        assert!(rb_tree_remove(tree.unowned(), c_ref!(i)) == 0);
        i = -1;
        assert!(rb_tree_remove(tree.unowned(), c_ref!(i)) == 0);
        expected_entries = NUM_TEST_VALUES!();
        c_for!(x = 0; x < 10; x += 1; {
            c_for!(y = 0; y < 10; y += 1; {
                c_for!(z = 0; z < 10; z += 1; {
                    value = z * 100 + (9 - y) * 10 + x;
                    assert!(rb_tree_remove(tree.unowned(), c_ref!(value)) != 0);
                    validate_tree(tree.unowned());
                    expected_entries -= 1;
                    assert!(rb_tree_num_entries(tree.unowned()) == expected_entries);
                });
            });
        });

        assert!(rb_tree_root_node(tree.unowned()).0.is_none());
        rb_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_rb_tree_to_array() {
    unsafe {
        let mut entries: Array<i32, 10> = arr![89, 23, 42, 4, 16, 15, 8, 99, 50, 30];
        let sorted: Array<i32, 10> = arr![4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
        let num_entries = 10;
        let mut i: i32;
        let mut array: Manual<RBTreeValue<Ptr<i32>>>;
        let mut tree = rb_tree_new(func!(int_compare));

        c_for!(i = 0; i < num_entries; i += 1; {
            let entry_ptr = c_ref!(entries[i as usize]);
            rb_tree_insert(tree.unowned(), entry_ptr, entry_ptr);
        });

        assert!(rb_tree_num_entries(tree.unowned()) == num_entries);
        array = rb_tree_to_array(tree.unowned());

        if array.0.is_some() {
            c_for!(i = 0; i < num_entries; i += 1; {
                let value = array.0.unwrap().as_ptr().offset(i as isize);
                assert!(**value == sorted[i as usize]);
            });
        }

        rb_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
