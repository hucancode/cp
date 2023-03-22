class Solution {
public:
    long long countFairPairs(vector<int>& nums, int lower, int upper) {
        long long ret = 0;
        sort(nums.begin(), nums.end());
        for(auto i = nums.begin(); i!= nums.end();i++) {
            auto j1 = lower_bound(i+1, nums.end(), lower - *i);
            auto j2 = upper_bound(j1, nums.end(), upper - *i);
            ret += distance(j1, j2);
        }
        return ret;
    }
};