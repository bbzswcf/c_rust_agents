
pub type ArrayListValue<T> = Option<T>;

pub struct ArrayList<T> {
    pub data: Vec<ArrayListValue<T>>,
    pub length: u32,
    pub _alloced: u32,
}

pub type ArrayListEqualFunc<T> = fn(ArrayListValue<T>, ArrayListValue<T>) -> i32;

pub type ArrayListCompareFunc<T> = fn(ArrayListValue<T>, ArrayListValue<T>) -> i32;
pub fn arraylist_new(length: usize) -> Option<Box<ArrayList>> {
    let length = if length <= 0 { 16 } else { length };

    let new_arraylist = Box::new(ArrayList {
        _alloced: length,
        length: 0,
        data: Vec::with_capacity(length),
    });

    Some(new_arraylist)
}