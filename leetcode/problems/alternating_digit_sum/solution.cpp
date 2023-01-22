class Solution {
public:
    int alternateDigitSum(int n) {
        int k = 1e9;
        int sign = 1;
        int ret = 0;
        while(n/k == 0) {
            k /= 10;
        }
        while(k != 0) {
            int d = n/k;
            n %= k;
            ret += d*sign;
            sign = -sign;
            k /= 10;
        }
        return ret;
    }
};