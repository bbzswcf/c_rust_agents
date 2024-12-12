use crate::avl_tree::*;
use crate::compare_int::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 1000 } }

static mut TEST_ARRAY: [i32; NUM_TEST_VALUES!()] = [0; NUM_TEST_VALUES!()];
static mut COUNTER: i32 = 0;
fn find_subtree_height(mut node: Manual<AVLTreeNode<Ptr<i32>>>) -> i32 {
    let mut left_subtree: Manual<AVLTreeNode<Ptr<i32>>>;
    let mut right_subtree: Manual<AVLTreeNode<Ptr<i32>>>;
    let mut left_height: i32;
    let mut right_height: i32;
    if node == null!() {
        return 0;
    }
    left_subtree = avl_tree_node_child(node, avl_tree_node_left!());
    right_subtree = avl_tree_node_child(node, avl_tree_node_right!());
    left_height = find_subtree_height(left_subtree);
    right_height = find_subtree_height(right_subtree);
    if left_height > right_height {
        return left_height + 1;
    } else {
        return right_height + 1;
    }
}
fn validate_subtree(mut node: Manual<AVLTreeNode<Ptr<i32>>>) -> i32 {
    unsafe {
        let mut left_node: Manual<AVLTreeNode<Ptr<i32>>>;
        let mut right_node: Manual<AVLTreeNode<Ptr<i32>>>;
        let mut left_height: i32;
        let mut right_height: i32;
        let mut key: Ptr<i32>;
        if node == null!() {
            return 0;
        }
        left_node = avl_tree_node_child(node, avl_tree_node_left!());
        right_node = avl_tree_node_child(node, avl_tree_node_right!());
        if left_node != null!() {
            assert!(avl_tree_node_parent(left_node) == node);
        }
        if right_node != null!() {
            assert!(avl_tree_node_parent(right_node) == node);
        }
        left_height = validate_subtree(left_node);
        key = avl_tree_node_key(node);
        assert!(*key > COUNTER);
        COUNTER = *key;
        right_height = validate_subtree(right_node);
        assert!(avl_tree_subtree_height(left_node) == left_height);
        assert!(avl_tree_subtree_height(right_node) == right_height);
        assert!(left_height - right_height < 2 && right_height - left_height < 2);
        if left_height > right_height {
            return left_height + 1;
        } else {
            return right_height + 1;
        }
    }
}
fn validate_tree(mut tree: Unowned<AVLTree<Ptr<i32>>>) {
    unsafe {
        let mut root_node: Manual<AVLTreeNode<Ptr<i32>>>;
        let mut height: i32;
        root_node = avl_tree_root_node(tree);
        if root_node != null!() {
            height = find_subtree_height(root_node);
            assert!(avl_tree_subtree_height(root_node) == height);
        }
        COUNTER = -1;
        validate_subtree(root_node);
    }
}
fn create_tree() -> Owned<AVLTree<Ptr<i32>>> {
    unsafe {
        let mut tree: Owned<AVLTree<Ptr<i32>>>;
        let mut i: i32;
        tree = avl_tree_new(func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            TEST_ARRAY[i as usize] = i;
            avl_tree_insert(tree.unowned(), c_ref!(TEST_ARRAY[i as usize]), c_ref!(TEST_ARRAY[i as usize]));
        });
        return tree;
    }
}
#[test]
fn test_avl_tree_new() {
    let mut tree: Owned<AVLTree<Ptr<i32>>>;
    tree = avl_tree_new(func!(int_compare));
    assert!(tree != null!());
    assert!(avl_tree_root_node(tree.unowned()) == null!());
    assert!(avl_tree_num_entries(tree.unowned()) == 0);
    avl_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_avl_tree_insert_lookup() {
    unsafe {
        let mut tree: Owned<AVLTree<Ptr<i32>>>;
        let mut node: Manual<AVLTreeNode<Ptr<i32>>>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = avl_tree_new(func!(int_compare));
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            TEST_ARRAY[i as usize] = i;
            avl_tree_insert(tree.unowned(), c_ref!(TEST_ARRAY[i as usize]), c_ref!(TEST_ARRAY[i as usize]));
            assert!(avl_tree_num_entries(tree.unowned()) == (i + 1) as u32);
            validate_tree(tree.unowned());
        });
        assert!(avl_tree_root_node(tree.unowned()) != null!());
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            node = avl_tree_lookup_node(tree.unowned(), c_ref!(i));
            assert!(node != null!());
            value = avl_tree_node_key(node);
            assert!(*value == i);
            value = avl_tree_node_value(node);
            assert!(*value == i);
        });
        i = NUM_TEST_VALUES!() + 100;
        assert!(avl_tree_lookup_node(tree.unowned(), c_ref!(i)) == null!());
        avl_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_avl_tree_child() {
    let mut tree: Owned<AVLTree<Ptr<i32>>>;
    let mut root: Manual<AVLTreeNode<Ptr<i32>>>;
    let mut left: Manual<AVLTreeNode<Ptr<i32>>>;
    let mut right: Manual<AVLTreeNode<Ptr<i32>>>;
    let mut values: Array<i32, 3> = arr![1, 2, 3];
    let mut p: Ptr<i32>;
    let mut i: i32;

    tree = avl_tree_new(func!(int_compare));

    c_for!(i = 0; i < 3; i += 1; {
        avl_tree_insert(tree.unowned(), c_ref!(values[i]), c_ref!(values[i]));
    });

    root = avl_tree_root_node(tree.unowned());
    p = avl_tree_node_value(root);
    assert!(*p == 2);

    left = avl_tree_node_child(root, avl_tree_node_left!());
    p = avl_tree_node_value(left);
    assert!(*p == 1);

    right = avl_tree_node_child(root, avl_tree_node_right!());
    p = avl_tree_node_value(right);
    assert!(*p == 3);

    assert!(avl_tree_node_child(root, 10000) == null!());
    assert!(avl_tree_node_child(root, 2) == null!());

    avl_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_avl_tree_free() {
    let mut tree: Owned<AVLTree<Ptr<i32>>>;
    tree = avl_tree_new(func!(int_compare));
    avl_tree_free(tree.unowned());
    tree = create_tree();
    avl_tree_free(tree.unowned());
    test_no_memory_leak!();
}
#[test]
fn test_avl_tree_lookup() {
    unsafe {
        let mut tree: Owned<AVLTree<Ptr<i32>>>;
        let mut i: i32;
        let mut value: Ptr<i32>;
        tree = create_tree();
        c_for!(i = 0; i < NUM_TEST_VALUES!(); i += 1; {
            value = avl_tree_lookup(tree.unowned(), c_ref!(i));
            assert!(value != null!());
            assert!(*value == i);
        });
        i = -1;
        assert!(avl_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        i = NUM_TEST_VALUES!() + 1;
        assert!(avl_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        i = 8724897;
        assert!(avl_tree_lookup(tree.unowned(), c_ref!(i)) == null!());
        avl_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_avl_tree_remove() {
    unsafe {
        let mut tree: Owned<AVLTree<Ptr<i32>>>;
        let mut i: i32;
        let mut x: i32;
        let mut y: i32;
        let mut z: i32;
        let mut value: i32;
        let mut expected_entries: u32;
        tree = create_tree();
        i = NUM_TEST_VALUES!() + 100;
        assert!(avl_tree_remove(tree.unowned(), c_ref!(i)) == 0);
        i = -1;
        assert!(avl_tree_remove(tree.unowned(), c_ref!(i)) == 0);
        expected_entries = NUM_TEST_VALUES!() as u32;
        c_for!(x = 0; x < 10; x += 1; {
            c_for!(y = 0; y < 10; y += 1; {
                c_for!(z = 0; z < 10; z += 1; {
                    value = z * 100 + (9 - y) * 10 + x;
                    assert!(avl_tree_remove(tree.unowned(), c_ref!(value)) != 0);
                    validate_tree(tree.unowned());
                    expected_entries -= 1;
                    assert!(avl_tree_num_entries(tree.unowned()) == expected_entries);
                });
            });
        });
        assert!(avl_tree_root_node(tree.unowned()) == null!());
        avl_tree_free(tree.unowned());
        test_no_memory_leak!();
    }
}
#[test]
fn test_avl_tree_to_array() {
    let mut tree: Owned<AVLTree<Ptr<i32>>>;
    let mut entries: Array<i32, 10> = arr![89, 23, 42, 4, 16, 15, 8, 99, 50, 30];
    let mut sorted: Array<i32, 10> = arr![4, 8, 15, 16, 23, 30, 42, 50, 89, 99];
    let mut num_entries: usize = entries.len();
    let mut i: usize;
    let mut array: Vector<Ptr<i32>>;

    tree = avl_tree_new(func!(int_compare));

    c_for!(i = 0; i < num_entries; i += 1; {
        avl_tree_insert(tree.unowned(), c_ref!(entries[i]), null!());
    });

    assert!(avl_tree_num_entries(tree.unowned()) == num_entries as u32);

    array = avl_tree_to_array(tree.unowned());

    c_for!(i = 0; i < num_entries; i += 1; {
        assert!(*array[i] == sorted[i]);
    });

    c_free!(array);
    avl_tree_free(tree.unowned());
    test_no_memory_leak!();
}
