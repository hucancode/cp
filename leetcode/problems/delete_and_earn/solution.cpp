class Solution {
public:
    int deleteAndEarn(vector<int>& nums) {
        int n = nums.size();
        int k = 1;
        map<int, int> counts;
        for(const auto& x: nums) {
            counts[x]++;
            k = max(k, x);
        }
        int prev = 0;
        int current = counts[1];
        int next = current;
        for(int i = 2;i<=k;i++) {
            next = max(counts[i]*i + prev, current);
            prev = current;
            current = next;
        }
        return next;
    }
};