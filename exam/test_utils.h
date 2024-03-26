#ifndef TEST_UTILS_H
#define TEST_UTILS_H

#include <stdbool.h>

#include "queue_int.h"

queue_int create_queue(int size);

bool same_queue(queue_int queue_1, queue_int queue_2);

bool same_sequence(queue_int queue, int array[], int size);

bool same_continuous_sequence(queue_int queue, int start, int size);

void print_queue(queue_int queue);

#endif
