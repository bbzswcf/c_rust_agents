use crate::translation_utils::*;pub type Set<T: GenericValue> = _Set<T>;
pub type SetIterator<T: GenericValue> = _SetIterator<T>;
pub type SetEntry<T: GenericValue> = _SetEntry<T>;
pub type SetValue<T: GenericValue> = T;

#[derive(Default)]
pub struct _SetIterator<T: GenericValue> {
    pub set: Unowned<Set<T>>,
    pub next_entry: Manual<SetEntry<T>>,
    pub next_chain: u32,
}

#[macro_export]
macro_rules! set_null { () => { null!() } }

pub type SetHashFunc<T: GenericValue> = FuncPtr<fn(SetValue<T>) -> u32>;
pub type SetEqualFunc<T: GenericValue> = FuncPtr<fn(SetValue<T>, SetValue<T>) -> i32>;
pub type SetFreeFunc<T: GenericValue> = FuncPtr<fn(SetValue<T>)>;

#[derive(Default)]
pub struct _SetEntry<T: GenericValue> {
    pub data: SetValue<T>,
    pub next: Manual<SetEntry<T>>,
}

#[derive(Default)]
pub struct _Set<T: GenericValue> {
    pub table: Vector<Manual<SetEntry<T>>>,
    pub entries: u32,
    pub table_size: u32,
    pub prime_index: u32,
    pub hash_func: SetHashFunc<T>,
    pub equal_func: SetEqualFunc<T>,
    pub free_func: SetFreeFunc<T>,
}

pub const set_primes: Array<u32, 24> = arr![
    193, 389, 769, 1543, 3079, 6151, 12289, 24593, 49157, 98317,
    196613, 393241, 786433, 1572869, 3145739, 6291469,
    12582917, 25165843, 50331653, 100663319, 201326611,
    402653189, 805306457, 1610612741,
];

