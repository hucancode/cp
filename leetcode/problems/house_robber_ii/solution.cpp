class Solution {
public:
    int rob(vector<int>& nums) {
        int n = nums.size();
        if(n < 3) {
            return *max_element(nums.begin(), nums.end());
        }
        vector<int> f(n+2, 0);
        for(int i = 2;i<n+1;i++) {
            f[i] = max(f[i-1], f[i-2] + nums[i-2]);
        }
        int robFirstHouse = f[n];
        fill(f.begin(), f.end(), 0);
        for(int i = 3;i<n+2;i++) {
            f[i] = max(f[i-1], f[i-2] + nums[i-2]);
        }
        int robLastHouse = f[n+1];
        return max(robFirstHouse, robLastHouse);
    }
};