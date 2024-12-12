use crate::trie::*;
use crate::translation_utils::*;macro_rules! NUM_TEST_VALUES { () => { 10000 } }

static mut test_array: Array<i32, {NUM_TEST_VALUES!()}> = arr![0; NUM_TEST_VALUES!()];
static mut test_strings: Array<CStr, {NUM_TEST_VALUES!()}> = arr![cstr!(""); NUM_TEST_VALUES!()];

static mut bin_key: Array<u8, 7> = arr![b'a', b'b', b'c', b'\0', 1, 2, 0xff];
static mut bin_key2: Array<u8, 8> = arr![b'a', b'b', b'c', b'\0', 1, 2, 0xff, b'\0'];
static mut bin_key3: Array<u8, 3> = arr![b'a', b'b', b'c'];
static mut bin_key4: Array<u8, 4> = arr![b'z', b'\0', b'z', b'z'];

macro_rules! LONG_STRING_LEN { () => { 4096 } }

fn main() {
    let mut free_list: Manual<TrieNode<T>> = Manual::new(TrieNode::new());
    *list = result.data.into_manual();
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
#[test]
fn test_trie_free_long() {
    unsafe {
        let mut long_string: Owned<CStr>;
        let mut trie: Owned<Trie<CStr>>;
        long_string = c_malloc!(LONG_STRING_LEN!());
        c_memset!(long_string.unowned().as_ptr(), b'A', LONG_STRING_LEN!());
        long_string[LONG_STRING_LEN!() - 1] = b'\0';
        trie = trie_new();
        trie_insert(trie.unowned(), *long_string.unowned(), *long_string.unowned());
        trie_free(trie.unowned());
        c_free!(long_string);
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
