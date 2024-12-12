use crate::translation_utils::*;pub type HashTable<T: GenericValue> = _HashTable<T>;
pub type HashTableIterator<T: GenericValue> = _HashTableIterator<T>;
pub type HashTableEntry<T: GenericValue> = _HashTableEntry<T>;
pub type HashTableKey<T: GenericValue> = T;
pub type HashTableValue<T: GenericValue> = T;

#[derive(Default, Clone, Copy)]
pub struct HashTablePair<T: GenericValue> {
    pub key: HashTableKey<T>,
    pub value: HashTableValue<T>,
}

#[derive(Default)]
pub struct _HashTableIterator<T: GenericValue> {
    pub hash_table: Unowned<HashTable<T>>,
    pub next_entry: Manual<HashTableEntry<T>>,
    pub next_chain: u32,
}

#[macro_export]
macro_rules! hash_table_null { () => { null!() } }

pub type HashTableHashFunc<T: GenericValue> = FuncPtr<fn(HashTableKey<T>) -> u32>;
pub type HashTableEqualFunc<T: GenericValue> = FuncPtr<fn(HashTableKey<T>, HashTableKey<T>) -> i32>;
pub type HashTableKeyFreeFunc<T: GenericValue> = FuncPtr<fn(HashTableKey<T>)>;
pub type HashTableValueFreeFunc<T: GenericValue> = FuncPtr<fn(HashTableValue<T>)>;

#[derive(Default)]
pub struct _HashTableEntry<T: GenericValue> {
    pub pair: HashTablePair<T>,
    pub next: Manual<HashTableEntry<T>>,
}

#[derive(Default)]
pub struct _HashTable<T: GenericValue> {
    pub table: Vector<Manual<HashTableEntry<T>>>,
    pub table_size: u32,
    pub hash_func: HashTableHashFunc<T>,
    pub equal_func: HashTableEqualFunc<T>,
    pub key_free_func: HashTableKeyFreeFunc<T>,
    pub value_free_func: HashTableValueFreeFunc<T>,
    pub entries: u32,
    pub prime_index: u32,
}

pub const hash_table_primes: Array<u32, 24> = arr![
    193, 389, 769, 1543, 3079, 6151, 12289, 24593, 49157, 98317,
    196613, 393241, 786433, 1572869, 3145739, 6291469,
    12582917, 25165843, 50331653, 100663319, 201326611,
    402653189, 805306457, 1610612741,
];

