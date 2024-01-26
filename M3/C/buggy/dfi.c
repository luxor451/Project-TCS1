/**
 * @brief Implementation of some dummy functions about grades...
 *
 * @file dfi.c
 *
 * @author Christophe Garion
 */

#include <stdio.h>

#include "dfi.h"

void print_repass(int size, struct grade array[size]) {
    // maximum number of students that have to repass the exam
    // Should be reachable for TCS3-MF
    struct grade repass_array[size];

    int j = 0;

    for (int i = 0; i < size; i++) {
        if (! is_valid(array[i])) {
            repass_array[j++] = array[i];
        }
    }

    printf("Students that have to repass %s exam...\n", array[size - 1].course);

    for (int i = 0; i < j; i++) {
            printf("Student %s must repass the %s exam!\n",
                   repass_array[i].name,
                   repass_array[i].course);
    }
}
