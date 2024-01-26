/**
 * @brief Implementation of some dummy functions about grade...
 *
 * @file grade.c
 *
 * @author Christophe Garion
 */

#include <stdio.h>

#include "grade.h"

void print_grades(struct grade the_grade) {
    printf("Student %s has obtained the grade %.1f for course %s\n",
           the_grade.name, the_grade.grade, the_grade.course);
}

bool is_valid(struct grade the_grade) {
    return the_grade.grade >= 10;
}
