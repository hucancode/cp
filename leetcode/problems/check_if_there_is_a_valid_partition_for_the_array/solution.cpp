class Solution {
public:
    bool validPartition(vector<int>& nums) {
        int n = nums.size();
        vector<bool> f(n+1, false);
        f[0] = true;
        for(int i = 2;i<=n;i++) {
            if(nums[i-1] == nums[i-2]) {
                f[i] = f[i] || f[i-2];
            }
            if(i < 3) {
                continue;
            }
            if(nums[i-1] == nums[i-2] && nums[i-1] == nums[i-3]) {
                f[i] = f[i] || f[i-3];
            }
            if(nums[i-1] == nums[i-2]+1 && nums[i-1] == nums[i-3] + 2) {
                f[i] = f[i] || f[i-3];
            }
        }
        return f[n];
    }
};