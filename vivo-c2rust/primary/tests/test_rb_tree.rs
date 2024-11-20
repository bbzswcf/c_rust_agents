#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern crate libc;
use primary::rb_tree::*;
use primary::compare_pointer::*;
use primary::compare_string::*;
use primary::compare_int::*;
use primary::hash_pointer::*;
use primary::hash_string::*;
use primary::hash_int::*;
extern "C" {
    pub type _RBTree;
    pub type _RBTreeNode;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn alloc_test_free(ptr: *mut libc::c_void);
    fn run_tests(tests_0: *mut UnitTestFunction);
    fn rb_tree_new(compare_func: RBTreeCompareFunc) -> *mut RBTree;
    fn rb_tree_free(tree: *mut RBTree);
    fn rb_tree_insert(
        tree: *mut RBTree,
        key: RBTreeKey,
        value: RBTreeValue,
    ) -> *mut RBTreeNode;
    fn rb_tree_remove(tree: *mut RBTree, key: RBTreeKey) -> libc::c_int;
    fn rb_tree_lookup_node(tree: *mut RBTree, key: RBTreeKey) -> *mut RBTreeNode;
    fn rb_tree_lookup(tree: *mut RBTree, key: RBTreeKey) -> RBTreeValue;
    fn rb_tree_root_node(tree: *mut RBTree) -> *mut RBTreeNode;
    fn rb_tree_node_key(node: *mut RBTreeNode) -> RBTreeKey;
    fn rb_tree_node_value(node: *mut RBTreeNode) -> RBTreeValue;
    fn rb_tree_node_child(
        node: *mut RBTreeNode,
        side: RBTreeNodeSide,
    ) -> *mut RBTreeNode;
    fn rb_tree_to_array(tree: *mut RBTree) -> *mut RBTreeValue;
    fn rb_tree_num_entries(tree: *mut RBTree) -> libc::c_int;
    fn int_compare(
        location1: *mut libc::c_void,
        location2: *mut libc::c_void,
    ) -> libc::c_int;
}
pub type UnitTestFunction = Option::<unsafe extern "C" fn() -> ()>;
pub type RBTree = _RBTree;
pub type RBTreeKey = *mut libc::c_void;
pub type RBTreeValue = *mut libc::c_void;
pub type RBTreeNode = _RBTreeNode;
pub type RBTreeCompareFunc = Option::<
    unsafe extern "C" fn(RBTreeValue, RBTreeValue) -> libc::c_int,
