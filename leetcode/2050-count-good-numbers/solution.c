const long long k = 1000000007;
long long fastpow(long long x, long long n) {
    if(n==0) return 1;
    long long p = fastpow(x, n/2);
    x = x*(n%2) + (n+1)%2;
    return p*p*x%k;
}
int countGoodNumbers(long long n) {
    return fastpow(4, n/2)*fastpow(5, n-n/2)%k;
}
