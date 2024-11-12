#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "hash-table.h"
#include "hash-int.h"
#include "compare-int.h"
#include "hash-string.h"
#include "compare-string.h"
#define NUM_TEST_VALUES 10000
int value1 = 1, value2 = 2, value3 = 3, value4 = 4;
int allocated_keys = 0;
int allocated_values = 0;
HashTable *generate_hash_table(void)
{
	HashTable *hash_table;
	char buf[10];
	char *value;
	int i;
	hash_table = hash_table_new(string_hash, string_equal);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		sprintf(buf, "%i", i);
		value = strdup(buf);
		hash_table_insert(hash_table, value, value);
	}
	hash_table_register_free_functions(hash_table, NULL, free);
	return hash_table;
}
void test_hash_table_new_free(void)
{
	HashTable *hash_table;
	hash_table = hash_table_new(int_hash, int_equal);
	assert(hash_table != NULL);
	hash_table_insert(hash_table, &value1, &value1);
	hash_table_insert(hash_table, &value2, &value2);
	hash_table_insert(hash_table, &value3, &value3);
	hash_table_insert(hash_table, &value4, &value4);
	hash_table_free(hash_table);
	alloc_test_set_limit(0);
	hash_table = hash_table_new(int_hash, int_equal);
	assert(hash_table == NULL);
	assert(alloc_test_get_allocated() == 0);
	alloc_test_set_limit(1);
	hash_table = hash_table_new(int_hash, int_equal);
	assert(hash_table == NULL);
	assert(alloc_test_get_allocated() == 0);
}
void test_hash_table_insert_lookup(void)
{
	HashTable *hash_table;
	char buf[10];
	char *value;
	int i;
	hash_table = generate_hash_table();
	assert(hash_table_num_entries(hash_table) == NUM_TEST_VALUES);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		sprintf(buf, "%i", i);
		value = hash_table_lookup(hash_table, buf);
		assert(strcmp(value, buf) == 0);
	}
	sprintf(buf, "%i", -1);
	assert(hash_table_lookup(hash_table, buf) == NULL);
	sprintf(buf, "%i", NUM_TEST_VALUES);
	assert(hash_table_lookup(hash_table, buf) == NULL);
	sprintf(buf, "%i", 12345);
	hash_table_insert(hash_table, buf, strdup("hello world"));
	value = hash_table_lookup(hash_table, buf);
	assert(strcmp(value, "hello world") == 0);
	hash_table_free(hash_table);
}
void test_hash_table_remove(void)
{
	HashTable *hash_table;
	char buf[10];
	hash_table = generate_hash_table();
	assert(hash_table_num_entries(hash_table) == NUM_TEST_VALUES);
	sprintf(buf, "%i", 5000);
	assert(hash_table_lookup(hash_table, buf) != NULL);
	hash_table_remove(hash_table, buf);
	assert(hash_table_num_entries(hash_table) == 9999);
	assert(hash_table_lookup(hash_table, buf) == NULL);
	sprintf(buf, "%i", -1);
	hash_table_remove(hash_table, buf);
	assert(hash_table_num_entries(hash_table) == 9999);
	hash_table_free(hash_table);
}
void test_hash_table_iterating(void)
{
	HashTable *hash_table;
	HashTableIterator iterator;
	int count;
	hash_table = generate_hash_table();
	count = 0;
	hash_table_iterate(hash_table, &iterator);
	while (hash_table_iter_has_more(&iterator)) {
		hash_table_iter_next(&iterator);
		++count;
	}
	assert(count == NUM_TEST_VALUES);
	HashTablePair pair = hash_table_iter_next(&iterator);
	assert(pair.value == HASH_TABLE_NULL);
	hash_table_free(hash_table);
	hash_table = hash_table_new(int_hash, int_equal);
	hash_table_iterate(hash_table, &iterator);
	assert(hash_table_iter_has_more(&iterator) == 0);
	hash_table_free(hash_table);
}
void test_hash_table_iterating_remove(void)
{
	HashTable *hash_table;
	HashTableIterator iterator;
	char buf[10];
	char *val;
	HashTablePair pair;
	int count;
	unsigned int removed;
	int i;
	hash_table = generate_hash_table();
	count = 0;
	removed = 0;
	hash_table_iterate(hash_table, &iterator);
	while (hash_table_iter_has_more(&iterator)) {
		pair = hash_table_iter_next(&iterator);
		val = pair.value;
		if ((atoi(val) % 100) == 0) {
			hash_table_remove(hash_table, val);
			++removed;
		}
		++count;
	}
	assert(removed == 100);
	assert(count == NUM_TEST_VALUES);
	assert(hash_table_num_entries(hash_table)
	       == NUM_TEST_VALUES - removed);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		sprintf(buf, "%i", i);
		if (i % 100 == 0) {
			assert(hash_table_lookup(hash_table, buf) == NULL);
		} else {
			assert(hash_table_lookup(hash_table, buf) != NULL);
		}
	}
	hash_table_free(hash_table);
}
int *new_key(int value)
{
	int *result;
	result = malloc(sizeof(int));
	*result = value;
	++allocated_keys;
	return result;
}
void free_key(void *key)
{
	free(key);
	--allocated_keys;
}
int *new_value(int value)
{
	int *result;
	result = malloc(sizeof(int));
	*result = value;
	++allocated_values;
	return result;
}
void free_value(void *value)
{
	free(value);
	--allocated_values;
}
void test_hash_table_free_functions(void)
{
	HashTable *hash_table;
	int *key;
	int *value;
	int i;
	hash_table = hash_table_new(int_hash, int_equal);
	hash_table_register_free_functions(hash_table, free_key, free_value);
	allocated_values = 0;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		key = new_key(i);
		value = new_value(99);
		hash_table_insert(hash_table, key, value);
	}
	assert(allocated_keys == NUM_TEST_VALUES);
	assert(allocated_values == NUM_TEST_VALUES);
	i = NUM_TEST_VALUES / 2;
	hash_table_remove(hash_table, &i);
	assert(allocated_keys == NUM_TEST_VALUES - 1);
	assert(allocated_values == NUM_TEST_VALUES - 1);
	key = new_key(NUM_TEST_VALUES / 3);
	value = new_value(999);
	assert(allocated_keys == NUM_TEST_VALUES);
	assert(allocated_values == NUM_TEST_VALUES);
	hash_table_insert(hash_table, key, value);
	assert(allocated_keys == NUM_TEST_VALUES - 1);
	assert(allocated_values == NUM_TEST_VALUES - 1);
	hash_table_free(hash_table);
	assert(allocated_keys == 0);
	assert(allocated_values == 0);
}
void test_hash_table_out_of_memory(void)
{
	HashTable *hash_table;
	int values[66];
	unsigned int i;
	hash_table = hash_table_new(int_hash, int_equal);
	alloc_test_set_limit(0);
	values[0] = 0;
	assert(hash_table_insert(hash_table, &values[0], &values[0]) == 0);
	assert(hash_table_num_entries(hash_table) == 0);
	alloc_test_set_limit(-1);
	for (i=0; i<65; ++i) {
		values[i] = (int) i;
		assert(hash_table_insert(hash_table,
		                         &values[i], &values[i]) != 0);
		assert(hash_table_num_entries(hash_table) == i + 1);
	}
	assert(hash_table_num_entries(hash_table) == 65);
	alloc_test_set_limit(0);
	values[65] = 65;
	assert(hash_table_insert(hash_table, &values[65], &values[65]) == 0);
	assert(hash_table_num_entries(hash_table) == 65);
	hash_table_free(hash_table);
}
void test_hash_iterator_key_pair()
{
	HashTable *hash_table;
	HashTableIterator iterator;
	HashTablePair pair;
	hash_table = hash_table_new(int_hash, int_equal);
	hash_table_insert(hash_table, &value1, &value1);
	hash_table_insert(hash_table, &value2, &value2);
	hash_table_iterate(hash_table, &iterator);
	while (hash_table_iter_has_more(&iterator)) {
		pair = hash_table_iter_next(&iterator);
		int *key = (int*) pair.key;
		int *val = (int*) pair.value;
		assert(*key == *val);
	}
	hash_table_free(hash_table);
}
static UnitTestFunction tests[] = {
	test_hash_table_new_free,
	test_hash_table_insert_lookup,
	test_hash_table_remove,
	test_hash_table_iterating,
	test_hash_table_iterating_remove,
	test_hash_table_free_functions,
	test_hash_table_out_of_memory,
	test_hash_iterator_key_pair,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
