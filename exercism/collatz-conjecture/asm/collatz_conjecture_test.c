#include <stdio.h>

extern int steps(int n);

void test(int n) {
    printf("Steps to reach from %d to 1: %d\n",n, steps(n));
}
int main() {
    test(1);
    test(16);
    test(12);
    test(1000000);
    test(0);
    test(-15);
    return 0;
}
