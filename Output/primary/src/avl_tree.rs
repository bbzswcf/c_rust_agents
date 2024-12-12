use crate::translation_utils::*;pub type AVLTree<T: GenericValue> = _AVLTree<T>;
pub type AVLTreeKey<T: GenericValue> = T;
pub type AVLTreeValue<T: GenericValue> = T;
#[macro_export]
macro_rules! avl_tree_null { () => { null!() } }
pub type AVLTreeNode<T: GenericValue> = _AVLTreeNode<T>;
pub type AVLTreeNodeSide = i32;
#[macro_export]
macro_rules! avl_tree_node_left { () => { 0 } }
#[macro_export]
macro_rules! avl_tree_node_right { () => { 1 } }
pub type AVLTreeCompareFunc<T: GenericValue> = FuncPtr<fn(AVLTreeValue<T>, AVLTreeValue<T>) -> i32>;

#[derive(Default)]
pub struct _AVLTreeNode<T: GenericValue> {
    pub children: Array<Manual<AVLTreeNode<T>>, 2>,
    pub parent: Manual<AVLTreeNode<T>>,
    pub key: AVLTreeKey<T>,
    pub value: AVLTreeValue<T>,
    pub height: i32,
}

#[derive(Default)]
pub struct _AVLTree<T: GenericValue> {
    pub root_node: Manual<AVLTreeNode<T>>,
    pub compare_func: AVLTreeCompareFunc<T>,
    pub num_nodes: u32,
}
pub fn avl_tree_new<T: GenericValue>(mut compare_func: AVLTreeCompareFunc<T>) -> Owned<AVLTree<T>> {
    let mut new_tree: Owned<AVLTree<T>>;
    new_tree = c_malloc!(c_sizeof!(AVLTree<T>));
    if new_tree == null!() {
        return null!();
    }
    new_tree.root_node = null!();
    new_tree.compare_func = compare_func;
    new_tree.num_nodes = 0;
    return new_tree;
}
pub fn avl_tree_free_subtree<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>) {
    if node == null!() {
        return;
    }
    avl_tree_free_subtree(tree, node.children[avl_tree_node_left!()]);
    avl_tree_free_subtree(tree, node.children[avl_tree_node_right!()]);
    c_free!(node);
}
pub fn avl_tree_free<T: GenericValue>(mut tree: Unowned<AVLTree<T>>) {
    avl_tree_free_subtree(tree, tree.root_node);
    c_free!(tree);
}
pub fn avl_tree_subtree_height<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) -> i32 {
    if node == null!() {
        return 0;
    } else {
        return node.height;
    }
}
pub fn avl_tree_update_height<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) {
    let mut left_subtree: Manual<AVLTreeNode<T>>;
    let mut right_subtree: Manual<AVLTreeNode<T>>;
    let mut left_height: i32;
    let mut right_height: i32;
    left_subtree = node.children[avl_tree_node_left!()];
    right_subtree = node.children[avl_tree_node_right!()];
    left_height = avl_tree_subtree_height(left_subtree);
    right_height = avl_tree_subtree_height(right_subtree);
    if left_height > right_height {
        node.height = left_height + 1;
    } else {
        node.height = right_height + 1;
    }
}
pub fn avl_tree_node_parent_side<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) -> AVLTreeNodeSide {
    if node.parent.children[avl_tree_node_left!()] == node {
        return avl_tree_node_left!();
    } else {
        return avl_tree_node_right!();
    }
}
pub fn avl_tree_node_replace<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node1: Manual<AVLTreeNode<T>>, mut node2: Manual<AVLTreeNode<T>>) {
    let mut side: i32;
    if node2 != null!() {
        node2.parent = node1.parent;
    }
    if node1.parent == null!() {
        tree.root_node = node2;
    } else {
        side = avl_tree_node_parent_side(node1);
        node1.parent.children[side] = node2;
        avl_tree_update_height(node1.parent);
    }
}
pub fn avl_tree_rotate<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>, mut direction: AVLTreeNodeSide) -> Manual<AVLTreeNode<T>> {
    let mut new_root: Manual<AVLTreeNode<T>>;
    new_root = node.children[1 - direction];
    avl_tree_node_replace(tree, node, new_root);
    node.children[1 - direction] = new_root.children[direction];
    new_root.children[direction] = node;
    node.parent = new_root;
    if node.children[1 - direction] != null!() {
        node.children[1 - direction].parent = node;
    }
    avl_tree_update_height(new_root);
    avl_tree_update_height(node);
    return new_root;
}
pub fn avl_tree_node_balance<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>) -> Manual<AVLTreeNode<T>> {
    let mut left_subtree: Manual<AVLTreeNode<T>>;
    let mut right_subtree: Manual<AVLTreeNode<T>>;
    let mut child: Manual<AVLTreeNode<T>>;
    let mut diff: i32;
    left_subtree = node.children[avl_tree_node_left!()];
    right_subtree = node.children[avl_tree_node_right!()];
    diff = avl_tree_subtree_height(right_subtree) - avl_tree_subtree_height(left_subtree);
    if diff >= 2 {
        child = right_subtree;
        if avl_tree_subtree_height(child.children[avl_tree_node_right!()]) < avl_tree_subtree_height(child.children[avl_tree_node_left!()]) {
            avl_tree_rotate(tree, right_subtree, avl_tree_node_right!());
        }
        node = avl_tree_rotate(tree, node, avl_tree_node_left!());
    } else if diff <= -2 {
        child = node.children[avl_tree_node_left!()];
        if avl_tree_subtree_height(child.children[avl_tree_node_left!()]) < avl_tree_subtree_height(child.children[avl_tree_node_right!()]) {
            avl_tree_rotate(tree, left_subtree, avl_tree_node_left!());
        }
        node = avl_tree_rotate(tree, node, avl_tree_node_right!());
    }
    avl_tree_update_height(node);
    return node;
}
pub fn avl_tree_balance_to_root<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>) {
    let mut rover: Manual<AVLTreeNode<T>>;
    rover = node;
    while rover != null!() {
        rover = avl_tree_node_balance(tree, rover);
        rover = rover.parent;
    }
}
pub fn avl_tree_insert<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut key: AVLTreeKey<T>, mut value: AVLTreeValue<T>) -> Manual<AVLTreeNode<T>> {
    let mut rover: Ptr<Manual<AVLTreeNode<T>>>;
    let mut new_node: Manual<AVLTreeNode<T>>;
    let mut previous_node: Manual<AVLTreeNode<T>>;
    rover = c_ref!(tree.root_node);
    previous_node = null!();
    while *rover != null!() {
        previous_node = *rover;
        if (tree.compare_func)(key, (*rover).key) < 0 {
            rover = c_ref!((*rover).children[avl_tree_node_left!()]);
        } else {
            rover = c_ref!((*rover).children[avl_tree_node_right!()]);
        }
    }
    new_node = c_malloc!(c_sizeof!(AVLTreeNode<T>));
    if new_node == null!() {
        return null!();
    }
    new_node.children[avl_tree_node_left!()] = null!();
    new_node.children[avl_tree_node_right!()] = null!();
    new_node.parent = previous_node;
    new_node.key = key;
    new_node.value = value;
    new_node.height = 1;
    *rover = new_node;
    avl_tree_balance_to_root(tree, previous_node);
    tree.num_nodes += 1;
    return new_node;
}
pub fn avl_tree_node_get_replacement<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>) -> Manual<AVLTreeNode<T>> {
    let mut left_subtree: Manual<AVLTreeNode<T>>;
    let mut right_subtree: Manual<AVLTreeNode<T>>;
    let mut result: Manual<AVLTreeNode<T>>;
    let mut child: Manual<AVLTreeNode<T>>;
    let mut left_height: i32;
    let mut right_height: i32;
    let mut side: i32;
    left_subtree = node.children[avl_tree_node_left!()];
    right_subtree = node.children[avl_tree_node_right!()];
    if left_subtree == null!() && right_subtree == null!() {
        return null!();
    }
    left_height = avl_tree_subtree_height(left_subtree);
    right_height = avl_tree_subtree_height(right_subtree);
    if left_height < right_height {
        side = avl_tree_node_right!();
    } else {
        side = avl_tree_node_left!();
    }
    result = node.children[side];
    while result.children[1 - side] != null!() {
        result = result.children[1 - side];
    }
    child = result.children[side];
    avl_tree_node_replace(tree, result, child);
    avl_tree_update_height(result.parent);
    return result;
}
pub fn avl_tree_remove_node<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut node: Manual<AVLTreeNode<T>>) {
    let mut swap_node: Manual<AVLTreeNode<T>>;
    let mut balance_startpoint: Manual<AVLTreeNode<T>>;
    let mut i: i32;
    swap_node = avl_tree_node_get_replacement(tree, node);
    if swap_node == null!() {
        avl_tree_node_replace(tree, node, null!());
        balance_startpoint = node.parent;
    } else {
        if swap_node.parent == node {
            balance_startpoint = swap_node;
        } else {
            balance_startpoint = swap_node.parent;
        }
        c_for!(i = 0; i < 2; i += 1; {
            swap_node.children[i] = node.children[i];
            if swap_node.children[i] != null!() {
                swap_node.children[i].parent = swap_node;
            }
        });
        swap_node.height = node.height;
        avl_tree_node_replace(tree, node, swap_node);
    }
    c_free!(node);
    tree.num_nodes -= 1;
    avl_tree_balance_to_root(tree, balance_startpoint);
}
pub fn avl_tree_remove<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut key: AVLTreeKey<T>) -> i32 {
    let mut node: Manual<AVLTreeNode<T>>;
    node = avl_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    avl_tree_remove_node(tree, node);
    return 1;
}
pub fn avl_tree_lookup_node<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut key: AVLTreeKey<T>) -> Manual<AVLTreeNode<T>> {
    let mut node: Manual<AVLTreeNode<T>>;
    let mut diff: i32;
    node = tree.root_node;
    while node != null!() {
        diff = (tree.compare_func)(key, node.key);
        if diff == 0 {
            return node;
        } else if diff < 0 {
            node = node.children[avl_tree_node_left!()];
        } else {
            node = node.children[avl_tree_node_right!()];
        }
    }
    return null!();
}
pub fn avl_tree_lookup<T: GenericValue>(mut tree: Unowned<AVLTree<T>>, mut key: AVLTreeKey<T>) -> AVLTreeValue<T> {
    let mut node: Manual<AVLTreeNode<T>>;
    node = avl_tree_lookup_node(tree, key);
    if node == null!() {
        return avl_tree_null!();
    } else {
        return node.value;
    }
}
pub fn avl_tree_root_node<T: GenericValue>(mut tree: Unowned<AVLTree<T>>) -> Manual<AVLTreeNode<T>> {
    return tree.root_node;
}
pub fn avl_tree_node_key<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) -> AVLTreeKey<T> {
    return node.key;
}
pub fn avl_tree_node_value<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) -> AVLTreeValue<T> {
    return node.value;
}
pub fn avl_tree_node_child<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>, mut side: AVLTreeNodeSide) -> Manual<AVLTreeNode<T>> {
    if side == avl_tree_node_left!() || side == avl_tree_node_right!() {
        return node.children[side];
    } else {
        return null!();
    }
}
pub fn avl_tree_node_parent<T: GenericValue>(mut node: Manual<AVLTreeNode<T>>) -> Manual<AVLTreeNode<T>> {
    return node.parent;
}
pub fn avl_tree_num_entries<T: GenericValue>(mut tree: Unowned<AVLTree<T>>) -> u32 {
    return tree.num_nodes;
}
pub fn avl_tree_to_array_add_subtree<T: GenericValue>(mut subtree: Manual<AVLTreeNode<T>>, mut array: SpanView<AVLTreeValue<T>>, mut index: Ptr<i32>) {
    if subtree == null!() {
        return;
    }
    avl_tree_to_array_add_subtree(subtree.children[avl_tree_node_left!()], array, index);
    array[*index] = subtree.key;
    *index += 1;
    avl_tree_to_array_add_subtree(subtree.children[avl_tree_node_right!()], array, index);
}
pub fn avl_tree_to_array<T: GenericValue>(mut tree: Unowned<AVLTree<T>>) -> Vector<AVLTreeValue<T>> {
    let mut array: Vector<AVLTreeValue<T>>;
    let mut index: i32;
    array = c_malloc!(c_sizeof!(AVLTreeValue<T>) * tree.num_nodes as usize);
    if array == null!() {
        return null!();
    }
    index = 0;
    avl_tree_to_array_add_subtree(tree.root_node, array.unowned(), c_ref!(index));
    return array;
}
