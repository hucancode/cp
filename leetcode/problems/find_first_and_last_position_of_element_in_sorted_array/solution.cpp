class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target)
    {
        vector<int> ret(2, -1);
        auto left = lower_bound(nums.begin(), nums.end(), target);
        if(left == nums.end() || *left != target)
        {
            return ret;
        }
        auto right = upper_bound(left, nums.end(), target);
        ret[0] = left - nums.begin();
        ret[1] = right - nums.begin() - 1;
        return ret;
    }
};