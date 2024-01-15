#include <stdio.h>

double compute_pi(int n){
    double res = 0;
    for (int i = 0; i < n; i++)
    {
        if (i % 2 == 0)
        {
            res = res + 1.0/(2*i + 1);
        } else
        {
            res = res - 1.0/(2*i + 1);
        }
    }
    
    return 4 * res;
}


int main(){
    int n = 10000000000;
    double pi = compute_pi(n);
    printf("%lf", pi);
}


