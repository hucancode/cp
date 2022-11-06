class Solution {
public:
    vector<int> applyOperations(vector<int>& nums) {
        int n = nums.size();
        vector<int> ret(n,0);
        int j = 0;
        for(int i = 0;i<n;i++) {
            if(i < n - 1 && nums[i] == nums[i+1] && nums[i] != 0) {
                nums[i] *= 2;
                ret[j++] = nums[i++];
            } else if(nums[i] != 0) {
                ret[j++] = nums[i];
            }
        }
        return ret;
    }
};