pub const hash_table_num_primes: u32 = hash_table_primes.len() as u32;
pub fn hash_table_allocate_table<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>) -> i32 {
    let mut new_table_size: u32;
    if hash_table.prime_index < hash_table_num_primes {
        new_table_size = hash_table_primes[hash_table.prime_index];
    } else {
        new_table_size = hash_table.entries * 10;
    }
    hash_table.table_size = new_table_size;
    hash_table.table = c_calloc!(hash_table.table_size, c_sizeof!(Manual<HashTableEntry<T>>));
    return (hash_table.table != null!()) as i32;
}
pub fn hash_table_free_entry<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut entry: Manual<HashTableEntry<T>>) {
	let mut pair: Ptr<HashTablePair<T>>;
	pair = c_ref!(entry.pair);
	if hash_table.key_free_func != null!() {
		(hash_table.key_free_func)(pair.key);
	}
	if hash_table.value_free_func != null!() {
		(hash_table.value_free_func)(pair.value);
	}
	c_free!(entry);
}
pub fn hash_table_new<T: GenericValue>(mut hash_func: HashTableHashFunc<T>, mut equal_func: HashTableEqualFunc<T>) -> Owned<HashTable<T>> {
    let mut hash_table: Owned<HashTable<T>>;
    hash_table = c_malloc!(c_sizeof!(HashTable<T>));
    if hash_table == null!() {
        return null!();
    }
    hash_table.hash_func = hash_func;
    hash_table.equal_func = equal_func;
    hash_table.key_free_func = null!();
    hash_table.value_free_func = null!();
    hash_table.entries = 0;
    hash_table.prime_index = 0;
    if !hash_table_allocate_table(hash_table.unowned()).as_bool() {
        c_free!(hash_table);
        return null!();
    }
    return hash_table;
}
pub fn hash_table_free<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>) {
	let mut rover: Manual<HashTableEntry<T>>;
	let mut next: Manual<HashTableEntry<T>>;
	let mut i: u32;
	c_for!(i = 0; i < hash_table.table_size; i += 1; {
		rover = hash_table.table[i];
		while rover != null!() {
			next = rover.next;
			hash_table_free_entry(hash_table, rover);
			rover = next;
		}
	});
	c_free!(hash_table.table);
	c_free!(hash_table);
}
pub fn hash_table_register_free_functions<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut key_free_func: HashTableKeyFreeFunc<T>, mut value_free_func: HashTableValueFreeFunc<T>) {
	hash_table.key_free_func = key_free_func;
	hash_table.value_free_func = value_free_func;
}
pub fn hash_table_enlarge<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>) -> i32 {
    let mut old_table: Vector<Manual<HashTableEntry<T>>>;
    let mut old_table_size: u32;
    let mut old_prime_index: u32;
    let mut rover: Manual<HashTableEntry<T>>;
    let mut pair: Ptr<HashTablePair<T>>;
    let mut next: Manual<HashTableEntry<T>>;
    let mut index: u32;
    let mut i: u32;
    old_table = hash_table.table.clone();
    old_table_size = hash_table.table_size;
    old_prime_index = hash_table.prime_index;
    hash_table.prime_index += 1;
    if !hash_table_allocate_table(hash_table).as_bool() {
        hash_table.table = old_table;
        hash_table.table_size = old_table_size;
        hash_table.prime_index = old_prime_index;
        return 0;
    }
    c_for!(i = 0; i < old_table_size; i += 1; {
        rover = old_table[i];
        while rover != null!() {
            next = rover.next;
            pair = c_ref!(rover.pair);
            index = (hash_table.hash_func)(pair.key) % hash_table.table_size;
            rover.next = hash_table.table[index];
            hash_table.table[index] = rover;
            rover = next;
        }
    });
    c_free!(old_table);
    return 1;
}
pub fn hash_table_insert<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut key: HashTableKey<T>, mut value: HashTableValue<T>) -> i32 {
    let mut rover: Manual<HashTableEntry<T>>;
    let mut pair: Ptr<HashTablePair<T>>;
    let mut newentry: Manual<HashTableEntry<T>>;
    let mut index: u32;
    if (hash_table.entries * 3) / hash_table.table_size > 0 {
        if !hash_table_enlarge(hash_table).as_bool() {
            return 0;
        }
    }
    index = (hash_table.hash_func)(key) % hash_table.table_size;
    rover = hash_table.table[index];
    while rover != null!() {
        pair = c_ref!(rover.pair);
        if (hash_table.equal_func)(pair.key, key) != 0 {
            if hash_table.value_free_func != null!() {
                (hash_table.value_free_func)(pair.value);
            }
            if hash_table.key_free_func != null!() {
                (hash_table.key_free_func)(pair.key);
            }
            pair.key = key;
            pair.value = value;
            return 1;
        }
        rover = rover.next;
    }
    newentry = c_malloc!(c_sizeof!(HashTableEntry<T>));
    if newentry == null!() {
        return 0;
    }
    newentry.pair.key = key;
    newentry.pair.value = value;
    newentry.next = hash_table.table[index];
    hash_table.table[index] = newentry;
    hash_table.entries += 1;
    return 1;
}
pub fn hash_table_lookup<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut key: HashTableKey<T>) -> HashTableValue<T> {
    let mut rover: Manual<HashTableEntry<T>>;
    let mut pair: Ptr<HashTablePair<T>>;
    let mut index: u32;
    index = (hash_table.hash_func)(key) % hash_table.table_size;
    rover = hash_table.table[index];
    while rover != null!() {
        pair = c_ref!(rover.pair);
        if (hash_table.equal_func)(key, pair.key) != 0 {
            return pair.value;
        }
        rover = rover.next;
    }
    return hash_table_null!();
}
pub fn hash_table_remove<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut key: HashTableKey<T>) -> i32 {
    let mut rover: Ptr<Manual<HashTableEntry<T>>>;
    let mut entry: Manual<HashTableEntry<T>>;
    let mut pair: Ptr<HashTablePair<T>>;
    let mut index: u32;
    let mut result: i32;
    result = 0;
    index = (hash_table.hash_func)(key) % hash_table.table_size;
    rover = c_ref!(hash_table.table[index]);
    while *rover != null!() {
        pair = c_ref!((*rover).pair);
        if (hash_table.equal_func)(key, pair.key) != 0 {
            entry = *rover;
            *rover = entry.next;
            hash_table_free_entry(hash_table, entry);
            hash_table.entries -= 1;
            result = 1;
            break;
        }
        rover = c_ref!((*rover).next);
    }
    return result;
}
pub fn hash_table_num_entries<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>) -> u32 {
    return hash_table.entries;
}
pub fn hash_table_iterate<T: GenericValue>(mut hash_table: Unowned<HashTable<T>>, mut iterator: Unowned<HashTableIterator<T>>) {
	let mut chain: u32;
	iterator.hash_table = hash_table;
	iterator.next_entry = null!();
	c_for!(chain = 0; chain < hash_table.table_size; chain += 1; {
		if hash_table.table[chain] != null!() {
			iterator.next_entry = hash_table.table[chain];
			iterator.next_chain = chain;
			break;
		}
	});
}
pub fn hash_table_iter_has_more<T: GenericValue>(mut iterator: Unowned<HashTableIterator<T>>) -> i32 {
    return (iterator.next_entry != null!()) as i32;
}
pub fn hash_table_iter_next<T: GenericValue>(mut iterator: Unowned<HashTableIterator<T>>) -> HashTablePair<T> {
    let mut current_entry: Manual<HashTableEntry<T>>;
    let mut hash_table: Unowned<HashTable<T>>;
    let mut pair: HashTablePair<T> = HashTablePair { key: null!(), value: null!() };
    let mut chain: u32;
    hash_table = iterator.hash_table;
    if iterator.next_entry == null!() {
        return pair;
    }
    current_entry = iterator.next_entry;
    pair = current_entry.pair;
    if current_entry.next != null!() {
        iterator.next_entry = current_entry.next;
    } else {
        chain = iterator.next_chain + 1;
        iterator.next_entry = null!();
        while chain < hash_table.table_size {
            if hash_table.table[chain] != null!() {
                iterator.next_entry = hash_table.table[chain];
                break;
            }
            chain += 1;
        }
        iterator.next_chain = chain;
    }
    return pair;
}
