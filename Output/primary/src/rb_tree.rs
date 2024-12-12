use crate::translation_utils::*;pub type RBTree<T: GenericValue> = _RBTree<T>;
pub type RBTreeKey<T: GenericValue> = T;
pub type RBTreeValue<T: GenericValue> = T;
#[macro_export]
macro_rules! rb_tree_null { () => { null!() } }
pub type RBTreeNode<T: GenericValue> = _RBTreeNode<T>;
pub type RBTreeCompareFunc<T: GenericValue> = FuncPtr<fn(RBTreeValue<T>, RBTreeValue<T>) -> i32>;
pub type RBTreeNodeColor = i32;
#[macro_export]
macro_rules! rb_tree_node_red { () => { 0 } }
#[macro_export]
macro_rules! rb_tree_node_black { () => { 1 } }
pub type RBTreeNodeSide = i32;
#[macro_export]
macro_rules! rb_tree_node_left { () => { 0 } }
#[macro_export]
macro_rules! rb_tree_node_right { () => { 1 } }

#[derive(Default)]
pub struct _RBTreeNode<T: GenericValue> {
    pub color: RBTreeNodeColor,
    pub key: RBTreeKey<T>,
    pub value: RBTreeValue<T>,
    pub parent: Manual<RBTreeNode<T>>,
    pub children: Array<Manual<RBTreeNode<T>>, 2>,
}

