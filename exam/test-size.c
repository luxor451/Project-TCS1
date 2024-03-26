#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"
#include "test_utils.h"

void test_size(int nb) {
    printf("    testing size for ");

    queue_int q = create_queue(nb);
    print_queue(q);

    assert(size(q) == nb);

    printf("\n");
}

int main(void) {
    printf("--- tests for size\n");

    test_size(0);
    test_size(1);
    test_size(2);
    test_size(3);

    printf("--- tests for size: OK!\n");

    return EXIT_SUCCESS;
}
