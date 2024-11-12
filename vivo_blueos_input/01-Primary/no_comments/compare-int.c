#ifndef ALGORITHM_COMPARE_INT_H
#define ALGORITHM_COMPARE_INT_H
#ifdef __cplusplus
extern "C" {
#endif
int int_equal(void *location1, void *location2);
int int_compare(void *location1, void *location2);
#ifdef __cplusplus
}
#endif
#endif 
#include "compare-int.h"
int int_equal(void *vlocation1, void *vlocation2)
{
	int *location1;
	int *location2;
	location1 = (int *) vlocation1;
	location2 = (int *) vlocation2;
	return *location1 == *location2;
}
int int_compare(void *vlocation1, void *vlocation2)
{
	int *location1;
	int *location2;
	location1 = (int *) vlocation1;
	location2 = (int *) vlocation2;
	if (*location1 < *location2) {
		return -1;
	} else if (*location1 > *location2) {
		return 1;
	} else {
		return 0;
	}
}
