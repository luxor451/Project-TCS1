#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"
#include "test_utils.h"

extern int array_1_1[];
extern int array_1_2[];
extern int array_2[];
extern int array_3[];
extern int array_4[];
extern int array_4_10[];

void test_enqueue(int e, int nb, int expected_array[]) {
    printf("    enqueing %d in ", e);

    queue_int q = create_queue(nb);

    print_queue(q);

    enqueue(&q, e);

    printf(", got ");
    print_queue(q);

    assert(same_sequence(q, expected_array, nb + 1));

    printf("\n");
}

int main(void) {
    printf("--- tests for enqueue\n");

    test_enqueue(1, 0, array_1_1);
    test_enqueue(2, 0, array_1_2);

    test_enqueue(2, 1, array_2);

    test_enqueue(3, 2, array_3);

    test_enqueue(4, 3, array_4);
    test_enqueue(10, 3, array_4_10);

    printf("--- tests for enqueue: OK!\n");

    return EXIT_SUCCESS;
}
