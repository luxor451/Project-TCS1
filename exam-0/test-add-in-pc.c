#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "grading.h"

char *tab_fn[3] = {"Christophe", "Pierre", "Tanguy"};
char *tab_ln[3] = {"Garion", "Siron", "Perennou"};

int main(void) {
    pc my_pc;

    my_pc.nb_students = 0;

    student *tab_std[3];

    for (int i = 0; i < 3; i++) {
        tab_std[i] = create_student(tab_fn[i], tab_ln[i]);
        add_in_pc(tab_std[i], &my_pc);
    }

    for (int i = 0; i < 3; i++) {
        assert (my_pc.students_tab[i] != NULL);
        assert (strcmp(my_pc.students_tab[i]->first_name, tab_fn[i]) == 0);
        assert (strcmp(my_pc.students_tab[i]->last_name, tab_ln[i]) == 0);
    }

    assert (my_pc.nb_students == 3);

    printf("OK for add_in_pc!\n");

    return 0;
}
