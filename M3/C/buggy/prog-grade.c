/**
 * @brief Program using dummy functions on grades...
 *
 * @file prog-grade.c
 *
 * @author Christophe Garion
 */

#include <stdio.h>

#include "grade.h"
#include "dfi.h"

int main() {
    struct grade g1 = { .name = "Ausias",
                        .course = "TCS3-IN",
                        .grade = 4.2 };

    struct grade g2 = { .name = "Christophe",
                        .course = "TCS3-IN",
                        .grade = 19.8 };

    struct grade g3 = { .name = "Caroline",
                        .course = "TCS3-IN",
                        .grade = 10.0 };

    struct grade g4 = { .name = "Anne",
                        .course = "TCS3-IN",
                        .grade = 8.5 };

    print_grade(g1);
    print_grade(g2);
    print_grade(g3);
    print_grade(g4);

    printf("\n");

    struct grade array[] = { g1, g2, g3, g4 };

    print_repass(4, array);

    return 0;
}
