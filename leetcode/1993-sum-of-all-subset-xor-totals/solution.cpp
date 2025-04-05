class Solution {
public:
    int subsetXORSum(vector<int>& nums) {
        int ret = accumulate(nums.begin(), nums.end(), 0, [](int acc, int x) { return acc | x; });
        return ret << (nums.size() - 1);
    }
};
