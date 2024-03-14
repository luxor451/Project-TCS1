#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "test_utils.h"
#include "ring_list.h"

int main(void) {
    printf("--- tests for is_empty\n");

    assert(is_empty(empty_list()));
    assert(! is_empty(singleton_list()));
    assert(! is_empty(simple_list_1()));

    printf("--- tests for is_empty: OK!\n");

    return EXIT_SUCCESS;
}
