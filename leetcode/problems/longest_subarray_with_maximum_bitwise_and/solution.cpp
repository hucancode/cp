class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        int n = nums.size();
        int ret = 1;
        int count = 1;
        int maxv = nums[0];
        for(int i = 1;i<n;i++) {
            if(nums[i] > maxv) {
                count = 1;
                ret = 1;
                maxv = nums[i];
            } else if(nums[i] == maxv) {
                count++;
            } else {
                count = 0;
            }
            ret = max(ret, count);
        }
        return ret;
    }
};