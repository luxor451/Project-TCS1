#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "grading.h"

char *tab_fn[16] = {"C.", "P.", "T.", "E.", "F.", "J.-L.", "A.", "B.", "P.", "O.", "O.", "A.", "J.", "V.", "E.", "T."};
char *tab_ln[16] = {"Garion", "Siron", "Perennou", "Lochin", "Frances", "Bussenot", "Brunet", "Crusson", "Schmidt", "Poitou", "Rousselot", "Hamez", "Lacan", "Vidal", "Noulard", "Bridel"};

int main(void) {
    promo *my_promo = create_promo(1930);

    student *tab_std[16];

    for (int i = 0; i < 16; i++) {
        tab_std[i] = create_student(tab_fn[i], tab_ln[i]);
        add_in_pc(tab_std[i], &(my_promo->pcs_tab[i/2]));
    }

    assert (nb_of_students(my_promo) == 16);

    printf("OK for nb_of_students!\n");

    return 0;
}
