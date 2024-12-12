use crate::translation_utils::*;pub type Trie<T: GenericValue> = _Trie<T>;
pub type TrieValue<T: GenericValue> = T;

#[macro_export]
macro_rules! trie_null { () => { null!() } }

pub type TrieNode<T: GenericValue> = _TrieNode<T>;

#[derive(Default)]
pub struct _TrieNode<T: GenericValue> {
    pub data: TrieValue<T>,
    pub use_count: u32,
    pub next: Array<Manual<TrieNode<T>>, 256>,
}

#[derive(Default)]
pub struct _Trie<T: GenericValue> {
    pub root_node: Manual<TrieNode<T>>,
}
pub fn trie_new<T: GenericValue>() -> Owned<Trie<T>> {
    let mut new_trie: Owned<Trie<T>>;
    new_trie = c_malloc!(c_sizeof!(Trie<T>));
    if new_trie == null!() {
        return null!();
    }
    new_trie.root_node = null!();
    return new_trie;
}
fn trie_free_list_push<T: GenericValue>(list: &mut Manual<TrieNode<T>>, mut node: Manual<TrieNode<T>>) {
    node.data = trie_null!();
    node.next[0] = *list;
    *list = node;
}
fn trie_free_list_pop<T: GenericValue>(list: &mut Manual<TrieNode<T>>) -> Manual<TrieNode<T>> {
    let result = *list;
    if result != null!() {
        *list = result.data.into_manual();
    }
    result
}
pub fn trie_free<T: GenericValue>(mut trie: Unowned<Trie<T>>) {
    if trie.0.is_none() {
        return;
    }

    let mut free_list: Manual<TrieNode<CStr>> = Manual::new(TrieNode {
        data: trie_null!(),
        use_count: 0,
        next: arr![Manual::new(None); 256],
    });
    
    if trie.root_node.0.is_some() {
        trie_free_list_push(&mut free_list, trie.root_node);
    }

    while free_list.0.is_some() {
        let mut node = trie_free_list_pop(&mut free_list);
        
        for i in 0..256 {
            if node.next[i].0.is_some() {
                trie_free_list_push(&mut free_list, node.next[i]);
            }
        }
        
        c_free!(node);
    }

    c_free!(trie);
}
pub fn trie_find_end<T: GenericValue>(mut trie: Unowned<Trie<T>>, mut key: CStr) -> Manual<TrieNode<T>> {
    let mut node: Manual<TrieNode<T>>;
    let mut p: CStr;
    node = trie.root_node;
    c_for!(p = key.clone(); *p != b'\0'; p += 1; {
        if node == null!() {
            return null!();
        }
        node = node.next[*p as usize];
    });
    return node;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn trie_insert_rollback<T: GenericValue>(mut trie: Unowned<Trie<T>>, mut key: SpanView<u8>) {
    let mut node: Manual<TrieNode<T>>;
    let mut prev_ptr: Ptr<Manual<TrieNode<T>>>;
    let mut next_node: Manual<TrieNode<T>>;
    let mut next_prev_ptr: Ptr<Manual<TrieNode<T>>>;
    let mut p: SpanView<u8>;
    node = trie.root_node;
    prev_ptr = c_ref!(trie.root_node);
    p = key;
    while node != null!() {
        next_prev_ptr = c_ref!(node.next[*p]);
        next_node = *next_prev_ptr;
        p += 1;
        node.use_count -= 1;
        if node.use_count == 0 {
            c_free!(node);
            if prev_ptr != null!() {
                *prev_ptr = null!();
            }
            next_prev_ptr = null!();
        }
        node = next_node;
        prev_ptr = next_prev_ptr;
    }
}
pub fn trie_insert<T: GenericValue>(mut trie: Unowned<Trie<T>>, mut key: CStr, mut value: TrieValue<T>) -> i32 {
    let mut rover: Ptr<Manual<TrieNode<T>>>;
    let mut node: Manual<TrieNode<T>>;
    let mut p: CStr;
    let mut c: i32;
    if value == trie_null!() {
        return 0;
    }
    node = trie_find_end(trie, key);
    if node != null!() && node.data != trie_null!() {
        node.data = value;
        return 1;
    }
    rover = c_ref!(trie.root_node);
    p = key.clone();
    loop {
        node = *rover;
        if node == null!() {
            node = c_calloc!(1, c_sizeof!(TrieNode<T>));
            if node == null!() {
                trie_insert_rollback(trie, key.to_view());
                return 0;
            }
            node.data = trie_null!();
            *rover = node;
        }
        node.use_count += 1;
        c = *p as i32;
        if c == b'\0' as i32 {
            node.data = value;
            break;
        }
        rover = c_ref!(node.next[c]);
        p += 1;
    }
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
use crate::translation_utils::GenericValue;
use crate::translation_utils::rb_tree_lookup_node;
use crate::translation_utils::rb_tree_remove_node;

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
use crate::translation_utils::GenericValue;
use crate::translation_utils::rb_tree_lookup_node;
use crate::translation_utils::rb_tree_remove_node;

pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node.is_null() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
