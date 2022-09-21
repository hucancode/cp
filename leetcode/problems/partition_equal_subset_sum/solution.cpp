class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int target = accumulate(nums.begin(), nums.end(), 0);
        if(target%2 == 1) {
            return false;
        }
        target /= 2;
        int n = nums.size();
        vector<vector<bool>> f(n+1, vector<bool>(target+1, false));
        for(int i = 0;i<=n;i++) {
            f[i][0] = true;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=target;j++) {
                f[i][j] = f[i-1][j];
                auto subtarget = j - nums[i-1];
                if(nums[i-1] <= j) {
                    f[i][j] = f[i][j] || f[i-1][subtarget];
                }
            }
        }
        return f[n][target];
    }
};