pub const set_num_primes: u32 = set_primes.len() as u32;
pub fn set_allocate_table<T: GenericValue>(mut set: Unowned<Set<T>>) -> i32 {
	if set.prime_index < set_num_primes {
		set.table_size = set_primes[set.prime_index];
	} else {
		set.table_size = set.entries * 10;
	}
	set.table = c_calloc!(set.table_size, c_sizeof!(Manual<SetEntry<T>>));
	return (set.table != null!()) as i32;
}
pub fn set_free_entry<T: GenericValue>(mut set: Unowned<Set<T>>, mut entry: Manual<SetEntry<T>>) {
	if set.free_func != null!() {
		(set.free_func)(entry.data);
	}
	c_free!(entry);
}
pub fn set_new<T: GenericValue>(mut hash_func: SetHashFunc<T>, mut equal_func: SetEqualFunc<T>) -> Owned<Set<T>> {
    let mut new_set: Owned<Set<T>>;
    new_set = c_malloc!(c_sizeof!(Set<T>));
    if new_set == null!() {
        return null!();
    }
    new_set.hash_func = hash_func;
    new_set.equal_func = equal_func;
    new_set.entries = 0;
    new_set.prime_index = 0;
    new_set.free_func = null!();
    if !set_allocate_table(new_set.unowned()).as_bool() {
        c_free!(new_set);
        return null!();
    }
    return new_set;
}
pub fn set_free<T: GenericValue>(mut set: Unowned<Set<T>>) {
    let mut rover: Manual<SetEntry<T>>;
    let mut next: Manual<SetEntry<T>>;
    let mut i: u32;
    c_for!(i = 0; i < set.table_size; i += 1; {
        rover = set.table[i];
        while rover != null!() {
            next = rover.next;
            set_free_entry(set, rover);
            rover = next;
        }
    });
    c_free!(set.table);
    c_free!(set);
}
pub fn set_register_free_function<T: GenericValue>(mut set: Unowned<Set<T>>, mut free_func: SetFreeFunc<T>) {
	set.free_func = free_func;
}
pub fn set_enlarge<T: GenericValue>(mut set: Unowned<Set<T>>) -> i32 {
    let mut rover: Manual<SetEntry<T>>;
    let mut next: Manual<SetEntry<T>>;
    let mut old_table: Vector<Manual<SetEntry<T>>>;
    let mut old_table_size: u32;
    let mut old_prime_index: u32;
    let mut index: u32;
    let mut i: u32;
    old_table = set.table.clone();
    old_table_size = set.table_size;
    old_prime_index = set.prime_index;
    set.prime_index += 1;
    if !set_allocate_table(set).as_bool() {
        set.table = old_table;
        set.table_size = old_table_size;
        set.prime_index = old_prime_index;
        return 0;
    }
    c_for!(i = 0; i < old_table_size; i += 1; {
        rover = old_table[i];
        while rover != null!() {
            next = rover.next;
            index = (set.hash_func)(rover.data) % set.table_size;
            rover.next = set.table[index];
            set.table[index] = rover;
            rover = next;
        }
    });
    c_free!(old_table);
    return 1;
}
pub fn set_insert<T: GenericValue>(mut set: Unowned<Set<T>>, mut data: SetValue<T>) -> i32 {
    let mut newentry: Manual<SetEntry<T>>;
    let mut rover: Manual<SetEntry<T>>;
    let mut index: u32;
    if (set.entries * 3) / set.table_size > 0 {
        if !set_enlarge(set).as_bool() {
            return 0;
        }
    }
    index = (set.hash_func)(data) % set.table_size;
    rover = set.table[index];
    while rover != null!() {
        if (set.equal_func)(data, rover.data) != 0 {
            return 0;
        }
        rover = rover.next;
    }
    newentry = c_malloc!(c_sizeof!(SetEntry<T>));
    if newentry == null!() {
        return 0;
    }
    newentry.data = data;
    newentry.next = set.table[index];
    set.table[index] = newentry;
    set.entries += 1;
    return 1;
}
pub fn set_remove<T: GenericValue>(mut set: Unowned<Set<T>>, mut data: SetValue<T>) -> i32 {
    let mut rover: Ptr<Manual<SetEntry<T>>>;
    let mut entry: Manual<SetEntry<T>>;
    let mut index: u32;
    index = (set.hash_func)(data) % set.table_size;
    rover = c_ref!(set.table[index]);
    while *rover != null!() {
        if (set.equal_func)(data, (*rover).data) != 0 {
            entry = *rover;
            *rover = entry.next;
            set.entries -= 1;
            set_free_entry(set, entry);
            return 1;
        }
        rover = c_ref!((*rover).next);
    }
    return 0;
}
pub fn set_query<T: GenericValue>(mut set: Unowned<Set<T>>, mut data: SetValue<T>) -> i32 {
    let mut rover: Manual<SetEntry<T>>;
    let mut index: u32;
    index = (set.hash_func)(data) % set.table_size;
    rover = set.table[index];
    while rover != null!() {
        if (set.equal_func)(data, rover.data) != 0 {
            return 1;
        }
        rover = rover.next;
    }
    return 0;
}
pub fn set_num_entries<T: GenericValue>(mut set: Unowned<Set<T>>) -> u32 {
    return set.entries;
}
pub fn set_to_array<T: GenericValue>(mut set: Unowned<Set<T>>) -> Vector<SetValue<T>> {
	let mut array: Vector<SetValue<T>>;
	let mut array_counter: i32;
	let mut i: u32;
	let mut rover: Manual<SetEntry<T>>;
	array = c_malloc!(c_sizeof!(SetValue<T>) * set.entries as usize);
	if array == null!() {
		return null!();
	}
	array_counter = 0;
	c_for!(i = 0; i < set.table_size; i += 1; {
		rover = set.table[i];
		while rover != null!() {
			array[array_counter] = rover.data;
			array_counter += 1;
			rover = rover.next;
		}
	});
	return array;
}
pub fn set_union<T: GenericValue>(mut set1: Unowned<Set<T>>, mut set2: Unowned<Set<T>>) -> Owned<Set<T>> {
    let mut iterator: SetIterator<T> = Default::default();
    let mut new_set: Owned<Set<T>>;
    let mut value: SetValue<T>;
    new_set = set_new(set1.hash_func, set1.equal_func);
    if new_set == null!() {
        return null!();
    }
    set_iterate(set1, c_ref!(iterator));
    while set_iter_has_more(c_ref!(iterator)).as_bool() {
        value = set_iter_next(c_ref!(iterator));
        if !set_insert(new_set.unowned(), value).as_bool() {
            set_free(new_set.unowned());
            return null!();
        }
    }
    set_iterate(set2, c_ref!(iterator));
    while set_iter_has_more(c_ref!(iterator)).as_bool() {
        value = set_iter_next(c_ref!(iterator));
        if set_query(new_set.unowned(), value) == 0 {
            if !set_insert(new_set.unowned(), value).as_bool() {
                set_free(new_set.unowned());
                return null!();
            }
        }
    }
    return new_set;
}
pub fn set_intersection<T: GenericValue>(mut set1: Unowned<Set<T>>, mut set2: Unowned<Set<T>>) -> Owned<Set<T>> {
    let mut new_set: Owned<Set<T>>;
    let mut iterator: SetIterator<T> = Default::default();
    let mut value: SetValue<T>;
    new_set = set_new(set1.hash_func, set2.equal_func);
    if new_set == null!() {
        return null!();
    }
    set_iterate(set1, c_ref!(iterator));
    while set_iter_has_more(c_ref!(iterator)).as_bool() {
        value = set_iter_next(c_ref!(iterator));
        if set_query(set2, value) != 0 {
            if !set_insert(new_set.unowned(), value).as_bool() {
                set_free(new_set.unowned());
                return null!();
            }
        }
    }
    return new_set;
}
pub fn set_iterate<T: GenericValue>(mut set: Unowned<Set<T>>, mut iter: Unowned<SetIterator<T>>) {
    let mut chain: u32;
    iter.set = set;
    iter.next_entry = null!();
    c_for!(chain = 0; chain < set.table_size; chain += 1; {
        if set.table[chain] != null!() {
            iter.next_entry = set.table[chain];
            break;
        }
    });
    iter.next_chain = chain;
}
fn set_iter_next<T: GenericValue>(mut iterator: Unowned<SetIterator<T>>) -> SetValue<T> {
    let mut set: Unowned<Set<T>>;
    let mut result: SetValue<T>;
    let mut current_entry: Manual<SetEntry<T>>;
    let mut chain: u32;
    set = iterator.set;
    if iterator.next_entry == null!() {
        return set_null!();
    }
    current_entry = iterator.next_entry;
    result = current_entry.data;
    if current_entry.next != null!() {
        iterator.next_entry = current_entry.next;
    } else {
        iterator.next_entry = null!();
        chain = iterator.next_chain + 1;
        while chain < set.table_size {
            if set.table[chain] != null!() {
                iterator.next_entry = set.table[chain];
                break;
            }
            chain += 1;
        }
        iterator.next_chain = chain;
    }
    return result;
}
pub fn set_iter_has_more<T: GenericValue>(mut iterator: Unowned<SetIterator<T>>) -> i32 {
    return (iterator.next_entry != null!()) as i32;
}
