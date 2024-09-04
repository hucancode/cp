int f(int n, int m) {
    int max = n/m*m;
    int min = m;
    int num = (max - min)/m + 1;
    if(n < m) {
        return 0;
    }
    int ret = (min+max)*num/2;
    //printf("m = %d, min = %d, max = %d, num = %d, ret = %d\n", m, min, max, num, ret);
    return ret;
}
int sumOfMultiples(int n){
    return f(n, 3) + f(n, 5) + f(n, 7) - 
        f(n, 3*5) - f(n, 5*7) - f(n, 7*3) + f(n, 3*5*7);
}
