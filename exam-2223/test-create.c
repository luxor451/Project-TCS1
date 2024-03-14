#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "ring_list.h"

int main(void) {
    printf("--- tests for create_empty_ring\n");

    assert(create_empty_ring() == NULL);

    printf("--- tests for create_empty_ring: OK!\n");

    return EXIT_SUCCESS;
}
