#include <stdint.h>
#include <stdio.h>

extern uint64_t square(int64_t number);
extern uint64_t total(void);

void test_sqr(int64_t n) {
    printf("Grain count at %lld? %llu\n", n, square(n));
}
void test_total() {
    printf("Total grain count? %llu\n", total());
}
int main() {
    test_sqr(1);
    test_sqr(2);
    test_sqr(3);
    test_sqr(4);
    test_sqr(33);
    test_sqr(65);
    test_sqr(0);
    test_sqr(66);
    test_sqr(-10);
    test_total();
    return 0;
}
