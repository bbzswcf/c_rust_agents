#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "set.h"
#include "compare-int.h"
#include "hash-int.h"
#include "compare-pointer.h"
#include "hash-pointer.h"
#include "compare-string.h"
#include "hash-string.h"
int allocated_values;
Set *generate_set(void)
{
	Set *set;
	char buf[10];
	unsigned int i;
	char *value;
	set = set_new(string_hash, string_equal);
	for (i=0; i<10000; ++i) {
		sprintf(buf, "%i", i);
		value = strdup(buf);
		set_insert(set, value);
		assert(set_num_entries(set) == i + 1);
	}
	set_register_free_function(set, free);
	return set;
}
void test_set_new_free(void)
{
	Set *set;
	int i;
	int *value;
	set = set_new(int_hash, int_equal);
	set_register_free_function(set, free);
	assert(set != NULL);
	for (i=0; i<10000; ++i) {
		value = (int *) malloc(sizeof(int));
		*value = i;
		set_insert(set, value);
	}
	set_free(set);
	alloc_test_set_limit(0);
	set = set_new(int_hash, int_equal);
	assert(set == NULL);
	alloc_test_set_limit(1);
	set = set_new(int_hash, int_equal);
	assert(set == NULL);
	assert(alloc_test_get_allocated() == 0);
}
void test_set_insert(void)
{
	Set *set;
	int numbers1[] = { 1, 2, 3, 4, 5, 6 };
	int numbers2[] = { 5, 6, 7, 8, 9, 10 };
	int i;
	set = set_new(int_hash, int_equal);
	for (i=0; i<6; ++i) {
		set_insert(set, &numbers1[i]);
	}
	for (i=0; i<6; ++i) {
		set_insert(set, &numbers2[i]);
	}
	assert(set_num_entries(set) == 10);
	set_free(set);
}
void test_set_query(void)
{
	Set *set;
	char buf[10];
	int i;
	set = generate_set();
	for (i=0; i<10000; ++i) {
		sprintf(buf, "%i", i);
		assert(set_query(set, buf) != 0);
	}
	assert(set_query(set, "-1") == 0);
	assert(set_query(set, "100001") == 0);
	set_free(set);
}
void test_set_remove(void)
{
	Set *set;
	char buf[10];
	int i;
	unsigned int num_entries;
	set = generate_set();
	num_entries = set_num_entries(set);
	assert(num_entries == 10000);
	for (i=4000; i<6000; ++i) {
		sprintf(buf, "%i", i);
		assert(set_query(set, buf) != 0);
		assert(set_remove(set, buf) != 0);
		assert(set_num_entries(set) == num_entries - 1);
		assert(set_query(set, buf) == 0);
		--num_entries;
	}
	for (i=-1000; i<-500; ++i) {
		sprintf(buf, "%i", i);
		assert(set_remove(set, buf) == 0);
		assert(set_num_entries(set) == num_entries);
	}
	for (i=50000; i<51000; ++i) {
		sprintf(buf, "%i", i);
		assert(set_remove(set, buf) == 0);
		assert(set_num_entries(set) == num_entries);
	}
	set_free(set);
}
void test_set_union(void)
{
	int numbers1[] = {1, 2, 3, 4, 5, 6, 7};
	int numbers2[] = {5, 6, 7, 8, 9, 10, 11};
	int result[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};
	int i;
	Set *set1;
	Set *set2;
	Set *result_set;
	size_t allocated;
	set1 = set_new(int_hash, int_equal);
	for (i=0; i<7; ++i) {
		set_insert(set1, &numbers1[i]);
	}
	set2 = set_new(int_hash, int_equal);
	for (i=0; i<7; ++i) {
		set_insert(set2, &numbers2[i]);
	}
	result_set = set_union(set1, set2);
	assert(set_num_entries(result_set) == 11);
	for (i=0; i<11; ++i) {
		assert(set_query(result_set, &result[i]) != 0);
	}
	set_free(result_set);
	alloc_test_set_limit(0);
	assert(set_union(set1, set2) == NULL);
	alloc_test_set_limit(2 + 2);
	allocated = alloc_test_get_allocated();
	assert(set_union(set1, set2) == NULL);
	assert(alloc_test_get_allocated() == allocated);
	alloc_test_set_limit(2 + 7 + 2);
	allocated = alloc_test_get_allocated();
	assert(set_union(set1, set2) == NULL);
	assert(alloc_test_get_allocated() == allocated);
	set_free(set1);
	set_free(set2);
}
void test_set_intersection(void)
{
	int numbers1[] = {1, 2, 3, 4, 5, 6, 7};
	int numbers2[] = {5, 6, 7, 8, 9, 10, 11};
	int result[] = {5, 6, 7};
	int i;
	Set *set1;
	Set *set2;
	Set *result_set;
	size_t allocated;
	set1 = set_new(int_hash, int_equal);
	for (i=0; i<7; ++i) {
		set_insert(set1, &numbers1[i]);
	}
	set2 = set_new(int_hash, int_equal);
	for (i=0; i<7; ++i) {
		set_insert(set2, &numbers2[i]);
	}
	result_set = set_intersection(set1, set2);
	assert(set_num_entries(result_set) == 3);
	for (i=0; i<3; ++i) {
		assert(set_query(result_set, &result[i]) != 0);
	}
	alloc_test_set_limit(0);
	assert(set_intersection(set1, set2) == NULL);
	alloc_test_set_limit(2 + 2);
	allocated = alloc_test_get_allocated();
	assert(set_intersection(set1, set2) == NULL);
	assert(alloc_test_get_allocated() == allocated);
	set_free(set1);
	set_free(set2);
	set_free(result_set);
}
void test_set_to_array(void)
{
	Set *set;
	int values[100];
	int **array;
	int i;
	set = set_new(pointer_hash, pointer_equal);
	for (i=0; i<100; ++i) {
		values[i] = 1;
		set_insert(set, &values[i]);
	}
	array = (int **) set_to_array(set);
	for (i=0; i<100; ++i) {
		assert(*array[i] == 1);
		*array[i] = 0;
	}
	alloc_test_set_limit(0);
	assert(set_to_array(set) == NULL);
	free(array);
	set_free(set);
}
void test_set_iterating(void)
{
	Set *set;
	SetIterator iterator;
	int count;
	set = generate_set();
	count = 0;
	set_iterate(set, &iterator);
	while (set_iter_has_more(&iterator)) {
		set_iter_next(&iterator);
		++count;
	}
	assert(set_iter_next(&iterator) == NULL);
	assert(count == 10000);
	set_free(set);
	set = set_new(int_hash, int_equal);
	set_iterate(set, &iterator);
	assert(set_iter_has_more(&iterator) == 0);
	set_free(set);
}
void test_set_iterating_remove(void)
{
	Set *set;
	SetIterator iterator;
	int count;
	unsigned int removed;
	char *value;
	set = generate_set();
	count = 0;
	removed = 0;
	set_iterate(set, &iterator);
	while (set_iter_has_more(&iterator)) {
		value = set_iter_next(&iterator);
		if ((atoi(value) % 100) == 0) {
			set_remove(set, value);
			++removed;
		}
		++count;
	}
	assert(count == 10000);
	assert(removed == 100);
	assert(set_num_entries(set) == 10000 - removed);
	set_free(set);
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
void test_set_free_function(void)
{
	Set *set;
	int i;
	int *value;
	set = set_new(int_hash, int_equal);
	set_register_free_function(set, free_value);
	allocated_values = 0;
	for (i=0; i<1000; ++i) {
		value = new_value(i);
		set_insert(set, value);
	}
	assert(allocated_values == 1000);
	i = 500;
	set_remove(set, &i);
	assert(allocated_values == 999);
	set_free(set);
	assert(allocated_values == 0);
}
void test_set_out_of_memory(void)
{
	Set *set;
	int values[66];
	unsigned int i;
	set = set_new(int_hash, int_equal);
	alloc_test_set_limit(0);
	values[0] = 0;
	assert(set_insert(set, &values[0]) == 0);
	assert(set_num_entries(set) == 0);
	alloc_test_set_limit(-1);
	for (i=0; i<65; ++i) {
		values[i] = (int) i;
		assert(set_insert(set, &values[i]) != 0);
		assert(set_num_entries(set) == i + 1);
	}
	assert(set_num_entries(set) == 65);
	alloc_test_set_limit(0);
	values[65] = 65;
	assert(set_insert(set, &values[65]) == 0);
	assert(set_num_entries(set) == 65);
	set_free(set);
}
static UnitTestFunction tests[] = {
	test_set_new_free,
	test_set_insert,
	test_set_query,
	test_set_remove,
	test_set_intersection,
	test_set_union,
	test_set_iterating,
	test_set_iterating_remove,
	test_set_to_array,
	test_set_free_function,
	test_set_out_of_memory,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
