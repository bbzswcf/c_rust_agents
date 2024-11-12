#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "hash-pointer.h"
#include "hash-int.h"
#include "hash-string.h"
#define NUM_TEST_VALUES 200
void test_pointer_hash(void)
{
	int array[NUM_TEST_VALUES];
	int i, j;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		array[i] = 0;
	}
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		for (j=i+1; j<NUM_TEST_VALUES; ++j) {
			assert(pointer_hash(&array[i])
			       != pointer_hash(&array[j]));
		}
	}
}
void test_int_hash(void)
{
	int array[NUM_TEST_VALUES];
	int i, j;
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		array[i] = i;
	}
	for (i=0; i<NUM_TEST_VALUES; ++i) {
		for (j=i+1; j<NUM_TEST_VALUES; ++j) {
			assert(int_hash(&array[i]) != int_hash(&array[j]));
		}
	}
	i = 5000;
	j = 5000;
	assert(int_hash(&i) == int_hash(&j));
}
void test_string_hash(void)
{
	char test1[] = "this is a test";
	char test2[] = "this is a tesu";
	char test3[] = "this is a test ";
	char test4[] = "this is a test";
	char test5[] = "This is a test";
	assert(string_hash(test1) != string_hash(test2));
	assert(string_hash(test1) != string_hash(test3));
	assert(string_hash(test1) != string_hash(test5));
	assert(string_hash(test1) == string_hash(test4));
}
void test_string_nocase_hash(void)
{
	char test1[] = "this is a test";
	char test2[] = "this is a tesu";
	char test3[] = "this is a test ";
	char test4[] = "this is a test";
	char test5[] = "This is a test";
	assert(string_nocase_hash(test1) != string_nocase_hash(test2));
	assert(string_nocase_hash(test1) != string_nocase_hash(test3));
	assert(string_nocase_hash(test1) == string_nocase_hash(test5));
	assert(string_nocase_hash(test1) == string_nocase_hash(test4));
}
static UnitTestFunction tests[] = {
	test_pointer_hash,
	test_int_hash,
	test_string_hash,
	test_string_nocase_hash,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
