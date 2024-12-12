use crate::translation_utils::*;pub type ArrayListValue<T: GenericValue> = T;

pub type ArrayList<T: GenericValue> = _ArrayList<T>;

#[derive(Default)]
pub struct _ArrayList<T: GenericValue> {
    pub data: Vector<ArrayListValue<T>>,
    pub length: u32,
    pub _alloced: u32,
}

pub type ArrayListEqualFunc<T: GenericValue> = FuncPtr<fn(ArrayListValue<T>, ArrayListValue<T>) -> i32>;

pub type ArrayListCompareFunc<T: GenericValue> = FuncPtr<fn(ArrayListValue<T>, ArrayListValue<T>) -> i32>;
pub fn arraylist_new<T: GenericValue>(mut length: u32) -> Owned<ArrayList<T>> {
    let mut new_arraylist: Owned<ArrayList<T>>;
    if length == 0 {
        length = 16;
    }
    new_arraylist = c_malloc!(c_sizeof!(ArrayList<T>));
    if new_arraylist == null!() {
        return null!();
    }
    new_arraylist._alloced = length;
    new_arraylist.length = 0;
    new_arraylist.data = c_malloc!(length as usize * c_sizeof!(ArrayListValue<T>));
    if new_arraylist.data == null!() {
        c_free!(new_arraylist);
        return null!();
    }
    return new_arraylist;
}
pub fn arraylist_free<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>) {
    if arraylist != null!() {
        c_free!(arraylist.data);
        c_free!(arraylist);
    }
}
pub fn arraylist_enlarge<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>) -> i32 {
    let mut data: Vector<ArrayListValue<T>>;
    let mut newsize: u32;
    newsize = arraylist._alloced * 2;
    data = c_realloc!(arraylist.data, c_sizeof!(ArrayListValue<T>) * newsize as usize);
    if data == null!() {
        return 0;
    } else {
        arraylist.data = data;
        arraylist._alloced = newsize;
        return 1;
    }
}
pub fn arraylist_insert<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut index: u32, mut data: ArrayListValue<T>) -> i32 {
    if index > arraylist.length {
        return 0;
    }
    if arraylist.length + 1 > arraylist._alloced {
        if !arraylist_enlarge(arraylist).as_bool() {
            return 0;
        }
    }
    c_memmove!(c_ref!(arraylist.data[index + 1]), c_ref!(arraylist.data[index]), (arraylist.length - index) as usize * c_sizeof!(ArrayListValue<T>));
    arraylist.data[index] = data;
    arraylist.length += 1;
    return 1;
}
pub fn arraylist_append<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut data: ArrayListValue<T>) -> i32 {
    return arraylist_insert(arraylist, arraylist.length, data);
}
pub fn arraylist_prepend<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut data: ArrayListValue<T>) -> i32 {
    return arraylist_insert(arraylist, 0, data);
}
pub fn arraylist_remove_range<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut index: u32, mut length: u32) {
    if index > arraylist.length || index + length > arraylist.length {
        return;
    }
    c_memmove!(c_ref!(arraylist.data[index]), c_ref!(arraylist.data[index + length]), (arraylist.length - (index + length)) as usize * c_sizeof!(ArrayListValue<T>));
    arraylist.length -= length;
}
pub fn arraylist_remove<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut index: u32) {
    arraylist_remove_range(arraylist, index, 1);
}
pub fn arraylist_index_of<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut callback: ArrayListEqualFunc<T>, mut data: ArrayListValue<T>) -> i32 {
    let mut i: u32;
    i = 0;
    while i < arraylist.length {
        if callback(arraylist.data[i], data) != 0 {
            return i as i32;
        }
        i += 1;
    }
    return -1;
}
pub fn arraylist_clear<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>) {
    arraylist.length = 0;
}
pub fn arraylist_sort_internal<T: GenericValue>(mut list_data: SpanView<ArrayListValue<T>>, mut list_length: u32, mut compare_func: ArrayListCompareFunc<T>) {
    let mut pivot: ArrayListValue<T>;
    let mut tmp: ArrayListValue<T>;
    let mut i: u32;
    let mut list1_length: u32;
    let mut list2_length: u32;
    if list_length <= 1 {
        return;
    }
    pivot = list_data[list_length - 1];
    list1_length = 0;
    i = 0;
    while i < list_length - 1 {
        if compare_func(list_data[i], pivot) < 0 {
            tmp = list_data[i];
            list_data[i] = list_data[list1_length];
            list_data[list1_length] = tmp;
            list1_length += 1;
        }
        i += 1;
    }
    list2_length = list_length - list1_length - 1;
    list_data[list_length - 1] = list_data[list1_length];
    list_data[list1_length] = pivot;
    arraylist_sort_internal(list_data, list1_length, compare_func);
    arraylist_sort_internal(c_ref!(list_data[list1_length + 1]), list2_length, compare_func);
}
pub fn arraylist_sort<T: GenericValue>(mut arraylist: Unowned<ArrayList<T>>, mut compare_func: ArrayListCompareFunc<T>) {
    arraylist_sort_internal(arraylist.data.unowned(), arraylist.length, compare_func);
}
