class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int n = nums.size();
        int target = n*(n+1)/2;
        return target - accumulate(nums.begin(), nums.end(), 0);
    }
};