class Solution {
public:
    int maximum69Number (int num) {
        int ret = 0;
        int k = 1e4;
        bool swapped = false;
        while(num > 0) {
            int digit = num/k;
            if(digit == 6 && !swapped) {
                digit = 9;
                swapped = true;
            }
            ret += digit*k;
            num = num%k;
            k/=10;
        }
        return ret;
    }
};