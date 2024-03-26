#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"
#include "test_utils.h"

void test_is_empty(int size) {
    printf("    testing is_empty for ");

    queue_int q = create_queue(size);
    print_queue(q);

    assert(size == 0 ? is_empty(q) : ! is_empty(q));

    printf("\n");
}

int main(void) {
    printf("--- tests for is_empty\n");

    test_is_empty(0);
    test_is_empty(1);
    test_is_empty(2);

    printf("--- tests for is_empty: OK!\n");

    return EXIT_SUCCESS;
}
