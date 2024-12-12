use crate::translation_utils::*;pub type BinomialHeapType = i32;
#[macro_export]
macro_rules! binomial_heap_type_min { () => { 0 } }
#[macro_export]
macro_rules! binomial_heap_type_max { () => { 1 } }
pub type BinomialHeapValue<T: GenericValue> = T;
#[macro_export]
macro_rules! binomial_heap_null { () => { null!() } }
pub type BinomialHeapCompareFunc<T: GenericValue> = FuncPtr<fn(BinomialHeapValue<T>, BinomialHeapValue<T>) -> i32>;
pub type BinomialHeap<T: GenericValue> = _BinomialHeap<T>;
pub type BinomialTree<T: GenericValue> = _BinomialTree<T>;

#[derive(Default)]
pub struct _BinomialTree<T: GenericValue> {
    pub value: BinomialHeapValue<T>,
    pub order: u16,
    pub refcount: u16,
    pub subtrees: Vector<Manual<BinomialTree<T>>>,
}

#[derive(Default)]
pub struct _BinomialHeap<T: GenericValue> {
    pub heap_type: BinomialHeapType,
    pub compare_func: BinomialHeapCompareFunc<T>,
    pub num_values: u32,
    pub roots: Vector<Manual<BinomialTree<T>>>,
    pub roots_length: u32,
}
pub fn binomial_heap_cmp<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>, mut data1: BinomialHeapValue<T>, mut data2: BinomialHeapValue<T>) -> i32 {
    if heap.heap_type == binomial_heap_type_min!() {
        return (heap.compare_func)(data1, data2);
    } else {
        return -(heap.compare_func)(data1, data2);
    }
}
pub fn binomial_tree_ref<T: GenericValue>(mut tree: Manual<BinomialTree<T>>) {
    if tree != null!() {
        tree.refcount += 1;
    }
}
pub fn binomial_tree_unref<T: GenericValue>(mut tree: Manual<BinomialTree<T>>) {
    let mut i: i32;
    if tree == null!() {
        return;
    }
    tree.refcount -= 1;
    if tree.refcount == 0 {
        c_for!(i = 0; i < tree.order as i32; i += 1; {
            binomial_tree_unref(tree.subtrees[i]);
        });
        c_free!(tree.subtrees);
        c_free!(tree);
    }
}
pub fn binomial_tree_merge<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>, mut tree1: Manual<BinomialTree<T>>, mut tree2: Manual<BinomialTree<T>>) -> Manual<BinomialTree<T>> {
    let mut new_tree: Manual<BinomialTree<T>>;
    let mut tmp: Manual<BinomialTree<T>>;
    let mut i: i32;
    if binomial_heap_cmp(heap, tree1.value, tree2.value) > 0 {
        tmp = tree1;
        tree1 = tree2;
        tree2 = tmp;
    }
    new_tree = c_malloc!(c_sizeof!(BinomialTree<T>));
    if new_tree == null!() {
        return null!();
    }
    new_tree.refcount = 0;
    new_tree.order = tree1.order + 1;
    new_tree.value = tree1.value;
    new_tree.subtrees = c_malloc!(c_sizeof!(Manual<BinomialTree<T>>) * new_tree.order as usize);
    if new_tree.subtrees == null!() {
        c_free!(new_tree);
        return null!();
    }
    c_memcpy!(new_tree.subtrees.unowned(), tree1.subtrees.unowned(), c_sizeof!(Manual<BinomialTree<T>>) * tree1.order as usize);
    let idx = new_tree.order - 1;
    new_tree.subtrees[idx] = tree2;
    c_for!(i = 0; i < new_tree.order as i32; i += 1; {
        binomial_tree_ref(new_tree.subtrees[i]);
    });
    return new_tree;
}
pub fn binomial_heap_merge_undo<T: GenericValue>(mut new_roots: Vector<Manual<BinomialTree<T>>>, mut count: u32) {
    let mut i: u32;
    c_for!(i = 0; i <= count; i += 1; {
        binomial_tree_unref(new_roots[i]);
    });
    c_free!(new_roots);
}
pub fn binomial_heap_merge<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>, mut other: Unowned<BinomialHeap<T>>) -> i32 {
    let mut new_roots: Vector<Manual<BinomialTree<T>>>;
    let mut new_roots_length: u32;
    let mut vals: Array<Manual<BinomialTree<T>>, 3> = arr![null!(); 3];
    let mut num_vals: i32;
    let mut carry: Manual<BinomialTree<T>>;
    let mut new_carry: Manual<BinomialTree<T>>;
    let mut max: u32;
    let mut i: u32;
    if heap.roots_length > other.roots_length {
        max = heap.roots_length + 1;
    } else {
        max = other.roots_length + 1;
    }
    new_roots = c_malloc!(c_sizeof!(Manual<BinomialTree<T>>) * max as usize);
    if new_roots == null!() {
        return 0;
    }
    new_roots_length = 0;
    carry = null!();
    c_for!(i = 0; i < max; i += 1; {
        num_vals = 0;
        if i < heap.roots_length && heap.roots[i] != null!() {
            vals[num_vals] = heap.roots[i];
            num_vals += 1;
        }
        if i < other.roots_length && other.roots[i] != null!() {
            vals[num_vals] = other.roots[i];
            num_vals += 1;
        }
        if carry != null!() {
            vals[num_vals] = carry;
            num_vals += 1;
        }
        if (num_vals & 1) != 0 {
            new_roots[i] = vals[num_vals - 1];
            binomial_tree_ref(new_roots[i]);
            new_roots_length = i + 1;
        } else {
            new_roots[i] = null!();
        }
        if (num_vals & 2) != 0 {
            new_carry = binomial_tree_merge(heap, vals[0], vals[1]);
            if new_carry == null!() {
                binomial_heap_merge_undo(new_roots, i);
                binomial_tree_unref(carry);
                return 0;
            }
        } else {
            new_carry = null!();
        }
        binomial_tree_unref(carry);
        carry = new_carry;
        binomial_tree_ref(carry);
    });
    c_for!(i = 0; i < heap.roots_length; i += 1; {
        if heap.roots[i] != null!() {
            binomial_tree_unref(heap.roots[i]);
        }
    });
    c_free!(heap.roots);
    heap.roots = new_roots;
    heap.roots_length = new_roots_length;
    return 1;
}
pub fn binomial_heap_new<T: GenericValue>(mut heap_type: BinomialHeapType, mut compare_func: BinomialHeapCompareFunc<T>) -> Owned<BinomialHeap<T>> {
	let mut new_heap: Owned<BinomialHeap<T>>;
	new_heap = c_calloc!(1, c_sizeof!(BinomialHeap<T>));
	if new_heap == null!() {
		return null!();
	}
	new_heap.heap_type = heap_type;
	new_heap.compare_func = compare_func;
	return new_heap;
}
pub fn binomial_heap_free<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>) {
    let mut i: u32;
    c_for!(i = 0; i < heap.roots_length; i += 1; {
        binomial_tree_unref(heap.roots[i]);
    });
    c_free!(heap.roots);
    c_free!(heap);
}
pub fn binomial_heap_insert<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>, mut value: BinomialHeapValue<T>) -> i32 {
    let mut fake_heap: BinomialHeap<T> = Default::default();
    let mut new_tree: Manual<BinomialTree<T>>;
    let mut result: i32;
    new_tree = c_malloc!(c_sizeof!(BinomialTree<T>));
    if new_tree == null!() {
        return 0;
    }
    new_tree.value = value;
    new_tree.order = 0;
    new_tree.refcount = 1;
    new_tree.subtrees = null!();
    fake_heap.heap_type = heap.heap_type;
    fake_heap.compare_func = heap.compare_func;
    fake_heap.num_values = 1;
    fake_heap.roots = c_ref!(new_tree);
    fake_heap.roots_length = 1;
    result = binomial_heap_merge(heap, c_ref!(fake_heap));
    if result != 0 {
        heap.num_values += 1;
    }
    binomial_tree_unref(new_tree);
    return result;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn binomial_heap_num_entries<T: GenericValue>(mut heap: Unowned<BinomialHeap<T>>) -> u32 {
    return heap.num_values;
}
