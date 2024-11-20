#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern crate libc;
use primary::avl_tree::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    pub type _AVLTree;
    pub type _AVLTreeNode;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn avl_tree_new(compare_func: AVLTreeCompareFunc) -> *mut AVLTree;
    fn avl_tree_free(tree: *mut AVLTree);
    fn avl_tree_insert(
        tree: *mut AVLTree,
        key: AVLTreeKey,
        value: AVLTreeValue,
    ) -> *mut AVLTreeNode;
    fn avl_tree_remove(tree: *mut AVLTree, key: AVLTreeKey) -> libc::c_int;
    fn avl_tree_lookup_node(tree: *mut AVLTree, key: AVLTreeKey) -> *mut AVLTreeNode;
    fn avl_tree_lookup(tree: *mut AVLTree, key: AVLTreeKey) -> AVLTreeValue;
    fn avl_tree_root_node(tree: *mut AVLTree) -> *mut AVLTreeNode;
    fn avl_tree_node_key(node: *mut AVLTreeNode) -> AVLTreeKey;
    fn avl_tree_node_value(node: *mut AVLTreeNode) -> AVLTreeValue;
    fn avl_tree_node_child(
        node: *mut AVLTreeNode,
        side: AVLTreeNodeSide,
    ) -> *mut AVLTreeNode;
    fn avl_tree_node_parent(node: *mut AVLTreeNode) -> *mut AVLTreeNode;
    fn avl_tree_subtree_height(node: *mut AVLTreeNode) -> libc::c_int;
    fn avl_tree_to_array(tree: *mut AVLTree) -> *mut AVLTreeValue;
    fn avl_tree_num_entries(tree: *mut AVLTree) -> libc::c_uint;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type AVLTree = _AVLTree;
pub type AVLTreeKey = *mut libc::c_void;
pub type AVLTreeValue = *mut libc::c_void;
pub type AVLTreeNode = _AVLTreeNode;
pub type AVLTreeNodeSide = libc::c_uint;
pub const AVL_TREE_NODE_RIGHT: AVLTreeNodeSide = 1;
pub const AVL_TREE_NODE_LEFT: AVLTreeNodeSide = 0;
pub type AVLTreeCompareFunc = Option::<
    unsafe extern "C" fn(AVLTreeValue, AVLTreeValue) -> libc::c_int,
