class Solution {
public:
    long long countSubarrays(vector<int>& nums, long long k) {
        long long ret = 0;
        int n = nums.size();
        vector<long long> sums(n+1, 0);
        for(int i = 1;i<=n;i++) {
            sums[i] = sums[i-1]+nums[i-1];
        }
        int j = 0;
        int i = 1;
        while(i<=n) {
            auto sum = sums[i] - sums[j];
            auto d = i-j;
            auto score = sum*d;
            if(score < k) {
                ret += d;
                i++;
            } else {
                j++;
            }
        }
        return ret;
    }
};