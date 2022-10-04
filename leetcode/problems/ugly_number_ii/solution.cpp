class Solution {
public:
    int nthUglyNumber(int n) {
        vector<int> f;
        f.push_back(1);
        int i2 = 0;
        int i3 = 0;
        int i5 = 0;
        int f2 = f[i2]*2;
        int f3 = f[i3]*3;
        int f5 = f[i5]*5;
        while(f.size() < n) {
            if(f2 <= f3 && f2 <= f5) {
                if(!binary_search(f.begin(), f.end(), f2)) {
                    f.push_back(f2);
                }
                i2++;
                f2 = f[i2]*2;
            }
            if(f3 <= f2 && f3 <= f5) {
                if(!binary_search(f.begin(), f.end(), f3)) {
                    f.push_back(f3);
                }
                i3++;
                f3 = f[i3]*3;
            }
            if(f5 <= f2 && f5 <= f3) {
                if(!binary_search(f.begin(), f.end(), f5)) {
                    f.push_back(f5);
                }
                i5++;
                f5 = f[i5]*5;
            }
        }
        return f[n-1];
    }
};