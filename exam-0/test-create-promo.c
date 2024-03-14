#include <assert.h>
#include <stdio.h>
#include "grading.h"

int main(void) {
    promo *p_promo = create_promo(2018);

    assert (p_promo != NULL);
    assert (p_promo->year == 2018);

    printf("OK for create_promo!\n");

    return 0;
}
