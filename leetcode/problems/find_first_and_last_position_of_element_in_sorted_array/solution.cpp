class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int n = nums.size();
        auto l = lower_bound(nums.begin(), nums.end(), target);
        auto r = upper_bound(l, nums.end(), target);
        if(l == nums.end() || *l != target) {
            return {-1, -1};
        }
        return {
            (int)distance(nums.begin(), l), 
            (int)distance(nums.begin(), r) - 1,
        };
    }
};