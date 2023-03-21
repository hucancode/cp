class Solution {
public:
    long long zeroFilledSubarray(vector<int>& nums) {
        long long ret = 0;
        int score = 0;
        for(auto x: nums) {
            if(x != 0) {
                score = 0;
                continue;
            }
            ret += ++score;
        }
        return ret;
    }
};