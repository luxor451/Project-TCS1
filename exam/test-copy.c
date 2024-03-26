#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"
#include "test_utils.h"

extern int array_1_1[];
extern int array_1_2[];
extern int array_4[];
extern int array_a_1_2[];
extern int array_a_2_1[];
extern int array_a_3_4[];
extern int array_a_4_3[];

void test_copy(int nb) {
    queue_int q1 = create_queue(nb);
    queue_int q2 = copy(q1);

    printf("    copying ");
    print_queue(q1);
    printf(", got ");
    print_queue(q2);

    assert(same_queue(q1, q2));

    printf("\n");
}

int main(void) {
    printf("--- tests for copy\n");

    test_copy(0);
    test_copy(1);
    test_copy(2);
    test_copy(3);
    test_copy(4);

    printf("--- tests for copy: OK!\n");

    return EXIT_SUCCESS;
}
