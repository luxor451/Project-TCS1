#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "queue_int.h"

int main(void) {
    printf("--- tests for nil\n");

    assert(nil().p_first == NULL);
    assert(nil().p_last  == NULL);
    assert(nil().size  == 0);

    printf("--- tests for nil: OK!\n");

    return EXIT_SUCCESS;
}
