#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "test_utils.h"
#include "ring_list.h"

extern int array_1[];
extern int array_2[];
extern int array_a[];
extern int array_a_1[];
extern int array_a_2[];

void test_insert_create_singleton() {
    ring_list l1 = create_empty_ring();
    l1 = insert(l1, 2);

    cell_int *p_cell = l1;

    assert(p_cell->value == 2);
    assert(p_cell->p_next == l1);
}

int main(void) {
    printf("--- tests for append\n");

    printf("    appending empty list to empty_list\n");
    assert(is_empty(append(create_empty_ring(), create_empty_ring())));

    printf("    appending empty list to [5, 6, 7]\n");
    assert(same_sequence(append(create_empty_ring(), simple_list_2()), array_2, 6));

    printf("    appending [1, 2, 3, 4] to empty_list\n");
    assert(same_sequence(append(simple_list_1(), create_empty_ring()), array_1, 8));

    printf("    appending [1, 2, 3, 4] to [5, 6, 7]\n");
    ring_list list_1 = simple_list_1();
    ring_list list_2 = simple_list_2();
    assert(same_sequence(append(list_1, list_2), array_a, 14));
    assert(same_sequence(list_1, array_a_1, 14));
    assert(same_sequence(list_2, array_a_2, 14));
    printf("--- tests for append: OK!\n");

    return EXIT_SUCCESS;
}
