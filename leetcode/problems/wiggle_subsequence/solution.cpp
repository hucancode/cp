class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        int n = nums.size();
        vector<int> up(n, 1);
        vector<int> down(n, 1);
        for(int i = 1;i<n;i++) {
            for(int j = 0;j<i;j++) {
                if(nums[j] < nums[i]) {
                    up[i] = max(up[i], down[j] + 1);
                }
                if(nums[j] > nums[i]) {
                    down[i] = max(down[i], up[j] + 1);
                }
            }
        }
        return max(*up.rbegin(), *down.rbegin());
    }
};