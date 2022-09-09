class Solution {
public:
    bool isHappy(int n) {
        const int MAX_N = 9*9*9+2*2;
        vector<bool> seen(MAX_N, false);
        while(true) {
            int sum = 0;
            int factor = 1e9;
            while(factor != 0) {
                int digit = n/factor;
                sum += digit*digit;
                n = n%factor;
                factor /= 10;
            }
            n = sum;
            if(seen[n]) {
                break;
            }
            seen[n] = true;
        }
        return n == 1;
    }
};