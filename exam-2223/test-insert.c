#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "test_utils.h"
#include "ring_list.h"

extern int array_single_10[];
extern int array_1_10[];

void test_insert_create_singleton() {
    ring_list l1 = create_empty_ring();
    l1 = insert(l1, 2);

    cell_int *p_cell = l1;

    assert(p_cell->value == 2);
    assert(p_cell->p_next == l1);
}

int main(void) {
    printf("--- tests for insert\n");

    printf("    inserting value 2 in empty list\n");
    test_insert_create_singleton();

    printf("    inserting value 10 in list [1]\n");
    ring_list singleton = singleton_list();
    assert(same_sequence(insert(singleton, 10), array_single_10, 2));

    printf("    inserting value 10 in list [1, 2, 3, 4]\n");
    ring_list l = simple_list_1();
    assert(same_sequence(insert(l, 10), array_1_10, 10));

    printf("--- tests for insert: OK!\n");

    return EXIT_SUCCESS;
}
