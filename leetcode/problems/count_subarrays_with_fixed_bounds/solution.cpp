class Solution {
public:
    long long countSubarrays(vector<int>& nums, int minK, int maxK) {
        int n = nums.size();
        long long ret = 0;
        int lastMinK = -1;
        int lastMaxK = -1;
        int firstValid = 0;
        for(int i = 0;i<n;i++) {
            if(nums[i] > maxK || nums[i] < minK) {
                firstValid = i+1;
                lastMinK = -1;
                lastMaxK = -1;
            }
            if(nums[i] == minK) {
                lastMinK = i;
            }
            if(nums[i] == maxK) {
                lastMaxK = i;
            }
            ret += max(0, min(lastMinK, lastMaxK) - firstValid + 1);
        }
        return ret;
    }
};