class Solution {
public:
    int reverse(int x) {
        if(x == 0) {
            return 0;
        }
        int ret = 0;
        int k = 1000000000;
        while(x/k == 0) {
            k /= 10;
        }
        while(k != 0) {
            int digit = x%10;
            if(digit > (INT_MAX/k)) {
                return 0;
            }
            if(digit < (INT_MIN/k)) {
                return 0;
            }
            int delta = digit*k;
            if(ret > 0 && delta > INT_MAX - ret) {
                return 0;
            }
            if(ret < 0 && delta < INT_MIN - ret) {
                return 0;
            }
            ret += delta;
            x /= 10;
            k /= 10;
        }
        return ret;
    }
};