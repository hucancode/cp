class Solution {
public:
    int combinationSum4(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        vector<unsigned int> f(target+1, 0);
        f[0] = 1;
        for(int i = 1;i<= target;i++) {
            for(const auto& x: nums) {
                int j = i-x;
                if(j < 0) {
                    break;
                }
                f[i] += f[j];
            }
        }
        return f[target];
    }
};