#ifndef ALGORITHM_HASH_INT_H
#define ALGORITHM_HASH_INT_H
#ifdef __cplusplus
extern "C" {
#endif
unsigned int int_hash(void *location);
#ifdef __cplusplus
}
#endif
#endif 
#include "hash-int.h"
unsigned int int_hash(void *vlocation)
{
	int *location;
	location = (int *) vlocation;
	return (unsigned int) *location;
}
