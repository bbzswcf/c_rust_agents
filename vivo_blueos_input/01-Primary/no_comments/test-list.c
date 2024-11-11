#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "list.h"
#include "compare-int.h"
int variable1 = 50, variable2, variable3, variable4;
ListEntry *generate_list(void)
{
	ListEntry *list = NULL;
	assert(list_append(&list, &variable1) != NULL);
	assert(list_append(&list, &variable2) != NULL);
	assert(list_append(&list, &variable3) != NULL);
	assert(list_append(&list, &variable4) != NULL);
	return list;
}
void check_list_integrity(ListEntry *list)
{
	ListEntry *prev;
	ListEntry *rover;
	prev = NULL;
	rover = list;
	while (rover != NULL) {
		assert(list_prev(rover) == prev);
		prev = rover;
		rover = list_next(rover);
	}
}
void test_list_append(void)
{
	ListEntry *list = NULL;
	assert(list_append(&list, &variable1) != NULL);
	check_list_integrity(list);
	assert(list_append(&list, &variable2) != NULL);
	check_list_integrity(list);
	assert(list_append(&list, &variable3) != NULL);
	check_list_integrity(list);
	assert(list_append(&list, &variable4) != NULL);
	check_list_integrity(list);
	assert(list_length(list) == 4);
	assert(list_nth_data(list, 0) == &variable1);
	assert(list_nth_data(list, 1) == &variable2);
	assert(list_nth_data(list, 2) == &variable3);
	assert(list_nth_data(list, 3) == &variable4);
	alloc_test_set_limit(0);
	assert(list_length(list) == 4);
	assert(list_append(&list, &variable1) == NULL);
	assert(list_length(list) == 4);
	check_list_integrity(list);
	list_free(list);
}
void test_list_prepend(void)
{
	ListEntry *list = NULL;
	assert(list_prepend(&list, &variable1) != NULL);
	check_list_integrity(list);
	assert(list_prepend(&list, &variable2) != NULL);
	check_list_integrity(list);
	assert(list_prepend(&list, &variable3) != NULL);
	check_list_integrity(list);
	assert(list_prepend(&list, &variable4) != NULL);
	check_list_integrity(list);
	assert(list_nth_data(list, 0) == &variable4);
	assert(list_nth_data(list, 1) == &variable3);
	assert(list_nth_data(list, 2) == &variable2);
	assert(list_nth_data(list, 3) == &variable1);
	alloc_test_set_limit(0);
	assert(list_length(list) == 4);
	assert(list_prepend(&list, &variable1) == NULL);
	assert(list_length(list) == 4);
	check_list_integrity(list);
	list_free(list);
}
void test_list_free(void)
{
	ListEntry *list;
	list = generate_list();
	list_free(list);
	list_free(NULL);
}
void test_list_next(void)
{
	ListEntry *list;
	ListEntry *rover;
	list = generate_list();
	rover = list;
	assert(list_data(rover) == &variable1);
	rover = list_next(rover);
	assert(list_data(rover) == &variable2);
	rover = list_next(rover);
	assert(list_data(rover) == &variable3);
	rover = list_next(rover);
	assert(list_data(rover) == &variable4);
	rover = list_next(rover);
	assert(rover == NULL);
	list_free(list);
}
void test_list_nth_entry(void)
{
	ListEntry *list;
	ListEntry *entry;
	list = generate_list();
	entry = list_nth_entry(list, 0);
	assert(list_data(entry) == &variable1);
	entry = list_nth_entry(list, 1);
	assert(list_data(entry) == &variable2);
	entry = list_nth_entry(list, 2);
	assert(list_data(entry) == &variable3);
	entry = list_nth_entry(list, 3);
	assert(list_data(entry) == &variable4);
	entry = list_nth_entry(list, 4);
	assert(entry == NULL);
	entry = list_nth_entry(list, 400);
	assert(entry == NULL);
	list_free(list);
}
void test_list_nth_data(void)
{
	ListEntry *list;
	list = generate_list();
	assert(list_nth_data(list, 0) == &variable1);
	assert(list_nth_data(list, 1) == &variable2);
	assert(list_nth_data(list, 2) == &variable3);
	assert(list_nth_data(list, 3) == &variable4);
	assert(list_nth_data(list, 4) == NULL);
	assert(list_nth_data(list, 400) == NULL);
	list_free(list);
}
void test_list_length(void)
{
	ListEntry *list;
	list = generate_list();
	assert(list_length(list) == 4);
	assert(list_prepend(&list, &variable1) != NULL);
	assert(list_length(list) == 5);
	list_free(list);
	assert(list_length(NULL) == 0);
}
void test_list_remove_entry(void)
{
	ListEntry *empty_list = NULL;
	ListEntry *list;
	ListEntry *entry;
	list = generate_list();
	entry = list_nth_entry(list, 2);
	assert(list_remove_entry(&list, entry) != 0);
	assert(list_length(list) == 3);
	check_list_integrity(list);
	entry = list_nth_entry(list, 0);
	assert(list_remove_entry(&list, entry) != 0);
	assert(list_length(list) == 2);
	check_list_integrity(list);
	assert(list_remove_entry(&list, NULL) == 0);
	assert(list_remove_entry(&empty_list, NULL) == 0);
	list_free(list);
	list = NULL;
	assert(list_append(&list, &variable1) != NULL);
	assert(list != NULL);
	assert(list_remove_entry(&list, list) != 0);
	assert(list == NULL);
	list = generate_list();
	entry = list_nth_entry(list, 3);
	assert(list_remove_entry(&list, entry) != 0);
	check_list_integrity(list);
	list_free(list);
}
void test_list_remove_data(void)
{
	int entries[] = { 89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4 };
	unsigned int num_entries = sizeof(entries) / sizeof(int);
	int val;
	ListEntry *list;
	unsigned int i;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		assert(list_prepend(&list, &entries[i]) != NULL);
	}
	val = 0;
	assert(list_remove_data(&list, int_equal, &val) == 0);
	val = 56;
	assert(list_remove_data(&list, int_equal, &val) == 0);
	check_list_integrity(list);
	val = 8;
	assert(list_remove_data(&list, int_equal, &val) == 1);
	assert(list_length(list) == num_entries - 1);
	check_list_integrity(list);
	val = 4;
	assert(list_remove_data(&list, int_equal, &val) == 4);
	assert(list_length(list) == num_entries - 5);
	check_list_integrity(list);
	val = 89;
	assert(list_remove_data(&list, int_equal, &val) == 1);
	assert(list_length(list) == num_entries - 6);
	check_list_integrity(list);
	list_free(list);
}
void test_list_sort(void)
{
	ListEntry *list;
	int entries[] = { 89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4 };
	int sorted[]  = { 4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99 };
	unsigned int num_entries = sizeof(entries) / sizeof(int);
	unsigned int i;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		assert(list_prepend(&list, &entries[i]) != NULL);
	}
	list_sort(&list, int_compare);
	assert(list_length(list) == num_entries);
	for (i=0; i<num_entries; ++i) {
		int *value;
		value = (int *) list_nth_data(list, i);
		assert(*value == sorted[i]);
	}
	list_free(list);
	list = NULL;
	list_sort(&list, int_compare);
	assert(list == NULL);
}
void test_list_find_data(void)
{
	int entries[] = { 89, 23, 42, 16, 15, 4, 8, 99, 50, 30 };
	int num_entries = sizeof(entries) / sizeof(int);
	ListEntry *list;
	ListEntry *result;
	int i;
	int val;
	int *data;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		assert(list_append(&list, &entries[i]) != NULL);
	}
	for (i=0; i<num_entries; ++i) {
		val = entries[i];
		result = list_find_data(list, int_equal, &val);
		assert(result != NULL);
		data = (int *) list_data(result);
		assert(*data == val);
	}
	val = 0;
	assert(list_find_data(list, int_equal, &val) == NULL);
	val = 56;
	assert(list_find_data(list, int_equal, &val) == NULL);
	list_free(list);
}
void test_list_to_array(void)
{
	ListEntry *list;
	void **array;
	list = generate_list();
	array = list_to_array(list);
	assert(array[0] == &variable1);
	assert(array[1] == &variable2);
	assert(array[2] == &variable3);
	assert(array[3] == &variable4);
	free(array);
	alloc_test_set_limit(0);
	array = list_to_array(list);
	assert(array == NULL);
	list_free(list);
}
void test_list_iterate(void)
{
	ListEntry *list;
	ListIterator iter;
	int i;
	int a;
	int counter;
	int *data;
	list = NULL;
	for (i=0; i<50; ++i) {
		assert(list_prepend(&list, &a) != NULL);
	}
	counter = 0;
	list_iterate(&list, &iter);
	list_iter_remove(&iter);
	while (list_iter_has_more(&iter)) {
		data = (int *) list_iter_next(&iter);
		++counter;
		if ((counter % 2) == 0) {
			list_iter_remove(&iter);
			list_iter_remove(&iter);
		}
	}
	assert(list_iter_next(&iter) == NULL);
	list_iter_remove(&iter);
	assert(counter == 50);
	assert(list_length(list) == 25);
	list_free(list);
	list = NULL;
	counter = 0;
	list_iterate(&list, &iter);
	while (list_iter_has_more(&iter)) {
		data = (int *) list_iter_next(&iter);
		++counter;
	}
	assert(counter == 0);
}
void test_list_iterate_bad_remove(void)
{
	ListEntry *list;
	ListIterator iter;
	int values[49];
	int i;
	int *val;
	list = NULL;
	for (i=0; i<49; ++i) {
		values[i] = i;
		assert(list_prepend(&list, &values[i]) != NULL);
	}
	list_iterate(&list, &iter);
	while (list_iter_has_more(&iter)) {
		val = list_iter_next(&iter);
		if ((*val % 2) == 0) {
			assert(list_remove_data(&list, int_equal, val) != 0);
			list_iter_remove(&iter);
		}
	}
	list_free(list);
}
static UnitTestFunction tests[] = {
	test_list_append,
	test_list_prepend,
	test_list_free,
	test_list_next,
	test_list_nth_entry,
	test_list_nth_data,
	test_list_length,
	test_list_remove_entry,
	test_list_remove_data,
	test_list_sort,
	test_list_find_data,
	test_list_to_array,
	test_list_iterate,
	test_list_iterate_bad_remove,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
