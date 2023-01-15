class Solution {
public:
    int differenceOfSum(vector<int>& nums) {
        auto sum = accumulate(nums.begin(), nums.end(), 0);
        int digit = 0;
        for(auto x: nums) {
            int k = 1000;
            while(x > 0) {
                int d = x/k;
                x %= k;
                k /= 10;
                digit += d;
            }
        }
        return abs(sum - digit);
    }
};