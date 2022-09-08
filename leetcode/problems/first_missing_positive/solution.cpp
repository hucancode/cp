class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        auto it = lower_bound(nums.begin(), nums.end(), 1);
        int ret = 1;
        while(it != nums.end() && *it == ret) {
            while(it != nums.end() && *it == ret) {
                it++;
            }
            ret++;
        }
        return ret;
    }
};