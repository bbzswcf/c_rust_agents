use crate::translation_utils::*;pub type BloomFilter<T: GenericValue> = _BloomFilter<T>;
pub type BloomFilterValue<T: GenericValue> = T;
pub type BloomFilterHashFunc<T: GenericValue> = FuncPtr<fn(BloomFilterValue<T>) -> u32>;

#[derive(Default)]
pub struct _BloomFilter<T: GenericValue> {
    pub hash_func: BloomFilterHashFunc<T>,
    pub table: Vector<u8>,
    pub table_size: u32,
    pub num_functions: u32,
}

const SALTS: Array<u32, 64> = arr![
    0x1953c322, 0x588ccf17, 0x64bf600c, 0xa6be3f3d,
    0x341a02ea, 0x15b03217, 0x3b062858, 0x5956fd06,
    0x18b5624f, 0xe3be0b46, 0x20ffcd5c, 0xa35dfd2b,
    0x1fc4a9bf, 0x57c45d5c, 0xa8661c4a, 0x4f1b74d2,
    0x5a6dde13, 0x3b18dac6, 0x05a8afbf, 0xbbda2fe2,
    0xa2520d78, 0xe7934849, 0xd541bc75, 0x09a55b57,
    0x9b345ae2, 0xfc2d26af, 0x38679cef, 0x81bd1e0d,
    0x654681ae, 0x4b3d87ad, 0xd5ff10fb, 0x23b32f67,
    0xafc7e366, 0xdd955ead, 0xe7c34b1c, 0xfeace0a6,
    0xeb16f09d, 0x3c57a72d, 0x2c8294c5, 0xba92662a,
    0xcd5b2d14, 0x743936c8, 0x2489beff, 0xc6c56e00,
    0x74a4f606, 0xb244a94a, 0x5edfc423, 0xf1901934,
    0x24af7691, 0xf6c98b25, 0xea25af46, 0x76d5f2e6,
    0x5e33cdf2, 0x445eb357, 0x88556bd2, 0x70d1da7a,
    0x54449368, 0x381020bc, 0x1c0520bf, 0xf7e44942,
    0xa27e2a58, 0x66866fc5, 0x12519ce7, 0x437a8456,
];
pub fn bloom_filter_new<T: GenericValue>(mut table_size: u32, mut hash_func: BloomFilterHashFunc<T>, mut num_functions: u32) -> Owned<BloomFilter<T>> {
	let mut filter: Owned<BloomFilter<T>>;
	if num_functions > SALTS.len() as u32 {
		return null!();
	}
	filter = c_malloc!(c_sizeof!(BloomFilter<T>));
	if filter == null!() {
		return null!();
	}
	filter.table = c_calloc!((table_size + 7) / 8, 1);
	if filter.table == null!() {
		c_free!(filter);
		return null!();
	}
	filter.hash_func = hash_func;
	filter.num_functions = num_functions;
	filter.table_size = table_size;
	return filter;
}
pub fn bloom_filter_free<T: GenericValue>(mut bloomfilter: Unowned<BloomFilter<T>>) {
    c_free!(bloomfilter.table);
    c_free!(bloomfilter);
}
pub fn bloom_filter_insert<T: GenericValue>(mut bloomfilter: Unowned<BloomFilter<T>>, mut value: BloomFilterValue<T>) {
    let mut hash: u32;
    let mut subhash: u32;
    let mut index: u32;
    let mut i: u32;
    let mut b: u8;
    hash = (bloomfilter.hash_func)(value);
    c_for!(i = 0; i < bloomfilter.num_functions; i += 1; {
        subhash = hash ^ SALTS[i as usize];
        index = subhash % bloomfilter.table_size;
        b = (1 << (index % 8)) as u8;
        bloomfilter.table[(index / 8) as usize] |= b;
    });
}
pub fn bloom_filter_query<T: GenericValue>(mut bloomfilter: Unowned<BloomFilter<T>>, mut value: BloomFilterValue<T>) -> i32 {
    let mut hash: u32;
    let mut subhash: u32;
    let mut index: u32;
    let mut i: u32;
    let mut b: u8;
    let mut bit: i32;
    hash = (bloomfilter.hash_func)(value);
    c_for!(i = 0; i < bloomfilter.num_functions; i += 1; {
        subhash = hash ^ SALTS[i as usize];
        index = subhash % bloomfilter.table_size;
        b = bloomfilter.table[(index / 8) as usize];
        bit = 1 << (index % 8);
        if (b & bit as u8) == 0 {
            return 0;
        }
    });
    return 1;
}
pub fn bloom_filter_read<T: GenericValue>(mut bloomfilter: Unowned<BloomFilter<T>>, mut array: SpanView<u8>) {
    let mut array_size: u32;
    array_size = (bloomfilter.table_size + 7) / 8;
    c_memcpy!(array, bloomfilter.table.unowned(), array_size);
}
pub fn bloom_filter_load<T: GenericValue>(mut bloomfilter: Unowned<BloomFilter<T>>, mut array: SpanView<u8>) {
    let mut array_size: u32;
    array_size = (bloomfilter.table_size + 7) / 8;
    c_memcpy!(bloomfilter.table.unowned(), array, array_size);
}
pub fn bloom_filter_union<T: GenericValue>(mut filter1: Unowned<BloomFilter<T>>, mut filter2: Unowned<BloomFilter<T>>) -> Owned<BloomFilter<T>> {
    let mut result: Owned<BloomFilter<T>>;
    let mut i: u32;
    let mut array_size: u32;
    if filter1.table_size != filter2.table_size
        || filter1.num_functions != filter2.num_functions
        || filter1.hash_func != filter2.hash_func {
        return null!();
    }
    result = bloom_filter_new(filter1.table_size, filter1.hash_func, filter1.num_functions);
    if result == null!() {
        return null!();
    }
    array_size = (filter1.table_size + 7) / 8;
    c_for!(i = 0; i < array_size; i += 1; {
        result.table[i] = filter1.table[i] | filter2.table[i];
    });
    return result;
}
pub fn bloom_filter_intersection<T: GenericValue>(mut filter1: Unowned<BloomFilter<T>>, mut filter2: Unowned<BloomFilter<T>>) -> Owned<BloomFilter<T>> {
    let mut result: Owned<BloomFilter<T>>;
    let mut i: u32;
    let mut array_size: u32;
    if filter1.table_size != filter2.table_size
        || filter1.num_functions != filter2.num_functions
        || filter1.hash_func != filter2.hash_func {
        return null!();
    }
    result = bloom_filter_new(filter1.table_size, filter1.hash_func, filter1.num_functions);
    if result == null!() {
        return null!();
    }
    array_size = (filter1.table_size + 7) / 8;
    c_for!(i = 0; i < array_size; i += 1; {
        result.table[i] = filter1.table[i] & filter2.table[i];
    });
    return result;
}
