#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "trie.h"
#define NUM_TEST_VALUES 10000
int test_array[NUM_TEST_VALUES];
char test_strings[NUM_TEST_VALUES][10];
unsigned char bin_key[] = { 'a', 'b', 'c', 0, 1, 2, 0xff };
unsigned char bin_key2[] = { 'a', 'b', 'c', 0, 1, 2, 0xff, 0 };
unsigned char bin_key3[] = { 'a', 'b', 'c' };
unsigned char bin_key4[] = { 'z', 0, 'z', 'z' };
Trie *generate_trie(void)
{
	Trie *trie;
	int i;
	unsigned int entries;
	trie = trie_new();
	entries = 0;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		test_array[i] = i;
		sprintf(test_strings[i], "%i", i);
		assert(trie_insert(trie, test_strings[i],
		                   &test_array[i]) != 0);
		++entries;
		assert(trie_num_entries(trie) == entries);
	}
	return trie;
}
void test_trie_new_free(void)
{
	Trie *trie;
	trie = trie_new();
	assert(trie != NULL);
	trie_free(trie);
	trie = trie_new();
	assert(trie_insert(trie, "hello", "there") != 0);
	assert(trie_insert(trie, "hell", "testing") != 0);
	assert(trie_insert(trie, "testing", "testing") != 0);
	assert(trie_insert(trie, "", "asfasf") != 0);
	trie_free(trie);
	trie = trie_new();
	assert(trie_insert(trie, "hello", "there") != 0);
	assert(trie_remove(trie, "hello") != 0);
	trie_free(trie);
	alloc_test_set_limit(0);
	trie = trie_new();
	assert(trie == NULL);
}
void test_trie_insert(void)
{
	Trie *trie;
	unsigned int entries;
	size_t allocated;
	trie = generate_trie();
	entries = trie_num_entries(trie);
	assert(trie_insert(trie, "hello world", NULL) == 0);
	assert(trie_num_entries(trie) == entries);
	allocated = alloc_test_get_allocated();
	alloc_test_set_limit(0);
	assert(trie_insert(trie, "a", "test value") == 0);
	assert(trie_num_entries(trie) == entries);
	alloc_test_set_limit(5);
	assert(trie_insert(trie, "hello world", "test value") == 0);
	assert(alloc_test_get_allocated() == allocated);
	assert(trie_num_entries(trie) == entries);
	trie_free(trie);
}
void test_trie_lookup(void)
{
	Trie *trie;
	char buf[10];
	int *val;
	int i;
	trie = generate_trie();
	assert(trie_lookup(trie, "000000000000000") == TRIE_NULL);
	assert(trie_lookup(trie, "") == TRIE_NULL);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		sprintf(buf, "%i", i);
		val = (int *) trie_lookup(trie, buf);
		assert(*val == i);
	}
	trie_free(trie);
}
void test_trie_remove(void)
{
	Trie *trie;
	char buf[10];
	int i;
	unsigned int entries;
	trie = generate_trie();
	assert(trie_remove(trie, "000000000000000") == 0);
	assert(trie_remove(trie, "") == 0);
	entries = trie_num_entries(trie);
	assert(entries == NUM_TEST_VALUES);
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		sprintf(buf, "%i", i);
		assert(trie_remove(trie, buf) != 0);
		--entries;
		assert(trie_num_entries(trie) == entries);
	}
	trie_free(trie);
}
void test_trie_replace(void)
{
	Trie *trie;
	int *val;
	trie = generate_trie();
	val = malloc(sizeof(int));
	*val = 999;
	assert(trie_insert(trie, "999", val) != 0);
	assert(trie_num_entries(trie) == NUM_TEST_VALUES);
	assert(trie_lookup(trie, "999") == val);
	free(val);
	trie_free(trie);
}
void test_trie_insert_empty(void)
{
	Trie *trie;
	char buf[10];
	trie = trie_new();
	assert(trie_insert(trie, "", buf) != 0);
	assert(trie_num_entries(trie) != 0);
	assert(trie_lookup(trie, "") == buf);
	assert(trie_remove(trie, "") != 0);
	assert(trie_num_entries(trie) == 0);
	trie_free(trie);
}
#define LONG_STRING_LEN 4096
static void test_trie_free_long(void)
{
	char *long_string;
	Trie *trie;
	long_string = malloc(LONG_STRING_LEN);
	memset(long_string, 'A', LONG_STRING_LEN);
	long_string[LONG_STRING_LEN - 1] = '\0';
	trie = trie_new();
	trie_insert(trie, long_string, long_string);
	trie_free(trie);
	free(long_string);
}
static void test_trie_negative_keys(void)
{
	char my_key[] = { 'a', 'b', 'c', -50, -20, '\0' };
	Trie *trie;
	void *value;
	trie = trie_new();
	assert(trie_insert(trie, my_key, "hello world") != 0);
	value = trie_lookup(trie, my_key);
	assert(!strcmp(value, "hello world"));
	assert(trie_remove(trie, my_key) != 0);
	assert(trie_remove(trie, my_key) == 0);
	assert(trie_lookup(trie, my_key) == NULL);
	trie_free(trie);
}
Trie *generate_binary_trie(void)
{
	Trie *trie;
	trie = trie_new();
	assert(trie_insert_binary(trie,
	                          bin_key2, sizeof(bin_key2),
	                          "goodbye world") != 0);
	assert(trie_insert_binary(trie,
	                          bin_key, sizeof(bin_key),
	                          "hello world") != 0);
	return trie;
}
void test_trie_insert_binary(void)
{
	Trie *trie;
	char *value;
	trie = generate_binary_trie();
	assert(trie_insert_binary(trie,
	                          bin_key, sizeof(bin_key),
	                          "hi world") != 0);
	assert(trie_insert_binary(trie, bin_key3,
	                          sizeof(bin_key3), NULL) == 0);
	value = trie_lookup_binary(trie, bin_key, sizeof(bin_key));
	assert(!strcmp(value, "hi world"));
	value = trie_lookup_binary(trie, bin_key2, sizeof(bin_key2));
	assert(!strcmp(value, "goodbye world"));
	trie_free(trie);
}
void test_trie_insert_out_of_memory(void)
{
	Trie *trie;
	trie = generate_binary_trie();
	alloc_test_set_limit(3);
	assert(trie_insert_binary(trie,
	                          bin_key4, sizeof(bin_key4),
	                          "test value") == 0);
	assert(trie_lookup_binary(trie, bin_key4, sizeof(bin_key4)) == NULL);
	assert(trie_num_entries(trie) == 2);
	trie_free(trie);
}
void test_trie_remove_binary(void)
{
	Trie *trie;
	void *value;
	trie = generate_binary_trie();
	value = trie_lookup_binary(trie, bin_key3, sizeof(bin_key3));
	assert(value == NULL);
	assert(trie_remove_binary(trie, bin_key3, sizeof(bin_key3)) == 0);
	assert(trie_lookup_binary(trie, bin_key4, sizeof(bin_key4)) == 0);
	assert(trie_remove_binary(trie, bin_key4, sizeof(bin_key4)) == 0);
	assert(trie_remove_binary(trie, bin_key2, sizeof(bin_key2)) != 0);
	assert(trie_lookup_binary(trie, bin_key2, sizeof(bin_key2)) == NULL);
	assert(trie_lookup_binary(trie, bin_key, sizeof(bin_key)) != NULL);
	assert(trie_remove_binary(trie, bin_key, sizeof(bin_key)) != 0);
	assert(trie_lookup_binary(trie, bin_key, sizeof(bin_key)) == NULL);
	trie_free(trie);
}
static UnitTestFunction tests[] = {
	test_trie_new_free,
	test_trie_insert,
	test_trie_lookup,
	test_trie_remove,
	test_trie_replace,
	test_trie_insert_empty,
	test_trie_free_long,
	test_trie_negative_keys,
	test_trie_insert_binary,
	test_trie_insert_out_of_memory,
	test_trie_remove_binary,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