>;
#[no_mangle]
pub static mut test_array: [libc::c_int; 1000] = [0; 1000];
#[no_mangle]
pub unsafe extern "C" fn find_subtree_height(mut node: *mut AVLTreeNode) -> libc::c_int {
    let mut left_subtree: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut right_subtree: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut left_height: libc::c_int = 0;
    let mut right_height: libc::c_int = 0;
    if node.is_null() {
        return 0 as libc::c_int;
    }
    left_subtree = avl_tree_node_child(node, AVL_TREE_NODE_LEFT);
    right_subtree = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT);
    left_height = find_subtree_height(left_subtree);
    right_height = find_subtree_height(right_subtree);
    if left_height > right_height {
        return left_height + 1 as libc::c_int
    } else {
        return right_height + 1 as libc::c_int
    };
}
#[no_mangle]
pub static mut counter: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn validate_subtree(mut node: *mut AVLTreeNode) -> libc::c_int {
    let mut left_node: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut right_node: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut left_height: libc::c_int = 0;
    let mut right_height: libc::c_int = 0;
    let mut key: *mut libc::c_int = 0 as *mut libc::c_int;
    if node.is_null() {
        return 0 as libc::c_int;
    }
    left_node = avl_tree_node_child(node, AVL_TREE_NODE_LEFT);
    right_node = avl_tree_node_child(node, AVL_TREE_NODE_RIGHT);
    if !left_node.is_null() {
        if avl_tree_node_parent(left_node) == node {} else {
            __assert_fail(
                b"avl_tree_node_parent(left_node) == node\0" as *const u8
                    as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                102 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int validate_subtree(AVLTreeNode *)\0"))
                    .as_ptr(),
            );
        };
    }
    if !right_node.is_null() {
        if avl_tree_node_parent(right_node) == node {} else {
            __assert_fail(
                b"avl_tree_node_parent(right_node) == node\0" as *const u8
                    as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                105 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"int validate_subtree(AVLTreeNode *)\0"))
                    .as_ptr(),
            );
        };
    }
    left_height = validate_subtree(left_node);
    key = avl_tree_node_key(node) as *mut libc::c_int;
    if *key > counter {} else {
        __assert_fail(
            b"*key > counter\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int validate_subtree(AVLTreeNode *)\0"))
                .as_ptr(),
        );
    };
    counter = *key;
    right_height = validate_subtree(right_node);
    if avl_tree_subtree_height(left_node) == left_height {} else {
        __assert_fail(
            b"avl_tree_subtree_height(left_node) == left_height\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int validate_subtree(AVLTreeNode *)\0"))
                .as_ptr(),
        );
    };
    if avl_tree_subtree_height(right_node) == right_height {} else {
        __assert_fail(
            b"avl_tree_subtree_height(right_node) == right_height\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int validate_subtree(AVLTreeNode *)\0"))
                .as_ptr(),
        );
    };
    if left_height - right_height < 2 as libc::c_int
        && right_height - left_height < 2 as libc::c_int
    {} else {
        __assert_fail(
            b"left_height - right_height < 2 && right_height - left_height < 2\0"
                as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"int validate_subtree(AVLTreeNode *)\0"))
                .as_ptr(),
        );
    };
    if left_height > right_height {
        return left_height + 1 as libc::c_int
    } else {
        return right_height + 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn validate_tree(mut tree: *mut AVLTree) {
    let mut root_node: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut height: libc::c_int = 0;
    root_node = avl_tree_root_node(tree);
    if !root_node.is_null() {
        height = find_subtree_height(root_node);
        if avl_tree_subtree_height(root_node) == height {} else {
            __assert_fail(
                b"avl_tree_subtree_height(root_node) == height\0" as *const u8
                    as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 30],
                    &[libc::c_char; 30],
                >(b"void validate_tree(AVLTree *)\0"))
                    .as_ptr(),
            );
        };
    }
    counter = -(1 as libc::c_int);
    validate_subtree(root_node);
}
#[no_mangle]
pub unsafe extern "C" fn create_tree() -> *mut AVLTree {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut i: libc::c_int = 0;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        test_array[i as usize] = i;
        avl_tree_insert(
            tree,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeKey,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeValue,
        );
        i += 1;
        i;
    }
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_new() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    if !tree.is_null() {} else {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_avl_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    if (avl_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"avl_tree_root_node(tree) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_avl_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    if avl_tree_num_entries(tree) == 0 as libc::c_int as libc::c_uint {} else {
        __assert_fail(
            b"avl_tree_num_entries(tree) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 29],
                &[libc::c_char; 29],
            >(b"void test_avl_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_insert_lookup() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut node: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut i: libc::c_uint = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < 1000 as libc::c_int as libc::c_uint {
        test_array[i as usize] = i as libc::c_int;
        avl_tree_insert(
            tree,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeKey,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeValue,
        );
        if avl_tree_num_entries(tree) == i.wrapping_add(1 as libc::c_int as libc::c_uint)
        {} else {
            __assert_fail(
                b"avl_tree_num_entries(tree) == i + 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                213 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_avl_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        validate_tree(tree);
        i = i.wrapping_add(1);
        i;
    }
    if !(avl_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"avl_tree_root_node(tree) != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_avl_tree_insert_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int as libc::c_uint;
    while i < 1000 as libc::c_int as libc::c_uint {
        node = avl_tree_lookup_node(tree, &mut i as *mut libc::c_uint as AVLTreeKey);
        if !node.is_null() {} else {
            __assert_fail(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                223 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_avl_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        value = avl_tree_node_key(node) as *mut libc::c_int;
        if *value == i as libc::c_int {} else {
            __assert_fail(
                b"*value == (int) i\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_avl_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        value = avl_tree_node_value(node) as *mut libc::c_int;
        if *value == i as libc::c_int {} else {
            __assert_fail(
                b"*value == (int) i\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                227 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[libc::c_char; 39],
                >(b"void test_avl_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    i = (1000 as libc::c_int + 100 as libc::c_int) as libc::c_uint;
    if (avl_tree_lookup_node(tree, &mut i as *mut libc::c_uint as AVLTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"avl_tree_lookup_node(tree, &i) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            233 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[libc::c_char; 39],
            >(b"void test_avl_tree_insert_lookup(void)\0"))
                .as_ptr(),
        );
    };
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_child() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut root: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut left: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut right: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut values: [libc::c_int; 3] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        avl_tree_insert(
            tree,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeKey,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeValue,
        );
        i += 1;
        i;
    }
    root = avl_tree_root_node(tree);
    p = avl_tree_node_value(root) as *mut libc::c_int;
    if *p == 2 as libc::c_int {} else {
        __assert_fail(
            b"*p == 2\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            261 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_avl_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    left = avl_tree_node_child(root, AVL_TREE_NODE_LEFT);
    p = avl_tree_node_value(left) as *mut libc::c_int;
    if *p == 1 as libc::c_int {} else {
        __assert_fail(
            b"*p == 1\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            265 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_avl_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    right = avl_tree_node_child(root, AVL_TREE_NODE_RIGHT);
    p = avl_tree_node_value(right) as *mut libc::c_int;
    if *p == 3 as libc::c_int {} else {
        __assert_fail(
            b"*p == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_avl_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    if (avl_tree_node_child(root, 10000 as AVLTreeNodeSide)).is_null() {} else {
        __assert_fail(
            b"avl_tree_node_child(root, 10000) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_avl_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    if (avl_tree_node_child(root, 2 as AVLTreeNodeSide)).is_null() {} else {
        __assert_fail(
            b"avl_tree_node_child(root, 2) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_avl_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_out_of_memory() {}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_free() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    avl_tree_free(tree);
    tree = create_tree();
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_lookup() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    tree = create_tree();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        value = avl_tree_lookup(tree, &mut i as *mut libc::c_int as AVLTreeKey)
            as *mut libc::c_int;
        if !value.is_null() {} else {
            __assert_fail(
                b"value != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                332 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_avl_tree_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        if *value == i {} else {
            __assert_fail(
                b"*value == i\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                333 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void test_avl_tree_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = -(1 as libc::c_int);
    if (avl_tree_lookup(tree, &mut i as *mut libc::c_int as AVLTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"avl_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 1000 as libc::c_int + 1 as libc::c_int;
    if (avl_tree_lookup(tree, &mut i as *mut libc::c_int as AVLTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"avl_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            341 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 8724897 as libc::c_int;
    if (avl_tree_lookup(tree, &mut i as *mut libc::c_int as AVLTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"avl_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            343 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_remove() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut expected_entries: libc::c_uint = 0;
    tree = create_tree();
    i = 1000 as libc::c_int + 100 as libc::c_int;
    if avl_tree_remove(tree, &mut i as *mut libc::c_int as AVLTreeKey)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"avl_tree_remove(tree, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    i = -(1 as libc::c_int);
    if avl_tree_remove(tree, &mut i as *mut libc::c_int as AVLTreeKey)
        == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"avl_tree_remove(tree, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    expected_entries = 1000 as libc::c_int as libc::c_uint;
    x = 0 as libc::c_int;
    while x < 10 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 10 as libc::c_int {
            z = 0 as libc::c_int;
            while z < 10 as libc::c_int {
                value = z * 100 as libc::c_int
                    + (9 as libc::c_int - y) * 10 as libc::c_int + x;
                if avl_tree_remove(tree, &mut value as *mut libc::c_int as AVLTreeKey)
                    != 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"avl_tree_remove(tree, &value) != 0\0" as *const u8
                            as *const libc::c_char,
                        b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                        376 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"void test_avl_tree_remove(void)\0"))
                            .as_ptr(),
                    );
                };
                validate_tree(tree);
                expected_entries = expected_entries
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
                if avl_tree_num_entries(tree) == expected_entries {} else {
                    __assert_fail(
                        b"avl_tree_num_entries(tree) == expected_entries\0" as *const u8
                            as *const libc::c_char,
                        b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                        380 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 32],
                            &[libc::c_char; 32],
                        >(b"void test_avl_tree_remove(void)\0"))
                            .as_ptr(),
                    );
                };
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    if (avl_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"avl_tree_root_node(tree) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            387 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void test_avl_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    avl_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_avl_tree_to_array() {
    let mut tree: *mut AVLTree = 0 as *mut AVLTree;
    let mut entries: [libc::c_int; 10] = [
        89 as libc::c_int,
        23 as libc::c_int,
        42 as libc::c_int,
        4 as libc::c_int,
        16 as libc::c_int,
        15 as libc::c_int,
        8 as libc::c_int,
        99 as libc::c_int,
        50 as libc::c_int,
        30 as libc::c_int,
    ];
    let mut sorted: [libc::c_int; 10] = [
        4 as libc::c_int,
        8 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
        23 as libc::c_int,
        30 as libc::c_int,
        42 as libc::c_int,
        50 as libc::c_int,
        89 as libc::c_int,
        99 as libc::c_int,
    ];
    let mut num_entries: libc::c_uint = (::core::mem::size_of::<[libc::c_int; 10]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_uint;
    let mut i: libc::c_uint = 0;
    let mut array: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    tree = avl_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            AVLTreeCompareFunc,
        >(
            Some(
                int_compare
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ),
    );
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        avl_tree_insert(
            tree,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as AVLTreeKey,
            0 as *mut libc::c_void,
        );
        i = i.wrapping_add(1);
        i;
    }
    if avl_tree_num_entries(tree) == num_entries {} else {
        __assert_fail(
            b"avl_tree_num_entries(tree) == num_entries\0" as *const u8
                as *const libc::c_char,
            b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
            409 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 34],
                &[libc::c_char; 34],
            >(b"void test_avl_tree_to_array(void)\0"))
                .as_ptr(),
        );
    };
    array = avl_tree_to_array(tree) as *mut *mut libc::c_int;
    i = 0 as libc::c_int as libc::c_uint;
    while i < num_entries {
        if **array.offset(i as isize) == sorted[i as usize] {} else {
            __assert_fail(
                b"*array[i] == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test/test-avl-tree.c\0" as *const u8 as *const libc::c_char,
                416 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 34],
                    &[libc::c_char; 34],
                >(b"void test_avl_tree_to_array(void)\0"))
                    .as_ptr(),
            );
        };
        i = i.wrapping_add(1);
        i;
    }
    alloc_test_free(array as *mut libc::c_void);
}
static mut tests: [UnitTestFunction; 8] = unsafe {
    [
        Some(test_avl_tree_new as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_free as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_child as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_insert_lookup as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_lookup as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_remove as unsafe extern "C" fn() -> ()),
        Some(test_avl_tree_to_array as unsafe extern "C" fn() -> ()),
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














