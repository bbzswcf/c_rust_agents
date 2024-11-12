#ifndef TEST_FRAMEWORK_H
#define TEST_FRAMEWORK_H
#ifdef __cplusplus
extern "C" {
#endif
typedef void (*UnitTestFunction)(void);
void run_tests(UnitTestFunction *tests);
#ifdef __cplusplus
}
#endif
#endif 
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include "alloc-testing.h"
#include "framework.h"
static void run_test(UnitTestFunction test)
{
	alloc_test_set_limit(-1);
	test();
	assert(alloc_test_get_allocated() == 0);
}
void run_tests(UnitTestFunction *tests)
{
	int i;
	for (i=0; tests[i] != NULL; ++i) {
		run_test(tests[i]);
	}
}