#[derive(Default)]
pub struct _RBTree<T: GenericValue> {
    pub root_node: Manual<RBTreeNode<T>>,
    pub compare_func: RBTreeCompareFunc<T>,
    pub num_nodes: i32,
}
pub fn rb_tree_node_side<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) -> RBTreeNodeSide {
    if node.parent.children[rb_tree_node_left!()] == node {
        return rb_tree_node_left!();
    } else {
        return rb_tree_node_right!();
    }
}
pub fn rb_tree_node_sibling<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) -> Manual<RBTreeNode<T>> {
    let mut side: RBTreeNodeSide;
    side = rb_tree_node_side(node);
    return node.parent.children[1 - side];
}
pub fn rb_tree_node_uncle<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) -> Manual<RBTreeNode<T>> {
    return rb_tree_node_sibling(node.parent);
}
pub fn rb_tree_node_replace<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node1: Manual<RBTreeNode<T>>, mut node2: Manual<RBTreeNode<T>>) {
    let mut side: i32;
    if node2 != null!() {
        node2.parent = node1.parent;
    }
    if node1.parent == null!() {
        tree.root_node = node2;
    } else {
        side = rb_tree_node_side(node1);
        node1.parent.children[side] = node2;
    }
}
pub fn rb_tree_rotate<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>, mut direction: RBTreeNodeSide) -> Manual<RBTreeNode<T>> {
    let mut new_root: Manual<RBTreeNode<T>>;
    new_root = node.children[1 - direction];
    rb_tree_node_replace(tree, node, new_root);
    node.children[1 - direction] = new_root.children[direction];
    new_root.children[direction] = node;
    node.parent = new_root;
    if node.children[1 - direction] != null!() {
        node.children[1 - direction].parent = node;
    }
    return new_root;
}
pub fn rb_tree_new<T: GenericValue>(mut compare_func: RBTreeCompareFunc<T>) -> Owned<RBTree<T>> {
    let mut new_tree: Owned<RBTree<T>>;
    new_tree = c_malloc!(c_sizeof!(RBTree<T>));
    if new_tree == null!() {
        return null!();
    }
    new_tree.root_node = null!();
    new_tree.num_nodes = 0;
    new_tree.compare_func = compare_func;
    return new_tree;
}
pub fn rb_tree_free_subtree<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) {
    if node != null!() {
        rb_tree_free_subtree(node.children[rb_tree_node_left!()]);
        rb_tree_free_subtree(node.children[rb_tree_node_right!()]);
        c_free!(node);
    }
}
pub fn rb_tree_free<T: GenericValue>(mut tree: Unowned<RBTree<T>>) {
    rb_tree_free_subtree(tree.root_node);
    c_free!(tree);
}
pub fn rb_tree_insert_case1<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    if node.parent == null!() {
        node.color = rb_tree_node_black!();
    } else {
        rb_tree_insert_case2(tree, node);
    }
}
pub fn rb_tree_insert_case2<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    if node.parent.color != rb_tree_node_black!() {
        rb_tree_insert_case3(tree, node);
    }
}
pub fn rb_tree_insert_case3<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    let mut grandparent: Manual<RBTreeNode<T>>;
    let mut uncle: Manual<RBTreeNode<T>>;
    grandparent = node.parent.parent;
    uncle = rb_tree_node_uncle(node);
    if uncle != null!() && uncle.color == rb_tree_node_red!() {
        node.parent.color = rb_tree_node_black!();
        uncle.color = rb_tree_node_black!();
        grandparent.color = rb_tree_node_red!();
        rb_tree_insert_case1(tree, grandparent);
    } else {
        rb_tree_insert_case4(tree, node);
    }
}
pub fn rb_tree_insert_case4<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    let mut next_node: Manual<RBTreeNode<T>>;
    let mut side: RBTreeNodeSide;
    side = rb_tree_node_side(node);
    if side != rb_tree_node_side(node.parent) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, 1 - side);
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}
pub fn rb_tree_insert_case5<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    let mut parent: Manual<RBTreeNode<T>>;
    let mut grandparent: Manual<RBTreeNode<T>>;
    let mut side: RBTreeNodeSide;
    parent = node.parent;
    grandparent = parent.parent;
    side = rb_tree_node_side(node);
    rb_tree_rotate(tree, grandparent, 1 - side);
    parent.color = rb_tree_node_black!();
    grandparent.color = rb_tree_node_red!();
}
pub fn rb_tree_insert<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut key: RBTreeKey<T>, mut value: RBTreeValue<T>) -> Manual<RBTreeNode<T>> {
    let mut node: Manual<RBTreeNode<T>>;
    let mut rover: Ptr<Manual<RBTreeNode<T>>>;
    let mut parent: Manual<RBTreeNode<T>>;
    let mut side: RBTreeNodeSide;
    node = c_malloc!(c_sizeof!(RBTreeNode<T>));
    if node == null!() {
        return null!();
    }
    node.key = key;
    node.value = value;
    node.color = rb_tree_node_red!();
    node.children[rb_tree_node_left!()] = null!();
    node.children[rb_tree_node_right!()] = null!();
    parent = null!();
    rover = c_ref!(tree.root_node);
    while *rover != null!() {
        parent = *rover;
        if (tree.compare_func)(value, (*rover).value) < 0 {
            side = rb_tree_node_left!();
        } else {
            side = rb_tree_node_right!();
        }
        rover = c_ref!((*rover).children[side]);
    }
    *rover = node;
    node.parent = parent;
    rb_tree_insert_case1(tree, node);
    tree.num_nodes += 1;
    return node;
}
pub fn rb_tree_lookup_node<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut key: RBTreeKey<T>) -> Manual<RBTreeNode<T>> {
    let mut node: Manual<RBTreeNode<T>>;
    let mut side: RBTreeNodeSide;
    let mut diff: i32;
    node = tree.root_node;
    while node != null!() {
        diff = (tree.compare_func)(key, node.key);
        if diff == 0 {
            return node;
        } else if diff < 0 {
            side = rb_tree_node_left!();
        } else {
            side = rb_tree_node_right!();
        }
        node = node.children[side];
    }
    return null!();
}
pub fn rb_tree_lookup<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut key: RBTreeKey<T>) -> RBTreeValue<T> {
    let mut node: Manual<RBTreeNode<T>>;
    node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return rb_tree_null!();
    } else {
        return node.value;
    }
}
pub fn rb_tree_remove_node<T: GenericValue>(mut tree: Unowned<RBTree<T>>, mut node: Manual<RBTreeNode<T>>) {
    let mut child: Manual<RBTreeNode<T>>;
    let mut next: Manual<RBTreeNode<T>>;
    let mut color: RBTreeNodeColor;
    if node.children[rb_tree_node_left!()].0.is_some() && node.children[rb_tree_node_right!()].0.is_some() {
        next = node.children[rb_tree_node_right!()];
        while next.children[rb_tree_node_left!()].0.is_some() {
            next = next.children[rb_tree_node_left!()];
        }
        let temp_key = node.key;
        let temp_value = node.value;
        node.key = next.key;
        node.value = next.value;
        next.key = temp_key;
        next.value = temp_value;
        node = next;
    }
    child = if node.children[rb_tree_node_left!()].0.is_some() {
        node.children[rb_tree_node_left!()]
    } else {
        node.children[rb_tree_node_right!()]
    };

    if node.color == rb_tree_node_black!() && child.0.is_some() {
        child.color = rb_tree_node_black!();
    }
    color = node.color;
    rb_tree_node_replace(tree, node, child);
    c_free!(node);

    tree.num_nodes -= 1;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node.0.is_none() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn rb_tree_root_node<T: GenericValue>(mut tree: Unowned<RBTree<T>>) -> Manual<RBTreeNode<T>> {
    return tree.root_node;
}
pub fn rb_tree_node_key<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) -> RBTreeKey<T> {
    return node.key;
}
pub fn rb_tree_node_value<T: GenericValue>(mut node: Manual<RBTreeNode<T>>) -> RBTreeValue<T> {
    return node.value;
}
pub fn rb_tree_node_child<T: GenericValue>(mut node: Manual<RBTreeNode<T>>, mut side: RBTreeNodeSide) -> Manual<RBTreeNode<T>> {
    if side == rb_tree_node_left!() || side == rb_tree_node_right!() {
        return node.children[side];
    } else {
        return null!();
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
pub fn rb_tree_to_array<T: GenericValue>(mut tree: Unowned<RBTree<T>>) -> Manual<RBTreeValue<T>> {
    return null!();
}
pub fn rb_tree_num_entries<T: GenericValue>(mut tree: Unowned<RBTree<T>>) -> i32 {
    return tree.num_nodes;
}
