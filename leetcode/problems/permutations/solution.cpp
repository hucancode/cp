class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> ret;
        do {
            vector<int> p(nums.begin(), nums.end());
            ret.push_back(p);
        } while(next_permutation(nums.begin(), nums.end()));
        return ret;
    }
};