#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

#include "test_utils.h"

int array_0[]    = {};
int array_1_1[]  = { 1 };
int array_1_2[]  = { 2 };
int array_2[]    = { 1, 2 };
int array_2_1[]  = { 2 };
int array_3[]    = { 1, 2, 3 };
int array_3_2[]  = { 2, 3 };
int array_3_1[]  = { 3 };
int array_4[]    = { 1, 2, 3, 4 };
int array_4_3[]  = { 2, 3, 4 };
int array_4_2[]  = { 3, 4 };
int array_4_1[]  = { 4 };
int array_4_10[] = { 1, 2, 3, 10 };

int array_a_1_2[] = { 1, 1, 2 };
int array_a_2_1[] = { 1, 2, 1 };
int array_a_3_4[] = { 1, 2, 3, 1, 2, 3, 4 };
int array_a_4_3[] = { 1, 2, 3, 4, 1, 2, 3 };

queue_int create_queue(int size) {
    queue_int q;

    q.size  = size;
    q.p_first = NULL;
    q.p_last  = NULL;

    cell_int *p_old_cell = NULL;
    cell_int *p_new_cell = NULL;

    for (int i = 0; i < size; i++) {
        p_new_cell = malloc(sizeof(cell_int));

        assert(p_new_cell != NULL);

        p_new_cell->value  = i + 1;
        p_new_cell->p_next = NULL;

        if (i == 0) {
            q.p_first = p_new_cell;
        } else {
            p_old_cell->p_next = p_new_cell;
        }

        if (i == size - 1) {
            q.p_last = p_new_cell;
        }

        p_old_cell = p_new_cell;
    }

    return q;
}

bool same_queue(queue_int queue_1, queue_int queue_2) {
    if (queue_1.size != queue_2.size) {
        return false;
    }

    cell_int *p_cell_1 = queue_1.p_first;
    cell_int *p_cell_2 = queue_2.p_first;
    int       i        = 0;

    while (p_cell_1 != NULL) {
        if (p_cell_1 == p_cell_2) {
            printf("\n       ERROR: cells at position %d are located in the same memory region (%p)!\n",
                   i, p_cell_1);

            return false;
        }

        if (p_cell_1->value != p_cell_2->value) {
            printf("\n      ERROR: expecting %d at position %d, get %d!\n",
                   p_cell_2->value, i, p_cell_1->value);

            return false;
        }

        p_cell_1 = p_cell_1->p_next;
        p_cell_2 = p_cell_2->p_next;
        i++;
    }

    if (queue_1.p_last != NULL && queue_1.p_last->value != queue_2.p_last->value) {
        printf("\n      ERROR: expecting %d at LAST position, get %d!\n",
               queue_2.p_last->value, queue_1.p_last->value);

        return false;
    }

    if (queue_1.p_last != NULL && queue_1.p_last == queue_2.p_last) {
        printf("\n      ERROR: cells at LAST position are pointing to the same memory region (%p)!\n",
               queue_2.p_last);

        return false;
    }

    return true;
}

bool same_sequence(queue_int queue, int array[], int size) {
    int i = 0;

    cell_int *p_cell = queue.p_first;

    while (i != size) {
        if (! (p_cell->value == array[i])) {
            printf("\n      ERROR: expecting %d at position %d, get %d!\n",
                   array[i], i, p_cell->value);

            return false;
        }

        if ((i == size - 1) && queue.p_last->value != array[i]) {
            printf("\n      ERROR: expecting %d at LAST position, get %d!\n",
                   array[i], p_cell->value);

            return false;
        }

        p_cell = p_cell->p_next;
        i++;
    }

    return true;
}

bool same_continuous_sequence(queue_int queue, int start, int size) {
    int i = 0;

    cell_int *p_cell = queue.p_first;

    while (i != size) {
        if (! (p_cell->value == start + i + 1)) {
            printf("\n      ERROR: expecting %d at position %d, get %d!\n",
                   start + i, i, p_cell->value);

            return false;
        }

        if ((i == size - 1) && queue.p_last->value != start + i + 1) {
            printf("\n      ERROR: expecting %d at LAST position, get %d!\n",
                   start + i, p_cell->value);

            return false;
        }

        p_cell = p_cell->p_next;
        i++;
    }

    return true;
}

void print_queue(queue_int queue) {
    if (queue.p_first == NULL) {
        printf("empty queue");

        return;
    }

    printf("<-");

    cell_int *p_cell = queue.p_first;

    while (p_cell != NULL) {
        printf(" %d", p_cell->value);
        p_cell = p_cell->p_next;
    }

    printf(" <-");
}
