#include <stdio.h>
#include <stdlib.h>

int main(void) {
    double a_euros  = 500;
    double a_francs = euros_to_francs(a_euros);

    print_conversion(a_euros, a_francs);
    print_conversion(francs_to_euros(a_francs),
                     a_francs);

    return EXIT_SUCCESS;
}
