#ifndef TEST_UTILS_H
#define TEST_UTILS_H

#include <stdbool.h>

#include "ring_list.h"

ring_list empty_list();
ring_list singleton_list();
ring_list simple_list_1();
ring_list simple_list_2();
ring_list simple_list_3();
bool same_sequence(ring_list list, int array[], int size);

#endif
