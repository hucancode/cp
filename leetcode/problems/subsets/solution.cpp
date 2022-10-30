class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        int n = nums.size();
        int k = 1<<n;
        vector<vector<int>> ret;
        for(int i = 0;i<k;i++) {
            vector<int> sub;
            for(int j = 0;j<n;j++) {
                if(i & (1<<j)) {
                    sub.push_back(nums[j]);
                }
            }
            ret.push_back(sub);
        }
        return ret;
    }
};