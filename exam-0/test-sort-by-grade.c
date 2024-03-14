#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "grading.h"

char *tab_fn[16] = {"C.", "P.", "T.", "E.", "F.", "J.-L.", "A.", "B.", "P.", "O.", "O.", "A.", "J.", "V.", "E.", "T."};
char *tab_ln[16] = {"Garion", "Siron", "Perennou", "Lochin", "Frances", "Bussenot", "Brunet", "Crusson", "Schmidt", "Poitou", "Rousselot", "Hamez", "Lacan", "Vidal", "Noulard", "Bridel"};
double tab_grade[16] = { 3, 12, 11, 15, 17, 14, 20, 19, 16, 13, 12, 19, 13, 17, 17, 20 };

int main(void) {
    promo *my_promo = create_promo(1930);

    student *tab_std[16];

    for (int i = 0; i < 16; i++) {
        tab_std[i] = create_student(tab_fn[i], tab_ln[i]);
        grade(tab_std[i], tab_grade[i]);
        add_in_pc(tab_std[i], &(my_promo->pcs_tab[i/2]));
    }

    student **tab_sorted = sort_by_grade(my_promo);

    for (int i = 0; i < 16; i++) {
        printf("%s %s, %.0f\n", tab_sorted[i]->first_name,
               tab_sorted[i]->last_name, tab_sorted[i]->grade);
    }

    return 0;
}
