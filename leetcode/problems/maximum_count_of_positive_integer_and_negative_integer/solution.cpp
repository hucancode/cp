class Solution {
public:
    int maximumCount(vector<int>& nums) {
        auto i = distance(nums.begin(), lower_bound(nums.begin(), nums.end(), 0));
        auto j = distance(upper_bound(nums.begin(), nums.end(), 0), nums.end());
        return max(i,j);
    }
};