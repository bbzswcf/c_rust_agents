#ifndef ALGORITHM_COMPARE_POINTER_H
#define ALGORITHM_COMPARE_POINTER_H
#ifdef __cplusplus
extern "C" {
#endif
int pointer_equal(void *location1, void *location2);
int pointer_compare(void *location1, void *location2);
#ifdef __cplusplus
}
#endif
#endif 
#include "compare-pointer.h"
int pointer_equal(void *location1, void *location2)
{
	return location1 == location2;
}
int pointer_compare(void *location1, void *location2)
{
	if (location1 < location2) {
		return -1;
	} else if (location1 > location2) {
		return 1;
	} else {
		return 0;
	}
}
