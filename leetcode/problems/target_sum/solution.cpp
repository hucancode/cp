class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        map<int, int> sums;
        sums[0] = 1;
        for(const auto& x: nums) {
            map<int, int> next;
            for(const auto& p: sums) {
                int k, count;
                tie(k, count) = p;
                next[k+x] += count;
                next[k-x] += count;
            }
            swap(sums, next);
        }
        return sums[target];
    }
};