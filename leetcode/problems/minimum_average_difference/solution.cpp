class Solution {
public:
    int minimumAverageDifference(vector<int>& nums) {
        int n = nums.size();
        vector<long long> prefix(n);
        long long all = 0;
        long long dmin = 1e18;
        int imin = -1;
        prefix[0] = nums[0];
        all = nums[0];
        for(int i = 1;i<n;i++) {
            prefix[i] = prefix[i-1]+nums[i];
            all += nums[i];
        }
        for(int i = 0;i<n;i++) {
            auto left = prefix[i];
            auto right = all - prefix[i];
            left /= i+1;
            if(i == n-1) {
                right = 0;
            } else {
                right /= n-i-1;   
            }
            auto delta = abs(left - right);
            if(delta < dmin) {
                imin = i;
                dmin = delta;
            }
        }
        return imin;
    }
};