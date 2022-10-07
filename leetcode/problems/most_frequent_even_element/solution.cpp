class Solution {
public:
    int mostFrequentEven(vector<int>& nums) {
        int n = 0;
        vector<int> f(100001, 0);
        for(const auto &x: nums) {
            f[x]++;
            n = max(n, x);
        }
        int ret = -1;
        for(int i = 0;i<=n;i+=2) {
            if(!f[i]) {
                continue;
            }
            if(ret == -1 || f[i] > f[ret]) {
                ret = i;
            }
        }
        return ret;
    }
};