#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn alloc_test_malloc(bytes: size_t) -> *mut libc::c_void;
    fn alloc_test_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AVLTree {
    pub root_node: *mut AVLTreeNode,
    pub compare_func: AVLTreeCompareFunc,
    pub num_nodes: libc::c_uint,
}
pub type AVLTreeCompareFunc = Option::<
    unsafe extern "C" fn(AVLTreeValue, AVLTreeValue) -> libc::c_int,
>;
pub type AVLTreeValue = *mut libc::c_void;
pub type AVLTreeNode = _AVLTreeNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _AVLTreeNode {
    pub children: [*mut AVLTreeNode; 2],
    pub parent: *mut AVLTreeNode,
    pub key: AVLTreeKey,
    pub value: AVLTreeValue,
    pub height: libc::c_int,
}
pub type AVLTreeKey = *mut libc::c_void;
pub type AVLTree = _AVLTree;
pub type AVLTreeNodeSide = libc::c_uint;
pub const AVL_TREE_NODE_RIGHT: AVLTreeNodeSide = 1;
pub const AVL_TREE_NODE_LEFT: AVLTreeNodeSide = 0;
#[no_mangle]
pub extern "C" fn avl_tree_new(
    mut compare_func: AVLTreeCompareFunc,
) -> *mut AVLTree {
    let mut new_tree: *mut AVLTree = 0 as *mut AVLTree;
    unsafe {
        new_tree = alloc_test_malloc(::core::mem::size_of::<AVLTree>() as libc::c_ulong)
            as *mut AVLTree;
        if new_tree.is_null() {
            return 0 as *mut AVLTree;
        }
        (*new_tree).root_node = 0 as *mut AVLTreeNode;
        (*new_tree).compare_func = compare_func;
        (*new_tree).num_nodes = 0 as libc::c_int as libc::c_uint;
    }
    new_tree
}
extern "C" fn avl_tree_free_subtree(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
) {
    if node.is_null() {
        return;
    }
    avl_tree_free_subtree(
        tree,
        unsafe { (*node).children[AVL_TREE_NODE_LEFT as libc::c_int as usize] },
    );
    avl_tree_free_subtree(
        tree,
        unsafe { (*node).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize] },
    );
    unsafe { alloc_test_free(node as *mut libc::c_void) };
}
#[no_mangle]
pub extern "C" fn avl_tree_free(mut tree: *mut AVLTree) {
    unsafe {
        avl_tree_free_subtree(tree, (*tree).root_node);
        alloc_test_free(tree as *mut libc::c_void);
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_subtree_height(
    mut node: *mut AVLTreeNode,
) -> libc::c_int {
    if node.is_null() {
        0
    } else {
        unsafe { (*node).height }
    }
}
extern "C" fn avl_tree_update_height(mut node: *mut AVLTreeNode) {
    let mut left_subtree: *mut AVLTreeNode = std::ptr::null_mut();
    let mut right_subtree: *mut AVLTreeNode = std::ptr::null_mut();
    let mut left_height: libc::c_int = 0;
    let mut right_height: libc::c_int = 0;

    unsafe {
        left_subtree = (*node).children[AVL_TREE_NODE_LEFT as libc::c_int as usize];
        right_subtree = (*node).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize];
    }

    left_height = avl_tree_subtree_height(left_subtree);
    right_height = avl_tree_subtree_height(right_subtree);

    unsafe {
        if left_height > right_height {
            (*node).height = left_height + 1;
        } else {
            (*node).height = right_height + 1;
        }
    }
}
extern "C" fn avl_tree_node_parent_side(mut node: *mut AVLTreeNode) -> AVLTreeNodeSide {
    unsafe {
        if (*(*node).parent).children[AVL_TREE_NODE_LEFT as libc::c_int as usize] == node {
            AVL_TREE_NODE_LEFT
        } else {
            AVL_TREE_NODE_RIGHT
        }
    }
}
extern "C" fn avl_tree_node_replace(
    mut tree: *mut AVLTree,
    mut node1: *mut AVLTreeNode,
    mut node2: *mut AVLTreeNode,
) {
    let mut side: libc::c_int = 0;
    if !node2.is_null() {
        unsafe { (*node2).parent = (*node1).parent; }
    }
    if unsafe { (*node1).parent.is_null() } {
        unsafe { (*tree).root_node = node2; }
    } else {
        side = unsafe { avl_tree_node_parent_side(node1) as libc::c_int };
        unsafe { (*(*node1).parent).children[side as usize] = node2; }
        unsafe { avl_tree_update_height((*node1).parent); }
    }
}
extern "C" fn avl_tree_rotate(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
    mut direction: AVLTreeNodeSide,
) -> *mut AVLTreeNode {
    unsafe {
        let mut new_root: *mut AVLTreeNode = (*node).children[(1 - direction as usize) as usize];
        avl_tree_node_replace(tree, node, new_root);
        (*node).children[(1 - direction as usize) as usize] = (*new_root).children[direction as usize];
        (*new_root).children[direction as usize] = node;
        (*node).parent = new_root;
        if !(*node).children[(1 - direction as usize) as usize].is_null() {
            (*(*node).children[(1 - direction as usize) as usize]).parent = node;
        }
        avl_tree_update_height(new_root);
        avl_tree_update_height(node);
        new_root
    }
}
extern "C" fn avl_tree_node_balance(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
) -> *mut AVLTreeNode {
    let mut left_subtree: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut right_subtree: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut child: *mut AVLTreeNode = 0 as *mut AVLTreeNode;
    let mut diff: libc::c_int = 0;

    unsafe {
        left_subtree = (*node).children[AVL_TREE_NODE_LEFT as libc::c_int as usize];
        right_subtree = (*node).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize];
    }

    diff = avl_tree_subtree_height(right_subtree)
        - avl_tree_subtree_height(left_subtree);

    if diff >= 2 {
        child = right_subtree;
        if avl_tree_subtree_height(
            unsafe { (*child).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize] },
        )
            < avl_tree_subtree_height(
                unsafe { (*child).children[AVL_TREE_NODE_LEFT as libc::c_int as usize] },
            )
        {
            avl_tree_rotate(tree, right_subtree, AVL_TREE_NODE_RIGHT);
        }
        node = avl_tree_rotate(tree, node, AVL_TREE_NODE_LEFT);
    } else if diff <= -2 {
        child = unsafe { (*node).children[AVL_TREE_NODE_LEFT as libc::c_int as usize] };
        if avl_tree_subtree_height(
            unsafe { (*child).children[AVL_TREE_NODE_LEFT as libc::c_int as usize] },
        )
            < avl_tree_subtree_height(
                unsafe { (*child).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize] },
            )
        {
            avl_tree_rotate(tree, left_subtree, AVL_TREE_NODE_LEFT);
        }
        node = avl_tree_rotate(tree, node, AVL_TREE_NODE_RIGHT);
    }

    avl_tree_update_height(node);
    node
}
extern "C" fn avl_tree_balance_to_root(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
) {
    let mut rover: *mut AVLTreeNode = node;
    while !rover.is_null() {
        rover = unsafe { avl_tree_node_balance(tree, rover) };
        rover = unsafe { (*rover).parent };
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_insert(
    mut tree: *mut AVLTree,
    mut key: AVLTreeKey,
    mut value: AVLTreeValue,
) -> *mut AVLTreeNode {
    let mut rover: *mut *mut AVLTreeNode = std::ptr::null_mut();
    let mut new_node: *mut AVLTreeNode = std::ptr::null_mut();
    let mut previous_node: *mut AVLTreeNode = std::ptr::null_mut();

    unsafe {
        rover = &mut (*tree).root_node;
        previous_node = std::ptr::null_mut();

        while !(*rover).is_null() {
            previous_node = *rover;
            if ((*tree).compare_func).expect("non-null function pointer")(key, (**rover).key) < 0 {
                rover = &mut (**rover).children[AVL_TREE_NODE_LEFT as usize];
            } else {
                rover = &mut (**rover).children[AVL_TREE_NODE_RIGHT as usize];
            }
        }

        new_node = alloc_test_malloc(std::mem::size_of::<AVLTreeNode>() as libc::c_ulong) as *mut AVLTreeNode;
        if new_node.is_null() {
            return std::ptr::null_mut();
        }

        (*new_node).children[AVL_TREE_NODE_LEFT as usize] = std::ptr::null_mut();
        (*new_node).children[AVL_TREE_NODE_RIGHT as usize] = std::ptr::null_mut();
        (*new_node).parent = previous_node;
        (*new_node).key = key;
        (*new_node).value = value;
        (*new_node).height = 1;
        *rover = new_node;
        avl_tree_balance_to_root(tree, previous_node);
        (*tree).num_nodes = (*tree).num_nodes.wrapping_add(1);
    }

    new_node
}
extern "C" fn avl_tree_node_get_replacement(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
) -> *mut AVLTreeNode {
    let mut left_subtree: *mut AVLTreeNode = std::ptr::null_mut();
    let mut right_subtree: *mut AVLTreeNode = std::ptr::null_mut();
    let mut result: *mut AVLTreeNode = std::ptr::null_mut();
    let mut child: *mut AVLTreeNode = std::ptr::null_mut();
    let mut left_height: libc::c_int = 0;
    let mut right_height: libc::c_int = 0;
    let mut side: libc::c_int = 0;

    unsafe {
        left_subtree = (*node).children[AVL_TREE_NODE_LEFT as usize];
        right_subtree = (*node).children[AVL_TREE_NODE_RIGHT as usize];
    }

    if left_subtree.is_null() && right_subtree.is_null() {
        return std::ptr::null_mut();
    }

    left_height = avl_tree_subtree_height(left_subtree);
    right_height = avl_tree_subtree_height(right_subtree);

    if left_height < right_height {
        side = AVL_TREE_NODE_RIGHT as libc::c_int;
    } else {
        side = AVL_TREE_NODE_LEFT as libc::c_int;
    }

    unsafe {
        result = (*node).children[side as usize];
    }

    while !unsafe { (*result).children[(1 - side) as usize] }.is_null() {
        unsafe {
            result = (*result).children[(1 - side) as usize];
        }
    }

    unsafe {
        child = (*result).children[side as usize];
        avl_tree_node_replace(tree, result, child);
        avl_tree_update_height((*result).parent);
    }

    result
}
#[no_mangle]
pub extern "C" fn avl_tree_remove_node(
    mut tree: *mut AVLTree,
    mut node: *mut AVLTreeNode,
) {
    let mut swap_node: *mut AVLTreeNode = std::ptr::null_mut();
    let mut balance_startpoint: *mut AVLTreeNode = std::ptr::null_mut();
    let mut i: libc::c_int = 0;

    swap_node = avl_tree_node_get_replacement(tree, node);
    if swap_node.is_null() {
        avl_tree_node_replace(tree, node, std::ptr::null_mut());
        balance_startpoint = unsafe { (*node).parent };
    } else {
        if unsafe { (*swap_node).parent } == node {
            balance_startpoint = swap_node;
        } else {
            balance_startpoint = unsafe { (*swap_node).parent };
        }
        i = 0;
        while i < 2 {
            unsafe {
                (*swap_node).children[i as usize] = (*node).children[i as usize];
                if !(*swap_node).children[i as usize].is_null() {
                    (*(*swap_node).children[i as usize]).parent = swap_node;
                }
            }
            i += 1;
        }
        unsafe {
            (*swap_node).height = (*node).height;
        }
        avl_tree_node_replace(tree, node, swap_node);
    }
    unsafe {
        alloc_test_free(node as *mut libc::c_void);
        (*tree).num_nodes = (*tree).num_nodes.wrapping_sub(1);
    }
    avl_tree_balance_to_root(tree, balance_startpoint);
}
#[no_mangle]
pub extern "C" fn avl_tree_lookup_node(
    mut tree: *mut AVLTree,
    mut key: AVLTreeKey,
) -> *mut AVLTreeNode {
    let mut node: *mut AVLTreeNode = std::ptr::null_mut();
    let mut diff: libc::c_int = 0;
    node = unsafe { (*tree).root_node };
    while !node.is_null() {
        diff = unsafe { ((*tree).compare_func).expect("non-null function pointer")(key, (*node).key) };
        if diff == 0 {
            return node;
        } else if diff < 0 {
            node = unsafe { (*node).children[AVL_TREE_NODE_LEFT as usize] };
        } else {
            node = unsafe { (*node).children[AVL_TREE_NODE_RIGHT as usize] };
        }
    }
    std::ptr::null_mut()
}
#[no_mangle]
pub extern "C" fn avl_tree_remove(
    mut tree: *mut AVLTree,
    mut key: AVLTreeKey,
) -> libc::c_int {
    let mut node: *mut AVLTreeNode = avl_tree_lookup_node(tree, key);
    if node.is_null() {
        return 0;
    }
    avl_tree_remove_node(tree, node);
    return 1;
}
#[no_mangle]
pub extern "C" fn avl_tree_lookup(
    mut tree: *mut AVLTree,
    mut key: AVLTreeKey,
) -> AVLTreeValue {
    let mut node: *mut AVLTreeNode = avl_tree_lookup_node(tree, key);
    if node.is_null() {
        0 as *mut libc::c_void
    } else {
        unsafe { (*node).value }
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_root_node(mut tree: *mut AVLTree) -> *mut AVLTreeNode {
    unsafe {
        (*tree).root_node
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_node_key(mut node: *mut AVLTreeNode) -> AVLTreeKey {
    unsafe {
        return (*node).key;
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_node_value(
    mut node: *mut AVLTreeNode,
) -> AVLTreeValue {
    unsafe { (*node).value }
}
#[no_mangle]
pub extern "C" fn avl_tree_node_child(
    mut node: *mut AVLTreeNode,
    mut side: AVLTreeNodeSide,
) -> *mut AVLTreeNode {
    if side as libc::c_uint == AVL_TREE_NODE_LEFT as libc::c_int as libc::c_uint
        || side as libc::c_uint == AVL_TREE_NODE_RIGHT as libc::c_int as libc::c_uint
    {
        return unsafe { (*node).children[side as usize] };
    } else {
        return std::ptr::null_mut();
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_node_parent(
    mut node: *mut AVLTreeNode,
) -> *mut AVLTreeNode {
    unsafe { (*node).parent }
}
#[no_mangle]
pub extern "C" fn avl_tree_num_entries(mut tree: *mut AVLTree) -> libc::c_uint {
    unsafe {
        return (*tree).num_nodes;
    }
}
extern "C" fn avl_tree_to_array_add_subtree(
    mut subtree: *mut AVLTreeNode,
    mut array: *mut AVLTreeValue,
    mut index: *mut libc::c_int,
) {
    if subtree.is_null() {
        return;
    }
    unsafe {
        avl_tree_to_array_add_subtree(
            (*subtree).children[AVL_TREE_NODE_LEFT as libc::c_int as usize],
            array,
            index,
        );
        *array.offset(*index as isize) = (*subtree).key;
        *index += 1;
        avl_tree_to_array_add_subtree(
            (*subtree).children[AVL_TREE_NODE_RIGHT as libc::c_int as usize],
            array,
            index,
        );
    }
}
#[no_mangle]
pub extern "C" fn avl_tree_to_array(mut tree: *mut AVLTree) -> *mut AVLTreeValue {
    let mut array: *mut AVLTreeValue = std::ptr::null_mut();
    let mut index: libc::c_int = 0;
    unsafe {
        array = alloc_test_malloc(
            (std::mem::size_of::<AVLTreeValue>() as libc::c_ulong)
                .wrapping_mul((*tree).num_nodes as libc::c_ulong),
        ) as *mut AVLTreeValue;
        if array.is_null() {
            return std::ptr::null_mut();
        }
    }
    index = 0;
    avl_tree_to_array_add_subtree(unsafe { (*tree).root_node }, array, &mut index);
    array
}
