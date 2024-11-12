#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "slist.h"
#include "compare-int.h"
int variable1 = 50, variable2, variable3, variable4;
SListEntry *generate_list(void)
{
	SListEntry *list = NULL;
	assert(slist_append(&list, &variable1) != NULL);
	assert(slist_append(&list, &variable2) != NULL);
	assert(slist_append(&list, &variable3) != NULL);
	assert(slist_append(&list, &variable4) != NULL);
	return list;
}
void test_slist_append(void)
{
	SListEntry *list = NULL;
	assert(slist_append(&list, &variable1) != NULL);
	assert(slist_append(&list, &variable2) != NULL);
	assert(slist_append(&list, &variable3) != NULL);
	assert(slist_append(&list, &variable4) != NULL);
	assert(slist_length(list) == 4);
	assert(slist_nth_data(list, 0) == &variable1);
	assert(slist_nth_data(list, 1) == &variable2);
	assert(slist_nth_data(list, 2) == &variable3);
	assert(slist_nth_data(list, 3) == &variable4);
	alloc_test_set_limit(0);
	assert(slist_length(list) == 4);
	assert(slist_append(&list, &variable1) == NULL);
	assert(slist_length(list) == 4);
	slist_free(list);
}
void test_slist_prepend(void)
{
	SListEntry *list = NULL;
	assert(slist_prepend(&list, &variable1) != NULL);
	assert(slist_prepend(&list, &variable2) != NULL);
	assert(slist_prepend(&list, &variable3) != NULL);
	assert(slist_prepend(&list, &variable4) != NULL);
	assert(slist_nth_data(list, 0) == &variable4);
	assert(slist_nth_data(list, 1) == &variable3);
	assert(slist_nth_data(list, 2) == &variable2);
	assert(slist_nth_data(list, 3) == &variable1);
	alloc_test_set_limit(0);
	assert(slist_length(list) == 4);
	assert(slist_prepend(&list, &variable1) == NULL);
	assert(slist_length(list) == 4);
	slist_free(list);
}
void test_slist_free(void)
{
	SListEntry *list;
	list = generate_list();
	slist_free(list);
	slist_free(NULL);
}
void test_slist_next(void)
{
	SListEntry *list;
	SListEntry *rover;
	list = generate_list();
	rover = list;
	assert(slist_data(rover) == &variable1);
	rover = slist_next(rover);
	assert(slist_data(rover) == &variable2);
	rover = slist_next(rover);
	assert(slist_data(rover) == &variable3);
	rover = slist_next(rover);
	assert(slist_data(rover) == &variable4);
	rover = slist_next(rover);
	assert(rover == NULL);
	slist_free(list);
}
void test_slist_nth_entry(void)
{
	SListEntry *list;
	SListEntry *entry;
	list = generate_list();
	entry = slist_nth_entry(list, 0);
	assert(slist_data(entry) == &variable1);
	entry = slist_nth_entry(list, 1);
	assert(slist_data(entry) == &variable2);
	entry = slist_nth_entry(list, 2);
	assert(slist_data(entry) == &variable3);
	entry = slist_nth_entry(list, 3);
	assert(slist_data(entry) == &variable4);
	entry = slist_nth_entry(list, 4);
	assert(entry == NULL);
	entry = slist_nth_entry(list, 400);
	assert(entry == NULL);
	slist_free(list);
}
void test_slist_nth_data(void)
{
	SListEntry *list;
	list = generate_list();
	assert(slist_nth_data(list, 0) == &variable1);
	assert(slist_nth_data(list, 1) == &variable2);
	assert(slist_nth_data(list, 2) == &variable3);
	assert(slist_nth_data(list, 3) == &variable4);
	assert(slist_nth_data(list, 4) == NULL);
	assert(slist_nth_data(list, 400) == NULL);
	slist_free(list);
}
void test_slist_length(void)
{
	SListEntry *list;
	list = generate_list();
	assert(slist_length(list) == 4);
	slist_prepend(&list, &variable1);
	assert(slist_length(list) == 5);
	assert(slist_length(NULL) == 0);
	slist_free(list);
}
void test_slist_remove_entry(void)
{
	SListEntry *empty_list = NULL;
	SListEntry *list;
	SListEntry *entry;
	list = generate_list();
	entry = slist_nth_entry(list, 2);
	assert(slist_remove_entry(&list, entry) != 0);
	assert(slist_length(list) == 3);
	entry = slist_nth_entry(list, 0);
	assert(slist_remove_entry(&list, entry) != 0);
	assert(slist_length(list) == 2);
	assert(slist_remove_entry(&list, entry) == 0);
	assert(slist_remove_entry(&list, NULL) == 0);
	assert(slist_remove_entry(&empty_list, NULL) == 0);
	slist_free(list);
}
void test_slist_remove_data(void)
{
	int entries[] = { 89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4 };
	unsigned int num_entries = sizeof(entries) / sizeof(int);
	int val;
	SListEntry *list;
	unsigned int i;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		slist_prepend(&list, &entries[i]);
	}
	val = 0;
	assert(slist_remove_data(&list, int_equal, &val) == 0);
	val = 56;
	assert(slist_remove_data(&list, int_equal, &val) == 0);
	val = 8;
	assert(slist_remove_data(&list, int_equal, &val) == 1);
	assert(slist_length(list) == num_entries - 1);
	val = 4;
	assert(slist_remove_data(&list, int_equal, &val) == 4);
	assert(slist_length(list) == num_entries - 5);
	val = 89;
	assert(slist_remove_data(&list, int_equal, &val) == 1);
	assert(slist_length(list) == num_entries - 6);
	slist_free(list);
}
void test_slist_sort(void)
{
	SListEntry *list;
	int entries[] = { 89, 4, 23, 42, 4, 16, 15, 4, 8, 99, 50, 30, 4 };
	int sorted[]  = { 4, 4, 4, 4, 8, 15, 16, 23, 30, 42, 50, 89, 99 };
	unsigned int num_entries = sizeof(entries) / sizeof(int);
	unsigned int i;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		slist_prepend(&list, &entries[i]);
	}
	slist_sort(&list, int_compare);
	assert(slist_length(list) == num_entries);
	for (i=0; i<num_entries; ++i) {
		int *value;
		value = (int *) slist_nth_data(list, i);
		assert(*value == sorted[i]);
	}
	slist_free(list);
	list = NULL;
	slist_sort(&list, int_compare);
	assert(list == NULL);
}
void test_slist_find_data(void)
{
	int entries[] = { 89, 23, 42, 16, 15, 4, 8, 99, 50, 30 };
	int num_entries = sizeof(entries) / sizeof(int);
	SListEntry *list;
	SListEntry *result;
	int i;
	int val;
	int *data;
	list = NULL;
	for (i=0; i<num_entries; ++i) {
		slist_append(&list, &entries[i]);
	}
	for (i=0; i<num_entries; ++i) {
		val = entries[i];
		result = slist_find_data(list, int_equal, &val);
		assert(result != NULL);
		data = (int *) slist_data(result);
		assert(*data == val);
	}
	val = 0;
	assert(slist_find_data(list, int_equal, &val) == NULL);
	val = 56;
	assert(slist_find_data(list, int_equal, &val) == NULL);
	slist_free(list);
}
void test_slist_to_array(void)
{
	SListEntry *list;
	void **array;
	list = generate_list();
	array = slist_to_array(list);
	assert(array[0] == &variable1);
	assert(array[1] == &variable2);
	assert(array[2] == &variable3);
	assert(array[3] == &variable4);
	free(array);
	alloc_test_set_limit(0);
	array = slist_to_array(list);
	assert(array == NULL);
	slist_free(list);
}
void test_slist_iterate(void)
{
	SListEntry *list;
	SListIterator iter;
	int *data;
	int a;
	int i;
	int counter;
	list = NULL;
	for (i=0; i<50; ++i) {
		slist_prepend(&list, &a);
	}
	counter = 0;
	slist_iterate(&list, &iter);
	slist_iter_remove(&iter);
	while (slist_iter_has_more(&iter)) {
		data = (int *) slist_iter_next(&iter);
		++counter;
		if ((counter % 2) == 0) {
			slist_iter_remove(&iter);
			slist_iter_remove(&iter);
		}
	}
	assert(slist_iter_next(&iter) == SLIST_NULL);
	slist_iter_remove(&iter);
	assert(counter == 50);
	assert(slist_length(list) == 25);
	slist_free(list);
	list = NULL;
	counter = 0;
	slist_iterate(&list, &iter);
	while (slist_iter_has_more(&iter)) {
		data = (int *) slist_iter_next(&iter);
		++counter;
		if ((counter % 2) == 0) {
			slist_iter_remove(&iter);
		}
	}
	assert(counter == 0);
}
void test_slist_iterate_bad_remove(void)
{
	SListEntry *list;
	SListIterator iter;
	int values[49];
	int i;
	int *val;
	list = NULL;
	for (i=0; i<49; ++i) {
		values[i] = i;
		slist_prepend(&list, &values[i]);
	}
	slist_iterate(&list, &iter);
	while (slist_iter_has_more(&iter)) {
		val = slist_iter_next(&iter);
		if ((*val % 2) == 0) {
			assert(slist_remove_data(&list, int_equal, val) != 0);
			slist_iter_remove(&iter);
		}
	}
	slist_free(list);
}
static UnitTestFunction tests[] = {
	test_slist_append,
	test_slist_prepend,
	test_slist_free,
	test_slist_next,
	test_slist_nth_entry,
	test_slist_nth_data,
	test_slist_length,
	test_slist_remove_entry,
	test_slist_remove_data,
	test_slist_sort,
	test_slist_find_data,
	test_slist_to_array,
	test_slist_iterate,
	test_slist_iterate_bad_remove,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
