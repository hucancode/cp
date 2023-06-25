class Solution {
public:
    int numberOfGoodSubarraySplits(vector<int>& nums) {
        const int INF = 1e9+7;
        int n = nums.size();
        vector<long long> f(n, 0);
        int last = -1;
        int ret = 0;
        for(int i = 0;i<n;i++) {
            if(nums[i] == 0) {
                continue;
            }
            if(last == -1) {
                f[i] = 1;
                last = i;
            } else {
                int d = i - last;
                f[i] = f[last]*d;
                f[i] %= INF;
                last = i;
            }
            ret = f[i];
            
        }
        return ret;
    }
};