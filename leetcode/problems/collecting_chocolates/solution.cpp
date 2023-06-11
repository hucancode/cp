class Solution {
public:
    long long minCost(vector<int>& nums, int x) {
        int n = nums.size();
        long long ret = accumulate(nums.begin(), nums.end(), 0LL);
        vector<int> cost(nums);
        for(int j = 1;j<n;j++) {
            long long score = (long long)j*(long long)x;
            for(int i = 0;i<n;i++) {
                int ij = (i-j+n)%n;
                cost[i] = min(cost[i], nums[ij]);
                score += cost[i];
            }
            ret = min(ret, score);
        }
        return ret;
    }
};