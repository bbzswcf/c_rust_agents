#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
#include "compare-int.h"
#include "compare-pointer.h"
#include "compare-string.h"
void test_int_compare(void)
{
	int a = 4;
	int b = 8;
	int c = 4;
	assert(int_compare(&a, &b) < 0);
	assert(int_compare(&b, &a) > 0);
	assert(int_compare(&a, &c) == 0);
}
void test_int_equal(void)
{
	int a = 4;
	int b = 8;
	int c = 4;
	assert(int_equal(&a, &c) != 0);
	assert(int_equal(&a, &b) == 0);
}
void test_pointer_compare(void)
{
	int array[5];
	assert(pointer_compare(&array[0], &array[4]) < 0);
	assert(pointer_compare(&array[3], &array[2]) > 0);
	assert(pointer_compare(&array[4], &array[4]) == 0);
}
void test_pointer_equal(void)
{
	int a, b;
	assert(pointer_equal(&a, &a) != 0);
	assert(pointer_equal(&a, &b) == 0);
}
void test_string_compare(void)
{
	char test1[] = "Apple";
	char test2[] = "Orange";
	char test3[] = "Apple";
	assert(string_compare(test1, test2) < 0);
	assert(string_compare(test2, test1) > 0);
	assert(string_compare(test1, test3) == 0);
}
void test_string_equal(void)
{
	char test1[] = "this is a test string";
	char test2[] = "this is a test string ";
	char test3[] = "this is a test strin";
	char test4[] = "this is a test strinG";
	char test5[] = "this is a test string";
	assert(string_equal(test1, test5) != 0);
	assert(string_equal(test1, test2) == 0);
	assert(string_equal(test1, test3) == 0);
	assert(string_equal(test1, test4) == 0);
}
void test_string_nocase_compare(void)
{
	char test1[] = "Apple";
	char test2[] = "Orange";
	char test3[] = "Apple";
	char test4[] = "Alpha";
	char test5[] = "bravo";
	char test6[] = "Charlie";
	assert(string_nocase_compare(test1, test2) < 0);
	assert(string_nocase_compare(test2, test1) > 0);
	assert(string_nocase_compare(test1, test3) == 0);
	assert(string_nocase_compare(test4, test5) < 0);
	assert(string_nocase_compare(test5, test6) < 0);
}
void test_string_nocase_equal(void)
{
	char test1[] = "this is a test string";
	char test2[] = "this is a test string ";
	char test3[] = "this is a test strin";
	char test4[] = "this is a test strinG";
	char test5[] = "this is a test string";
	assert(string_nocase_equal(test1, test5) != 0);
	assert(string_nocase_equal(test1, test2) == 0);
	assert(string_nocase_equal(test1, test3) == 0);
	assert(string_nocase_equal(test1, test4) != 0);
}
static UnitTestFunction tests[] = {
	test_int_compare,
	test_int_equal,
	test_pointer_compare,
	test_pointer_equal,
	test_string_compare,
	test_string_equal,
	test_string_nocase_compare,
	test_string_nocase_equal,
	NULL
};
int main(int argc, char *argv[])
{
	run_tests(tests);
	return 0;
}
