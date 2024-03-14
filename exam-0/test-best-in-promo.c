#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "grading.h"

char *tab_fn[16] = {"C.", "P.", "T.", "E.", "F.", "J.-L.", "A.", "B.", "P.", "O.", "O.", "A.", "J.", "V.", "E.", "T."};
char *tab_ln[16] = {"Garion", "Siron", "Perennou", "Lochin", "Frances", "Bussenot", "Brunet", "Crusson", "Schmidt", "Poitou", "Rousselot", "Hamez", "Lacan", "Vidal", "Noulard", "Bridel"};

void change_grade_and_test(promo *p_promo, student *tab_std[], int index, double new_grade) {
    grade(tab_std[index], new_grade);

    student *best = best_in_promo(p_promo);

    assert (strcmp(best->first_name, tab_fn[index]) == 0);
    assert (strcmp(best->last_name, tab_ln[index]) == 0);
}

void change_grade_and_test_ko(promo *p_promo, student *tab_std[], int index, double new_grade) {
    grade(tab_std[index], new_grade);

    student *best = best_in_promo(p_promo);

    assert (strcmp(best->first_name, tab_fn[index]) != 0);
    assert (strcmp(best->last_name, tab_ln[index]) != 0);
}

int main(void) {
    promo *my_promo = create_promo(1930);

    student *tab_std[16];

    for (int i = 0; i < 16; i++) {
        tab_std[i] = create_student(tab_fn[i], tab_ln[i]);
        add_in_pc(tab_std[i], &(my_promo->pcs_tab[i/2]));
    }

    change_grade_and_test(my_promo, tab_std, 0, 15);
    change_grade_and_test(my_promo, tab_std, 15, 16);
    change_grade_and_test(my_promo, tab_std, 5, 17);
    change_grade_and_test_ko(my_promo, tab_std, 4, 15);

    printf("OK for best_in_promo!\n");

    return 0;
}
