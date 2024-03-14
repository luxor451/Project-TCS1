#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#include "test_utils.h"
#include "ring_list.h"

extern int array_single[];
extern int array_1[];
extern int array_1_r_1[];
extern int array_1_r_2[];
extern int array_1_r_3[];

void test_rotate_empty_list() {
    ring_list list = create_empty_ring();

    for (int i = 0; i < 5; i++) {
        assert(is_empty(rotate(list, i)));
    }
}

void test_rotate_singleton_list() {
    ring_list list = singleton_list();

    for (int i = 0; i < 5; i++) {
        assert(same_sequence(rotate(list, i), array_single, 3));
    }
}

void test_rotate_list_1() {
    ring_list list = simple_list_1();

    assert(same_sequence(rotate(list, 0), array_1, 8));
    assert(same_sequence(rotate(list, 1), array_1_r_1, 8));
    assert(same_sequence(rotate(list, 2), array_1_r_2, 8));
    assert(same_sequence(rotate(list, 3), array_1_r_3, 8));
    assert(same_sequence(rotate(list, 4), array_1, 8));
}

int main(void) {
    printf("--- tests for rotate\n");

    printf("    rotating empty list\n");
    test_rotate_empty_list();

    printf("    rotating list [1]\n");
    test_rotate_singleton_list();

    printf("    rotating list [1, 2, 3, 4]\n");
    test_rotate_list_1();

    printf("--- tests for rotate: OK!\n");

    return EXIT_SUCCESS;
}
