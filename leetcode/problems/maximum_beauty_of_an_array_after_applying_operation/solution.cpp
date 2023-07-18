class Solution {
public:
    int maximumBeauty(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        long ret = 1;
        for(auto x: nums) {
            ret = max(ret, distance(lower_bound(nums.begin(), nums.end(), x - k), 
                                   upper_bound(nums.begin(), nums.end(), x + k)));
            ret = max(ret, distance(lower_bound(nums.begin(), nums.end(), x - k*2), 
                                   upper_bound(nums.begin(), nums.end(), x)));
            ret = max(ret, distance(lower_bound(nums.begin(), nums.end(), x), 
                                   upper_bound(nums.begin(), nums.end(), x+k*2)));
        }
        return ret;
    }
};