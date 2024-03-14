#include <stdio.h>
#include <stdlib.h>
#include "grading.h"

promo *create_promo(int year) {
    promo *p_promo = malloc(sizeof(promo));

    p_promo->year = year;

    return p_promo;
}

student *create_student(char *first_name, char *last_name) {
    student *p_std = malloc(sizeof(student));

    p_std->first_name = first_name;
    p_std->last_name  = last_name;
    p_std->grade      = -1;

    return p_std;
}

void grade(student *p_student, double grade) {
    p_student->grade = grade;
}

void add_in_pc(student *p_student, pc *p_pc) {
    p_pc->students_tab[p_pc->nb_students] = p_student;
    p_pc->nb_students = p_pc->nb_students + 1;
}

int nb_of_students(promo *p_promo) {
    int nb = 0;

    for (int i = 0; i < 8; i++) {
        nb = nb + p_promo->pcs_tab[i].nb_students;
    }

    return nb;
}

student *best_in_promo(promo *p_promo) {
    double   best_grade = -1;
    student *p_best_std = NULL;

    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < p_promo->pcs_tab[i].nb_students; j++) {
            if (p_promo->pcs_tab[i].students_tab[j]->grade > best_grade) {
                p_best_std = p_promo->pcs_tab[i].students_tab[j];
                best_grade = p_promo->pcs_tab[i].students_tab[j]->grade;
            }
        }
    }

    return p_best_std;
}

student **sort_by_grade(promo *p_promo) {
    int nb_std = nb_of_students(p_promo);

    student **tab_std = malloc(nb_std * sizeof(student *));

    int index = 0;

    for (int i = 0; i < 8; i++) {
        for (int j = 0; j < p_promo->pcs_tab[i].nb_students; j++) {
            tab_std[index] = p_promo->pcs_tab[i].students_tab[j];
            index = index + 1;
        }
    }

    for (int i = 0; i < nb_std; i++) {
        int index_max = i;

        for (int j = i; j < nb_std; j++) {
            if (tab_std[index_max]->grade < tab_std[j]->grade) {
                index_max = j;
            }
        }

        student * p_temp = tab_std[i];
        tab_std[i] = tab_std[index_max];
        tab_std[index_max] = p_temp;
    }

    return tab_std;
}
