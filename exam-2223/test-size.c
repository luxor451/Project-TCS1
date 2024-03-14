#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "test_utils.h"
#include "ring_list.h"

int main(void) {
    printf("--- tests for size\n");

    printf("    size of empty list\n");
    assert(size(create_empty_ring()) == 0);
    printf("    size of list [2]\n");
    assert(size(singleton_list()) == 1);
    printf("    size of list [1, 2, 3, 4]\n");
    assert(size(simple_list_1()) == 4);
    printf("    size of list [1, 2, 1, 4]\n");
    assert(size(simple_list_3()) == 4);

    printf("--- tests for size: OK!\n");

    return EXIT_SUCCESS;
}
