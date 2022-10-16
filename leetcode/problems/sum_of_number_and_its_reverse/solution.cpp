class Solution {
public:
    int invert(int n) {
        int remainder;
        int ret = 0;
        while (n != 0) {
            remainder = n % 10;
            ret = ret * 10 + remainder;
            n /= 10;
        }
        return ret;
    }
    bool sumOfNumberAndReverse(int num) {
        vector<int> f(num+1, -1);
        for(int i = 0;i<=num;i++) {
            if(f[i] != -1) {
                continue;
            }
            int j = invert(i);
            if(j > num) {
                continue;
            }
            f[i] = f[j] = i+j;
        }
        for(int i = 0;i<=num;i++) {
            if(f[i] == num) {
                return true;
            }
        }
        return false;
    }
};