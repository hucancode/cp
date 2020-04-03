class Solution {
public:
    int maxSubArray(vector<int>& nums)
    {
        int dp;
        int ret;
        auto it = nums.begin();
        ret = dp = *it++;
        for(;it!=nums.end();it++)
        {
            dp = *it + max(0, dp);
            ret = max(ret, dp);
        }
        return ret;
    }
};