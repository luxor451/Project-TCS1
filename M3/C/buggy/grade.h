/**
 * @brief Some dummy functions about grade...
 *
 * @file grade.h
 *
 * @author Christophe Garion
 */

/**
 * @brief a structure to represent grade
 */

#ifndef GRADE_H
#define GRADE_H
#include <stdbool.h>
struct grade {
    /** the name of the student */
    char * name;
    /** the course */
    char * course;
    /** the grade of the student for the course */
    double grade;
};

/**
 * @brief Print a grade
 *
 * @param the_grade the grade to be printed
 */
void print_grade(struct grade the_grade);

/**
 * @brief Is the grade sufficient to validate the course?
 *
 * @param the_grade the grade to be verified
 *
 * @return `true` if the grade is valid, `false` otherwise
 *
 */
bool is_valid(struct grade the_grade);

#endif