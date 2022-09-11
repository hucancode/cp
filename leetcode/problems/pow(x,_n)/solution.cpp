class Solution {
public:
    double myPow(double x, int n) {
        const int MIN_N = -2147483647;
        if(n <= MIN_N) {
            return 1.0/myPow(x, n+1)/x;
        }
        if(n < 0) {
            return 1.0/myPow(x, -n);
        }
        if(n == 0) {
            return 1;
        }
        if(n == 1) {
            return x;
        }
        if(n%2 == 1)
        {
            return myPow(x, n-1)*x;
        }
        auto k = myPow(x, n/2);
        return k*k;
    }
};