class Solution {
public:
    int rob(vector<int>& nums) {
        int n = nums.size();
        vector<int> f(n+2, 0);
        for(int i = 2;i<n+2;i++) {
            f[i] = max(f[i-1], f[i-2] + nums[i-2]);
        }
        return *f.rbegin();
    }
};