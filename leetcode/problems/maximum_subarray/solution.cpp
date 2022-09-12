class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int n = nums.size();
        vector<int> f(n);
        f[0] = nums[0];
        int ret = f[0];
        for(int i = 1;i<n;i++) {
            f[i] = max(f[i-1], 0) + nums[i];
            ret = max(ret, f[i]);
        }
        return ret;
    }
};