class Solution {
public:
    int triangularSum(vector<int>& nums) {
        int n = nums.size();
        int ret = 0;
        while(n > 1) {
            for(int i = 0;i<n-1;i++) {
                nums[i] += nums[i+1];
                nums[i] %= 10;
            }
            n--;
        }
        return nums[0];
    }
};