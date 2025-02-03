#include <stdio.h>

extern int leap_year(int year);

void test(int n) {
    printf("The year %d is leap year? %d\n",n, leap_year(n));
}
int main() {
    test(2015);
    test(1970);
    test(1996);
    test(1960);
    test(2100);
    test(1900);
    test(2000);
    test(2400);
    test(1800);
    return 0;
}
