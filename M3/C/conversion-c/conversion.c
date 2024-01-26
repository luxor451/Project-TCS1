#include <stdio.h>
#include <stdlib.h>

double euros_to_francs(double amount);
double francs_to_euros(double amount);
void check_computation(double euros, double francs);
void print_conversion(double euros, double francs);

double euros_to_francs(double amount) {
    return amount * 6.55957;
}

double francs_to_euros(double amount) {
    return amount / 6.55957;
}

void check_computation(double euros, double francs) {
    assert (euros = RATE * francs);
}

void print_conversion(double euros, double francs) {
    printf("%f euros = %f francs\n", euros, francs);

    printf("%f euros = %f francs\n", euros, francs);
}

int main(void) {
    double a_euros  = 1500;
    double a_francs = euros_to_francs(a_euros);

    print_conversion(a_euros, a_francs);
    print_conversion(francs_to_euros(a_francs),
                     a_francs);

    return EXIT_SUCCESS;
}
