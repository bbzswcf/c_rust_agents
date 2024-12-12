use crate::translation_utils::*;pub type SListEntry<T: GenericValue> = _SListEntry<T>;
pub type SListIterator<T: GenericValue> = _SListIterator<T>;
pub type SListValue<T: GenericValue> = T;

#[derive(Default)]
pub struct _SListIterator<T: GenericValue> {
    pub prev_next: Ptr<Manual<SListEntry<T>>>,
    pub current: Manual<SListEntry<T>>,
}

#[macro_export]
macro_rules! slist_null { () => { null!() } }

pub type SListCompareFunc<T: GenericValue> = FuncPtr<fn(SListValue<T>, SListValue<T>) -> i32>;
pub type SListEqualFunc<T: GenericValue> = FuncPtr<fn(SListValue<T>, SListValue<T>) -> i32>;

#[derive(Default)]
pub struct _SListEntry<T: GenericValue> {
    pub data: SListValue<T>,
    pub next: Manual<SListEntry<T>>,
}
pub fn slist_free<T: GenericValue>(mut list: Manual<SListEntry<T>>) {
    let mut entry: Manual<SListEntry<T>>;
    entry = list;
    while entry != null!() {
        let mut next: Manual<SListEntry<T>>;
        next = entry.next;
        c_free!(entry);
        entry = next;
    }
}
pub fn slist_prepend<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut data: SListValue<T>) -> Manual<SListEntry<T>> {
    let mut newentry: Manual<SListEntry<T>>;
    newentry = c_malloc!(c_sizeof!(SListEntry<T>));
    if newentry == null!() {
        return null!();
    }
    newentry.data = data;
    newentry.next = *list;
    *list = newentry;
    return newentry;
}
pub fn slist_append<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut data: SListValue<T>) -> Manual<SListEntry<T>> {
    let mut rover: Manual<SListEntry<T>>;
    let mut newentry: Manual<SListEntry<T>>;
    newentry = c_malloc!(c_sizeof!(SListEntry<T>));
    if newentry == null!() {
        return null!();
    }
    newentry.data = data;
    newentry.next = null!();
    if *list == null!() {
        *list = newentry;
    } else {
        c_for!(rover = *list; rover.next != null!(); rover = rover.next; {});
        rover.next = newentry;
    }
    return newentry;
}
pub fn slist_data<T: GenericValue>(mut listentry: Manual<SListEntry<T>>) -> SListValue<T> {
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
pub fn slist_next<T: GenericValue>(mut listentry: Manual<SListEntry<T>>) -> Manual<SListEntry<T>> {
    return listentry.next;
}
pub fn slist_nth_entry<T: GenericValue>(mut list: Manual<SListEntry<T>>, mut n: u32) -> Manual<SListEntry<T>> {
    let mut entry: Manual<SListEntry<T>>;
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
pub fn slist_nth_data<T: GenericValue>(mut list: Manual<SListEntry<T>>, mut n: u32) -> SListValue<T> {
    let mut entry: Manual<SListEntry<T>>;
    entry = slist_nth_entry(list, n);
    if entry == null!() {
        return slist_null!();
    } else {
        return entry.data;
    }
}
pub fn slist_length<T: GenericValue>(mut list: Manual<SListEntry<T>>) -> u32 {
    let mut entry: Manual<SListEntry<T>>;
    let mut length: u32;
    length = 0;
    entry = list;
    while entry != null!() {
        length += 1;
        entry = entry.next;
    }
    return length;
}
pub fn slist_to_array<T: GenericValue>(mut list: Manual<SListEntry<T>>) -> Vector<SListValue<T>> {
    let mut rover: Manual<SListEntry<T>>;
    let mut array: Vector<SListValue<T>>;
    let mut length: u32;
    let mut i: u32;
    length = slist_length(list);
    array = c_malloc!(c_sizeof!(SListValue<T>) * length as usize);
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
pub fn slist_remove_entry<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut entry: Manual<SListEntry<T>>) -> i32 {
    let mut rover: Manual<SListEntry<T>>;
    if *list == null!() || entry == null!() {
        return 0;
    }
    if *list == entry {
        *list = entry.next;
    } else {
        rover = *list;
        while rover != null!() && rover.next != entry {
            rover = rover.next;
        }
        if rover == null!() {
            return 0;
        } else {
            rover.next = entry.next;
        }
    }
    c_free!(entry);
    return 1;
}
pub fn slist_remove_data<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut callback: SListEqualFunc<T>, mut data: SListValue<T>) -> u32 {
    let mut rover: Ptr<Manual<SListEntry<T>>>;
    let mut next: Manual<SListEntry<T>>;
    let mut entries_removed: u32;
    entries_removed = 0;
    rover = list;
    while *rover != null!() {
        if (callback)((*rover).data, data) != 0 {
            next = (*rover).next;
            c_free!(*rover);
            *rover = next;
            entries_removed += 1;
        } else {
            rover = c_ref!((*rover).next);
        }
    }
    return entries_removed;
}
pub fn slist_sort_internal<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut compare_func: SListCompareFunc<T>) -> Manual<SListEntry<T>> {
    let mut pivot: Manual<SListEntry<T>>;
    let mut rover: Manual<SListEntry<T>>;
    let mut less_list: Manual<SListEntry<T>>;
    let mut more_list: Manual<SListEntry<T>>;
    let mut less_list_end: Manual<SListEntry<T>>;
    let mut more_list_end: Manual<SListEntry<T>>;
    if *list == null!() || (*list).next == null!() {
        return *list;
    }
    pivot = *list;
    less_list = null!();
    more_list = null!();
    rover = (*list).next;
    while rover != null!() {
        let mut next: Manual<SListEntry<T>>;
        next = rover.next;
        if (compare_func)(rover.data, pivot.data) < 0 {
            rover.next = less_list;
            less_list = rover;
        } else {
            rover.next = more_list;
            more_list = rover;
        }
        rover = next;
    }
    less_list_end = slist_sort_internal(c_ref!(less_list), compare_func);
    more_list_end = slist_sort_internal(c_ref!(more_list), compare_func);
    *list = less_list;
    if less_list == null!() {
        *list = pivot;
    } else {
        less_list_end.next = pivot;
    }
    pivot.next = more_list;
    if more_list == null!() {
        return pivot;
    } else {
        return more_list_end;
    }
}
pub fn slist_sort<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut compare_func: SListCompareFunc<T>) {
    slist_sort_internal(list, compare_func);
}
pub fn slist_find_data<T: GenericValue>(mut list: Manual<SListEntry<T>>, mut callback: SListEqualFunc<T>, mut data: SListValue<T>) -> Manual<SListEntry<T>> {
    let mut rover: Manual<SListEntry<T>>;
    rover = list;
    c_for!(rover = list; rover != null!(); rover = rover.next; {
        if (callback)(rover.data, data) != 0 {
            return rover;
        }
    });
    return null!();
}
pub fn slist_iterate<T: GenericValue>(mut list: Ptr<Manual<SListEntry<T>>>, mut iter: Unowned<SListIterator<T>>) {
    iter.prev_next = list;
    iter.current = null!();
}
pub fn slist_iter_has_more<T: GenericValue>(mut iter: Unowned<SListIterator<T>>) -> i32 {
    if iter.current == null!() || iter.current != *iter.prev_next {
        return (*iter.prev_next != null!()) as i32;
    } else {
        return (iter.current.next != null!()) as i32;
    }
}
pub fn slist_iter_next<T: GenericValue>(mut iter: Unowned<SListIterator<T>>) -> SListValue<T> {
    if iter.current == null!() || iter.current != *iter.prev_next {
        iter.current = *iter.prev_next;
    } else {
        iter.prev_next = c_ref!(iter.current.next);
        iter.current = iter.current.next;
    }
    if iter.current == null!() {
        return slist_null!();
    } else {
        return iter.current.data;
    }
}
pub fn slist_iter_remove<T: GenericValue>(mut iter: Unowned<SListIterator<T>>) {
    if iter.current == null!() || iter.current != *iter.prev_next {
    } else {
        *iter.prev_next = iter.current.next;
        c_free!(iter.current);
        iter.current = null!();
    }
}
