class Solution {
public:
    int maxProduct(vector<int>& nums) {
        int n = nums.size();
        vector<int> f(n);
        vector<int> g(n);
        f[0] = nums[0];
        g[0] = nums[0];
        int ret = f[0];
        for(int i = 1;i<n;i++) {
            int k1 = nums[i] * f[i-1];
            int k2 = nums[i] * g[i-1];
            f[i] = max(nums[i], max(k1, k2));
            g[i] = min(nums[i], min(k1, k2));
            ret = max(ret, f[i]);
        }
        return ret;
    }
};