>;
pub type RBTreeNodeSide = libc::c_uint;
pub const RB_TREE_NODE_RIGHT: RBTreeNodeSide = 1;
pub const RB_TREE_NODE_LEFT: RBTreeNodeSide = 0;
#[no_mangle]
pub static mut test_array: [libc::c_int; 1000] = [0; 1000];
#[no_mangle]
pub unsafe extern "C" fn find_subtree_height(mut node: *mut RBTreeNode) -> libc::c_int {
    let mut left_subtree: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut right_subtree: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut left_height: libc::c_int = 0;
    let mut right_height: libc::c_int = 0;
    if node.is_null() {
        return 0 as libc::c_int;
    }
    left_subtree = rb_tree_node_child(node, RB_TREE_NODE_LEFT);
    right_subtree = rb_tree_node_child(node, RB_TREE_NODE_RIGHT);
    left_height = find_subtree_height(left_subtree);
    right_height = find_subtree_height(right_subtree);
    if left_height > right_height {
        return left_height + 1 as libc::c_int
    } else {
        return right_height + 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn validate_tree(mut tree: *mut RBTree) {}
#[no_mangle]
pub unsafe extern "C" fn create_tree() -> *mut RBTree {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    let mut i: libc::c_int = 0;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
        rb_tree_insert(
            tree,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeKey,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeValue,
        );
        i += 1;
        i;
    }
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_new() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_rb_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    if (rb_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"rb_tree_root_node(tree) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            110 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_rb_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    if rb_tree_num_entries(tree) == 0 as libc::c_int {} else {
        __assert_fail(
            b"rb_tree_num_entries(tree) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            111 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 28],
                &[libc::c_char; 28],
            >(b"void test_rb_tree_new(void)\0"))
                .as_ptr(),
        );
    };
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_insert_lookup() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    let mut node: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
        rb_tree_insert(
            tree,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeKey,
            &mut *test_array.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeValue,
        );
        if rb_tree_num_entries(tree) == i + 1 as libc::c_int {} else {
            __assert_fail(
                b"rb_tree_num_entries(tree) == i + 1\0" as *const u8
                    as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                141 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_rb_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        validate_tree(tree);
        i += 1;
        i;
    }
    if !(rb_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"rb_tree_root_node(tree) != NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_rb_tree_insert_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        node = rb_tree_lookup_node(tree, &mut i as *mut libc::c_int as RBTreeKey);
        if !node.is_null() {} else {
            __assert_fail(
                b"node != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_rb_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        value = rb_tree_node_key(node) as *mut libc::c_int;
        if *value == i {} else {
            __assert_fail(
                b"*value == i\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                153 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_rb_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        value = rb_tree_node_value(node) as *mut libc::c_int;
        if *value == i {} else {
            __assert_fail(
                b"*value == i\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                155 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void test_rb_tree_insert_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = -(1 as libc::c_int);
    if (rb_tree_lookup_node(tree, &mut i as *mut libc::c_int as RBTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"rb_tree_lookup_node(tree, &i) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_rb_tree_insert_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 1000 as libc::c_int + 100 as libc::c_int;
    if (rb_tree_lookup_node(tree, &mut i as *mut libc::c_int as RBTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"rb_tree_lookup_node(tree, &i) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void test_rb_tree_insert_lookup(void)\0"))
                .as_ptr(),
        );
    };
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_child() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    let mut root: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut left: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut right: *mut RBTreeNode = 0 as *mut RBTreeNode;
    let mut values: [libc::c_int; 3] = [
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
    ];
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
        rb_tree_insert(
            tree,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeKey,
            &mut *values.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeValue,
        );
        i += 1;
        i;
    }
    root = rb_tree_root_node(tree);
    p = rb_tree_node_value(root) as *mut libc::c_int;
    if *p == 2 as libc::c_int {} else {
        __assert_fail(
            b"*p == 2\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_rb_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    left = rb_tree_node_child(root, RB_TREE_NODE_LEFT);
    p = rb_tree_node_value(left) as *mut libc::c_int;
    if *p == 1 as libc::c_int {} else {
        __assert_fail(
            b"*p == 1\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            195 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_rb_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    right = rb_tree_node_child(root, RB_TREE_NODE_RIGHT);
    p = rb_tree_node_value(right) as *mut libc::c_int;
    if *p == 3 as libc::c_int {} else {
        __assert_fail(
            b"*p == 3\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_rb_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    if (rb_tree_node_child(root, 10000 as RBTreeNodeSide)).is_null() {} else {
        __assert_fail(
            b"rb_tree_node_child(root, 10000) == NULL\0" as *const u8
                as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_rb_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    if (rb_tree_node_child(root, 2 as RBTreeNodeSide)).is_null() {} else {
        __assert_fail(
            b"rb_tree_node_child(root, 2) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 30],
                &[libc::c_char; 30],
            >(b"void test_rb_tree_child(void)\0"))
                .as_ptr(),
        );
    };
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_out_of_memory() {}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_free() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
    rb_tree_free(tree);
    tree = create_tree();
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_lookup() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    let mut i: libc::c_int = 0;
    let mut value: *mut libc::c_int = 0 as *mut libc::c_int;
    tree = create_tree();
    i = 0 as libc::c_int;
    while i < 1000 as libc::c_int {
        value = rb_tree_lookup(tree, &mut i as *mut libc::c_int as RBTreeKey)
            as *mut libc::c_int;
        if !value.is_null() {} else {
            __assert_fail(
                b"value != NULL\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                262 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_rb_tree_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        if *value == i {} else {
            __assert_fail(
                b"*value == i\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                263 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"void test_rb_tree_lookup(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    i = -(1 as libc::c_int);
    if (rb_tree_lookup(tree, &mut i as *mut libc::c_int as RBTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"rb_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 1000 as libc::c_int + 1 as libc::c_int;
    if (rb_tree_lookup(tree, &mut i as *mut libc::c_int as RBTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"rb_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            271 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    i = 8724897 as libc::c_int;
    if (rb_tree_lookup(tree, &mut i as *mut libc::c_int as RBTreeKey)).is_null()
    {} else {
        __assert_fail(
            b"rb_tree_lookup(tree, &i) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            273 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_lookup(void)\0"))
                .as_ptr(),
        );
    };
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_remove() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut expected_entries: libc::c_int = 0;
    tree = create_tree();
    i = 1000 as libc::c_int + 100 as libc::c_int;
    if rb_tree_remove(tree, &mut i as *mut libc::c_int as RBTreeKey) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"rb_tree_remove(tree, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            291 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    i = -(1 as libc::c_int);
    if rb_tree_remove(tree, &mut i as *mut libc::c_int as RBTreeKey) == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"rb_tree_remove(tree, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    expected_entries = 1000 as libc::c_int;
    x = 0 as libc::c_int;
    while x < 10 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 10 as libc::c_int {
            z = 0 as libc::c_int;
            while z < 10 as libc::c_int {
                value = z * 100 as libc::c_int
                    + (9 as libc::c_int - y) * 10 as libc::c_int + x;
                if rb_tree_remove(tree, &mut value as *mut libc::c_int as RBTreeKey)
                    != 0 as libc::c_int
                {} else {
                    __assert_fail(
                        b"rb_tree_remove(tree, &value) != 0\0" as *const u8
                            as *const libc::c_char,
                        b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                        306 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 31],
                            &[libc::c_char; 31],
                        >(b"void test_rb_tree_remove(void)\0"))
                            .as_ptr(),
                    );
                };
                validate_tree(tree);
                expected_entries -= 1 as libc::c_int;
                if rb_tree_num_entries(tree) == expected_entries {} else {
                    __assert_fail(
                        b"rb_tree_num_entries(tree) == expected_entries\0" as *const u8
                            as *const libc::c_char,
                        b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                        310 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 31],
                            &[libc::c_char; 31],
                        >(b"void test_rb_tree_remove(void)\0"))
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
    if (rb_tree_root_node(tree)).is_null() {} else {
        __assert_fail(
            b"rb_tree_root_node(tree) == NULL\0" as *const u8 as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"void test_rb_tree_remove(void)\0"))
                .as_ptr(),
        );
    };
    rb_tree_free(tree);
}
#[no_mangle]
pub unsafe extern "C" fn test_rb_tree_to_array() {
    let mut tree: *mut RBTree = 0 as *mut RBTree;
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
    let mut num_entries: libc::c_int = (::core::mem::size_of::<[libc::c_int; 10]>()
        as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut array: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    tree = rb_tree_new(
        ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
            >,
            RBTreeCompareFunc,
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
    while i < num_entries {
        rb_tree_insert(
            tree,
            &mut *entries.as_mut_ptr().offset(i as isize) as *mut libc::c_int
                as RBTreeKey,
            0 as *mut libc::c_void,
        );
        i += 1;
        i;
    }
    if rb_tree_num_entries(tree) == num_entries {} else {
        __assert_fail(
            b"rb_tree_num_entries(tree) == num_entries\0" as *const u8
                as *const libc::c_char,
            b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
            339 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 33],
                &[libc::c_char; 33],
            >(b"void test_rb_tree_to_array(void)\0"))
                .as_ptr(),
        );
    };
    array = rb_tree_to_array(tree) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < num_entries {
        if **array.offset(i as isize) == sorted[i as usize] {} else {
            __assert_fail(
                b"*array[i] == sorted[i]\0" as *const u8 as *const libc::c_char,
                b"test/test-rb-tree.c\0" as *const u8 as *const libc::c_char,
                346 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 33],
                    &[libc::c_char; 33],
                >(b"void test_rb_tree_to_array(void)\0"))
                    .as_ptr(),
            );
        };
        i += 1;
        i;
    }
    alloc_test_free(array as *mut libc::c_void);
    rb_tree_free(tree);
}
static mut tests: [UnitTestFunction; 7] = unsafe {
    [
        Some(test_rb_tree_new as unsafe extern "C" fn() -> ()),
        Some(test_rb_tree_free as unsafe extern "C" fn() -> ()),
        Some(test_rb_tree_child as unsafe extern "C" fn() -> ()),
        Some(test_rb_tree_insert_lookup as unsafe extern "C" fn() -> ()),
        Some(test_rb_tree_lookup as unsafe extern "C" fn() -> ()),
        Some(test_out_of_memory as unsafe extern "C" fn() -> ()),
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


#[test]
fn s_test_rb_tree_new() {
    unsafe {
        test_rb_tree_new();
    }
}


#[test]
fn s_test_rb_tree_insert_lookup() {
    unsafe {
        test_rb_tree_insert_lookup();
    }
}


#[test]
fn s_test_rb_tree_child() {
    unsafe {
        test_rb_tree_child();
    }
}


#[test]
fn s_test_rb_tree_free() {
    unsafe {
        test_rb_tree_free();
    }
}


#[test]
fn s_test_rb_tree_lookup() {
    unsafe {
        test_rb_tree_lookup();
    }
}


#[test]
#[ignore]
fn s_test_rb_tree_remove() {
    unsafe {
        test_rb_tree_remove();
    }
}


#[test]
#[ignore]
fn s_test_rb_tree_to_array() {
    unsafe {
        test_rb_tree_to_array();
    }
}
