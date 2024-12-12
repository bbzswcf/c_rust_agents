use crate::translation_utils::*;pub type SortedArrayValue<T: GenericValue> = T;

pub type SortedArray<T: GenericValue> = _SortedArray<T>;

pub type SortedArrayEqualFunc<T: GenericValue> = FuncPtr<fn(SortedArrayValue<T>, SortedArrayValue<T>) -> i32>;

pub type SortedArrayCompareFunc<T: GenericValue> = FuncPtr<fn(SortedArrayValue<T>, SortedArrayValue<T>) -> i32>;

#[derive(Default)]
pub struct _SortedArray<T: GenericValue> {
    pub data: Vector<SortedArrayValue<T>>,
    pub length: u32,
    pub _alloced: u32,
    pub equ_func: SortedArrayEqualFunc<T>,
    pub cmp_func: SortedArrayCompareFunc<T>,
}
pub fn sortedarray_first_index<T: GenericValue>(mut sortedarray: Unowned<SortedArray<T>>, mut data: SortedArrayValue<T>, mut left: u32, mut right: u32) -> u32 {
    let mut index: u32 = left;
    while left < right {
        index = (left + right) / 2;
        let order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index as usize]);
        if order > 0 {
            left = index + 1;
        } else {
            right = index;
        }
    }
    return index;
}
pub fn sortedarray_last_index<T: GenericValue>(mut sortedarray: Unowned<SortedArray<T>>, mut data: SortedArrayValue<T>, mut left: u32, mut right: u32) -> u32 {
    let mut index: u32 = right;
    while left < right {
        index = (left + right) / 2;
        let order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index as usize]);
        if order <= 0 {
            left = index + 1;
        } else {
            right = index;
        }
    }
    return index;
}
pub fn sortedarray_get<T: GenericValue>(mut array: Unowned<SortedArray<T>>, mut i: u32) -> SortedArrayValue<T> {
    if array == null!() {
        return null!();
    }
    return array.data[i];
}
pub fn sortedarray_length<T: GenericValue>(mut array: Unowned<SortedArray<T>>) -> u32 {
    return array.length;
}
pub fn sortedarray_new<T: GenericValue>(mut length: u32, mut equ_func: SortedArrayEqualFunc<T>, mut cmp_func: SortedArrayCompareFunc<T>) -> Owned<SortedArray<T>> {
    if equ_func == null!() || cmp_func == null!() {
        return null!();
    }
    if length == 0 {
        length = 16;
    }
    let mut array: Vector<SortedArrayValue<T>> = c_malloc!(c_sizeof!(SortedArrayValue<T>) * length as usize);
    if array == null!() {
        return null!();
    }
    let mut sortedarray: Owned<SortedArray<T>> = c_malloc!(c_sizeof!(SortedArray<T>));
    if sortedarray == null!() {
        c_free!(array);
        return null!();
    }
    sortedarray.data = array;
    sortedarray.length = 0;
    sortedarray._alloced = length;
    sortedarray.equ_func = equ_func;
    sortedarray.cmp_func = cmp_func;
    return sortedarray;
}
pub fn sortedarray_free<T: GenericValue>(mut sortedarray: Unowned<SortedArray<T>>) {
    if sortedarray != null!() {
        c_free!(sortedarray.data);
        c_free!(sortedarray);
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
pub fn sortedarray_insert<T: GenericValue>(mut sortedarray: Unowned<SortedArray<T>>, mut data: SortedArrayValue<T>) -> i32 {
    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length;
    let mut index: u32 = 0;
    right = if right > 1 { right } else { 0 };
    while left != right {
        index = (left + right) / 2;
        let order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index]);
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = index + 1;
        } else {
            break;
        }
    }
    if sortedarray.length > 0 && (sortedarray.cmp_func)(data, sortedarray.data[index]) > 0 {
        index += 1;
    }
    if sortedarray.length + 1 > sortedarray._alloced {
        let mut newsize: u32;
        let mut data: Vector<SortedArrayValue<T>>;
        newsize = sortedarray._alloced * 2;
        data = c_realloc!(sortedarray.data, c_sizeof!(SortedArrayValue<T>) * newsize as usize);
        if data == null!() {
            return 0;
        } else {
            sortedarray.data = data;
            sortedarray._alloced = newsize;
        }
    }    
    c_memmove!(c_ref!(sortedarray.data[index + 1]), c_ref!(sortedarray.data[index]), (sortedarray.length - index) as usize * c_sizeof!(SortedArrayValue<T>));    
    sortedarray.data[index] = data;
    sortedarray.length += 1;
    return 1;
}
pub fn sortedarray_index_of<T: GenericValue>(mut sortedarray: Unowned<SortedArray<T>>, mut data: SortedArrayValue<T>) -> i32 {
    if sortedarray == null!() {
        return -1;
    }
    let mut left: u32 = 0;
    let mut right: u32 = sortedarray.length;
    let mut index: u32 = 0;
    right = if right > 1 { right } else { 0 };
    while left != right {
        index = (left + right) / 2;
        let order: i32 = (sortedarray.cmp_func)(data, sortedarray.data[index]);
        if order < 0 {
            right = index;
        } else if order > 0 {
            left = index + 1;
        } else {
            left = sortedarray_first_index(sortedarray, data, left, index);
            right = sortedarray_last_index(sortedarray, data, index, right);
            for index in left..=right {
                if (sortedarray.equ_func)(data, sortedarray.data[index]).as_bool() {
                    return index as i32;
                }
            }
            return -1;
        }
    }
    return -1;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
