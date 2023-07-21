class Solution {
public:
    int alternatingSubarray(vector<int>& nums) {
        int n = nums.size();
        int ret = -1;
        for(int i = 0;i<n;i++) {
            int j = i;
            while(j<n) {
                int x = (j-i)%2;
                if(x == 0 && nums[j] == nums[i]
                    || x == 1 && nums[j] == nums[i]+1) {
                    j++;
                } else {
                    break;
                }
            }
            if(j-i > 1) {
                ret = max(ret, j-i);
            }
        }
        return ret;
    }
};