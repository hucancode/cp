class Solution {
public:
    int maximumJumps(vector<int>& nums, int target) {
        int n = nums.size();
        vector<int> f(n, -1);
        f[0] = 0;
        for(int i = 0;i<n;i++) {
            if(f[i] == -1) continue;
            for(int j = i+1;j<n;j++) {
                if(abs(nums[i] - nums[j]) <= target) {
                    f[j] = max(f[j], f[i]+1);
                }
            }
        }
        return f[n-1];
    }
};