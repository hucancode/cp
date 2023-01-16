class Solution {
public:
    long long countGood(vector<int>& nums, int k) {
        // count the number of subarray that has at least k pairs
        int n = nums.size();
        long long ret = 0;
        map<int,int> count;
        long long l0 = 0, l = 0, r = 0;
        long long pairs = 0;
        while(r<n) {
            auto pairsGain = count[nums[r]];
            count[nums[r]]++;
            pairs += pairsGain;
            if(pairs < k) {
                r++;
                continue;
            }
            while(pairs >= k) {
                auto pairsLoss = count[nums[l]] - 1;
                count[nums[l]]--;
                pairs -= pairsLoss;
                l++;
            }
            auto subCount = (l-l0)*(n-r);
            ret += subCount;
            l0 = l;
            r++;
        }
        return ret;
    }
};