#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"
#include "test_utils.h"

extern int array_0[];
extern int array_1_1[];
extern int array_1_2[];
extern int array_2[];
extern int array_3[];
extern int array_4[];
extern int array_4_10[];

void test_dequeue(int nb) {
    queue_int q = create_queue(nb);

    int v = 0;

    for (int i = 1; i <= nb; i++) {
        printf("    dequeing ");
        print_queue(q);

        v = dequeue(&q);

        printf(", got value %d and queue is now ", v);
        print_queue(q);

        assert(v == i);
        same_continuous_sequence(q, i, nb - i);

        assert(q.size == nb - i);

        printf("\n");
    }
}

int main(void) {
    printf("--- tests for dequeue\n");

    test_dequeue(1);
    test_dequeue(2);
    test_dequeue(3);
    test_dequeue(4);

    printf("--- tests for dequeue: OK!\n");

    return EXIT_SUCCESS;
}
