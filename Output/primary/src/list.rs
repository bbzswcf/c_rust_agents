use crate::translation_utils::*;pub type ListEntry<T: GenericValue> = _ListEntry<T>;
pub type ListIterator<T: GenericValue> = _ListIterator<T>;
pub type ListValue<T: GenericValue> = T;

#[derive(Default)]
pub struct _ListIterator<T: GenericValue> {
    pub prev_next: Ptr<Manual<ListEntry<T>>>,
    pub current: Manual<ListEntry<T>>,
}

#[macro_export]
macro_rules! list_null { () => { null!() } }

pub type ListCompareFunc<T: GenericValue> = FuncPtr<fn(ListValue<T>, ListValue<T>) -> i32>;
pub type ListEqualFunc<T: GenericValue> = FuncPtr<fn(ListValue<T>, ListValue<T>) -> i32>;

#[derive(Default)]
pub struct _ListEntry<T: GenericValue> {
    pub data: ListValue<T>,
    pub prev: Manual<ListEntry<T>>,
    pub next: Manual<ListEntry<T>>,
}
pub fn list_free<T: GenericValue>(mut list: Manual<ListEntry<T>>) {
    let mut entry: Manual<ListEntry<T>>;
    entry = list;
    while entry != null!() {
        let mut next: Manual<ListEntry<T>>;
        next = entry.next;
        c_free!(entry);
        entry = next;
    }
}
pub fn list_prepend<T: GenericValue>(mut list: Unowned<Manual<ListEntry<T>>>, mut data: ListValue<T>) -> Manual<ListEntry<T>> {
    let mut newentry: Manual<ListEntry<T>>;
    if list == null!() {
        return null!();
    }
    newentry = c_malloc!(c_sizeof!(ListEntry<T>));
    if newentry == null!() {
        return null!();
    }
    newentry.data = data;
    if *list != null!() {
        (*list).prev = newentry;
    }
    newentry.prev = null!();
    newentry.next = *list;
    *list = newentry;
    return newentry;
}
pub fn list_append<T: GenericValue>(mut list: Unowned<Manual<ListEntry<T>>>, mut data: ListValue<T>) -> Manual<ListEntry<T>> {
    let mut rover: Manual<ListEntry<T>>;
    let mut newentry: Manual<ListEntry<T>>;
    if list == null!() {
        return null!();
    }
    newentry = c_malloc!(c_sizeof!(ListEntry<T>));
    if newentry == null!() {
        return null!();
    }
    newentry.data = data;
    newentry.next = null!();
    if *list == null!() {
        *list = newentry;
        newentry.prev = null!();
    } else {
        c_for!(rover = *list; rover.next != null!(); rover = rover.next; {});
        newentry.prev = rover;
        rover.next = newentry;
    }
    return newentry;
}
pub fn list_data<T: GenericValue>(mut listentry: Manual<ListEntry<T>>) -> ListValue<T> {
    if listentry == null!() {
        return list_null!();
    }
    return listentry.data;
}
pub fn rb_tree_remove<T: GenericValue>(mut tree: Unowned<RBTree<T>>, key: RBTreeKey<T>) -> i32 {
    let mut node = rb_tree_lookup_node(tree, key);
    if node == null!() {
        return 0;
    }
    rb_tree_remove_node(tree, node);
    return 1;
}
pub fn list_prev<T: GenericValue>(mut listentry: Manual<ListEntry<T>>) -> Manual<ListEntry<T>> {
    if listentry == null!() {
        return null!();
    }
    return listentry.prev;
}
pub fn list_next<T: GenericValue>(mut listentry: Manual<ListEntry<T>>) -> Manual<ListEntry<T>> {
    if listentry == null!() {
        return null!();
    }
    return listentry.next;
}
pub fn list_nth_entry<T: GenericValue>(mut list: Manual<ListEntry<T>>, mut n: u32) -> Manual<ListEntry<T>> {
    let mut entry: Manual<ListEntry<T>>;
    let mut i: u32;
    entry = list;
    c_for!(i = 0; i < n; i += 1; {
        if entry == null!() {
            return null!();
        }
        entry = entry.next;
    });
    return entry;
}
pub fn list_nth_data<T: GenericValue>(mut list: Manual<ListEntry<T>>, mut n: u32) -> ListValue<T> {
    let mut entry: Manual<ListEntry<T>>;
    entry = list_nth_entry(list, n);
    if entry == null!() {
        return list_null!();
    } else {
        return entry.data;
    }
}
pub fn list_length<T: GenericValue>(mut list: Manual<ListEntry<T>>) -> u32 {
    let mut entry: Manual<ListEntry<T>>;
    let mut length: u32;
    length = 0;
    entry = list;
    while entry != null!() {
        length += 1;
        entry = entry.next;
    }
    return length;
}
pub fn list_to_array<T: GenericValue>(mut list: Manual<ListEntry<T>>) -> Vector<ListValue<T>> {
    let mut rover: Manual<ListEntry<T>>;
    let mut array: Vector<ListValue<T>>;
    let mut length: u32;
    let mut i: u32;
    length = list_length(list);
    array = c_malloc!(c_sizeof!(ListValue<T>) * length as usize);
    if array == null!() {
        return null!();
    }
    rover = list;
    c_for!(i = 0; i < length; i += 1; {
        array[i] = rover.data;
        rover = rover.next;
    });
    return array;
}
pub fn list_remove_entry<T: GenericValue>(mut list: Ptr<Manual<ListEntry<T>>>, mut entry: Manual<ListEntry<T>>) -> i32 {
    if list == null!() || *list == null!() || entry == null!() {
        return 0;
    }
    if entry.prev == null!() {
        *list = entry.next;
        if entry.next != null!() {
            entry.next.prev = null!();
        }
    } else {
        entry.prev.next = entry.next;
        if entry.next != null!() {
            entry.next.prev = entry.prev;
        }
    }
    c_free!(entry);
    return 1;
}
pub fn list_remove_data<T: GenericValue>(mut list: Ptr<Manual<ListEntry<T>>>, mut callback: ListEqualFunc<T>, mut data: ListValue<T>) -> u32 {
    let mut entries_removed: u32;
    let mut rover: Manual<ListEntry<T>>;
    let mut next: Manual<ListEntry<T>>;
    if list == null!() || callback == null!() {
        return 0;
    }
    entries_removed = 0;
    rover = *list;
    while rover != null!() {
        next = rover.next;
        if (callback)(rover.data, data) != 0 {
            if rover.prev == null!() {
                *list = rover.next;
            } else {
                rover.prev.next = rover.next;
            }
            if rover.next != null!() {
                rover.next.prev = rover.prev;
            }
            c_free!(rover);
            entries_removed += 1;
        }
        rover = next;
    }
    return entries_removed;
}
pub fn list_sort_internal<T: GenericValue>(mut list: Ptr<Manual<ListEntry<T>>>, mut compare_func: ListCompareFunc<T>) -> Manual<ListEntry<T>> {
    let mut pivot: Manual<ListEntry<T>>;
    let mut rover: Manual<ListEntry<T>>;
    let mut less_list: Manual<ListEntry<T>>;
    let mut more_list: Manual<ListEntry<T>>;
    let mut less_list_end: Manual<ListEntry<T>>;
    let mut more_list_end: Manual<ListEntry<T>>;
    if list == null!() || compare_func == null!() {
        return null!();
    }
    if *list == null!() || (*list).next == null!() {
        return *list;
    }
    pivot = *list;
    less_list = null!();
    more_list = null!();
    rover = (*list).next;
    while rover != null!() {
        let mut next: Manual<ListEntry<T>>;
        next = rover.next;
        if (compare_func)(rover.data, pivot.data) < 0 {
            rover.prev = null!();
            rover.next = less_list;
            if less_list != null!() {
                less_list.prev = rover;
            }
            less_list = rover;
        } else {
            rover.prev = null!();
            rover.next = more_list;
            if more_list != null!() {
                more_list.prev = rover;
            }
            more_list = rover;
        }
        rover = next;
    }
    less_list_end = list_sort_internal(c_ref!(less_list), compare_func);
    more_list_end = list_sort_internal(c_ref!(more_list), compare_func);
    *list = less_list;
    if less_list == null!() {
        pivot.prev = null!();
        *list = pivot;
    } else {
        pivot.prev = less_list_end;
        less_list_end.next = pivot;
    }
    pivot.next = more_list;
    if more_list != null!() {
        more_list.prev = pivot;
    }
    if more_list == null!() {
        return pivot;
    } else {
        return more_list_end;
    }
}
pub fn list_sort<T: GenericValue>(mut list: Ptr<Manual<ListEntry<T>>>, mut compare_func: ListCompareFunc<T>) {
    list_sort_internal(list, compare_func);
}
pub fn list_find_data<T: GenericValue>(mut list: Manual<ListEntry<T>>, mut callback: ListEqualFunc<T>, mut data: ListValue<T>) -> Manual<ListEntry<T>> {
    let mut rover: Manual<ListEntry<T>>;
    rover = list;
    c_for!(rover = list; rover != null!(); rover = rover.next; {
        if (callback)(rover.data, data) != 0 {
            return rover;
        }
    });
    return null!();
}
pub fn list_iterate<T: GenericValue>(mut list: Ptr<Manual<ListEntry<T>>>, mut iter: Unowned<ListIterator<T>>) {
    iter.prev_next = list;
    iter.current = null!();
}
pub fn list_iter_has_more<T: GenericValue>(mut iter: Unowned<ListIterator<T>>) -> i32 {
    if iter.current == null!() || iter.current != *iter.prev_next {
        return (*iter.prev_next != null!()) as i32;
    } else {
        return (iter.current.next != null!()) as i32;
    }
}
pub fn list_iter_next<T: GenericValue>(mut iter: Unowned<ListIterator<T>>) -> ListValue<T> {
    if iter.current == null!() || iter.current != *iter.prev_next {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next);
        iter.current = iter.current.next;
    }
    if iter.current == null!() {
        return list_null!();
    } else {
        return iter.current.data;
    }
}
pub fn list_iter_remove<T: GenericValue>(mut iter: Unowned<ListIterator<T>>) {
    if iter.current == null!() || iter.current != *iter.prev_next {
    } else {
        *iter.prev_next = iter.current.next;
        if iter.current.next != null!() {
            iter.current.next.prev = iter.current.prev;
        }
        c_free!(iter.current);
        iter.current = null!();
    }
}
