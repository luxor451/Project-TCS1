#include <stdlib.h>
#include <stdio.h>
#include "test_utils.h"

int array_single[] = { 2, 2, 2 };
int array_single_10[] = { 2, 10, 2, 10 };
int array_1[] = { 1, 2, 3, 4, 1, 2, 3, 4 };
int array_2[] = { 5, 6, 7, 5, 6, 7 };
int array_1_10[] = { 1, 10, 2, 3, 4, 1, 10, 2, 3, 4 };
int array_1_r_1[] = { 2, 3, 4, 1, 2, 3, 4, 1 };
int array_1_r_2[] = { 3, 4, 1, 2, 3, 4, 1, 2 };
int array_1_r_3[] = { 4, 1, 2, 3, 4, 1, 2, 3 };
int array_a[] = { 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7 };
int array_a_1[] = { 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6, 7 };
int array_a_2[] = { 5, 6, 7, 1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4 };

ring_list empty_list() {
    return NULL;
}

ring_list singleton_list() {
    cell_int *p_cell = malloc(sizeof(cell_int));

    p_cell->value  = 2;
    p_cell->p_next = p_cell;

    return p_cell;
}

ring_list simple_list_1() {
    cell_int *p_cell_1 = malloc(sizeof(cell_int));
    cell_int *p_cell_2 = malloc(sizeof(cell_int));
    cell_int *p_cell_3 = malloc(sizeof(cell_int));
    cell_int *p_cell_4 = malloc(sizeof(cell_int));

    p_cell_1->value  = 1;
    p_cell_1->p_next = p_cell_2;

    p_cell_2->value  = 2;
    p_cell_2->p_next = p_cell_3;

    p_cell_3->value  = 3;
    p_cell_3->p_next = p_cell_4;

    p_cell_4->value  = 4;
    p_cell_4->p_next = p_cell_1;

    return p_cell_1;
}

ring_list simple_list_2() {
    cell_int *p_cell_1 = malloc(sizeof(cell_int));
    cell_int *p_cell_2 = malloc(sizeof(cell_int));
    cell_int *p_cell_3 = malloc(sizeof(cell_int));

    p_cell_1->value  = 5;
    p_cell_1->p_next = p_cell_2;

    p_cell_2->value  = 6;
    p_cell_2->p_next = p_cell_3;

    p_cell_3->value  = 7;
    p_cell_3->p_next = p_cell_1;

    return p_cell_1;
}

ring_list simple_list_3() {
    cell_int *p_cell_1 = malloc(sizeof(cell_int));
    cell_int *p_cell_2 = malloc(sizeof(cell_int));
    cell_int *p_cell_3 = malloc(sizeof(cell_int));
    cell_int *p_cell_4 = malloc(sizeof(cell_int));

    p_cell_1->value  = 1;
    p_cell_1->p_next = p_cell_2;

    p_cell_2->value  = 2;
    p_cell_2->p_next = p_cell_3;

    p_cell_3->value  = 1;
    p_cell_3->p_next = p_cell_4;

    p_cell_4->value  = 4;
    p_cell_4->p_next = p_cell_1;

    return p_cell_1;
}

bool same_sequence(ring_list list, int array[], int size) {
    int i = 0;

    cell_int *p_cell = list;

    while (i != size) {
        if (! (p_cell->value == array[i])) {
            printf("ERROR: expecting %d at position %d, get %d!\n",
                   array[i], i, p_cell->value);

            return false;
        }

        p_cell = p_cell->p_next;
        i++;
    }

    return true;
}
