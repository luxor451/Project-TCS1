#include <assert.h>
#include <stdio.h>
#include <string.h>
#include "grading.h"

int main(void) {
    student *p_std = create_student("Christophe", "Garion");

    grade(p_std, 20.0);

    assert (p_std != NULL);
    assert (strcmp(p_std->first_name, "Christophe") == 0);
    assert (strcmp(p_std->last_name, "Garion") == 0);
    assert (p_std->grade == 20.0);

    printf("OK for grade!\n");

    return 0;
}
