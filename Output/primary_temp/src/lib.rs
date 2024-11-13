pub type ArrayListValue<T> = Option<T>;

pub struct ArrayList<T> {
    pub data: Vec<ArrayListValue<T>>,
    pub length: u32,
    pub _alloced: u32,
}

pub type ArrayListEqualFunc<T> = fn(ArrayListValue<T>, ArrayListValue<T>) -> i32;

pub type ArrayListCompareFunc<T> = fn(ArrayListValue<T>, ArrayListValue<T>) -> i32;
pub static mut VARIABLE1: i32 = 0;
pub static mut VARIABLE2: i32 = 0;
pub static mut VARIABLE3: i32 = 0;
pub static mut VARIABLE4: i32 = 0;

pub fn arraylist_new<T>(length: usize) -> Option<Box<ArrayList<T>>> {
    let length = if length == 0 { 16 } else { length };

    let new_arraylist = Box::new(ArrayList {
        _alloced: length as u32,
        length: 0,
        data: Vec::with_capacity(length),
    });

    Some(new_arraylist)
}




use std::convert::TryInto;

pub fn arraylist_enlarge<T>(arraylist: &mut ArrayList<T>) -> i32 {
    let newsize = arraylist._alloced * 2;
    let layout = std::alloc::Layout::array::<ArrayListValue<T>>(newsize.try_into().unwrap()).unwrap();
    let data = unsafe {
        std::alloc::realloc(arraylist.data.as_mut_ptr() as *mut u8, layout, layout.size()) as *mut ArrayListValue<T>
    };

    if data.is_null() {
        return 0;
    } else {
        let data_vec = unsafe {
            Vec::from_raw_parts(data, arraylist._alloced.try_into().unwrap(), arraylist._alloced.try_into().unwrap())
        };
        arraylist.data = data_vec;
        arraylist._alloced = newsize;

        return 1;
    }
}




pub fn arraylist_insert<T>(arraylist: &mut ArrayList<T>, index: usize, data: T) -> i32 {
    if index > arraylist.length.try_into().unwrap() {
        return 0;
    }

    if arraylist.length + 1 > arraylist._alloced {
        if arraylist_enlarge(arraylist) == 0 {
            return 0;
        }
    }

    unsafe {
        std::ptr::copy(
            &arraylist.data[index],
            &mut arraylist.data[index + 1],
            (arraylist.length as usize - index).try_into().unwrap(),
        );
    }

    arraylist.data[index] = Some(data);
    arraylist.length += 1;

    return 1;
}




pub fn arraylist_append<T>(arraylist: &mut ArrayList<T>, data: T) -> i32 {
    arraylist_insert(arraylist, arraylist.length.try_into().unwrap(), data)
}




pub fn arraylist_free<T>(arraylist: &mut Option<Box<ArrayList<T>>>) {
    if let Some(al) = arraylist.take() {
        drop(al.data);
    }
}




#[test]
pub fn test_arraylist_append() {
    let mut arraylist: Option<Box<ArrayList<i32>>> = arraylist_new(0);

    assert!(arraylist.as_ref().unwrap().length == 0);

    /* Append some entries */

    assert!(arraylist_append(&mut arraylist, unsafe { VARIABLE1 }) != 0);
    assert!(arraylist.as_ref().unwrap().length == 1);

    assert!(arraylist_append(&mut arraylist, unsafe { VARIABLE2 }) != 0);
    assert!(arraylist.as_ref().unwrap().length == 2);

    assert!(arraylist_append(&mut arraylist, unsafe { VARIABLE3 }) != 0);
    assert!(arraylist.as_ref().unwrap().length == 3);

    assert!(arraylist_append(&mut arraylist, unsafe { VARIABLE4 }) != 0);
    assert!(arraylist.as_ref().unwrap().length == 4);

    assert!(arraylist.as_ref().unwrap().data[0] == Some(unsafe { VARIABLE1 }));
    assert!(arraylist.as_ref().unwrap().data[1] == Some(unsafe { VARIABLE2 }));
    assert!(arraylist.as_ref().unwrap().data[2] == Some(unsafe { VARIABLE3 }));
    assert!(arraylist.as_ref().unwrap().data[3] == Some(unsafe { VARIABLE4 }));

    /* Test appending many entries */

    for _i in 0..10000 { // Prefix with underscore to silence unused variable warning
        assert!(arraylist_append(&mut arraylist, 0) != 0);
    }

    arraylist_free(&mut arraylist);

    /* Test low memory scenario */

    arraylist = arraylist_new(100);
}