class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        int n = nums.size();
        vector<vector<int>> ret;
        if(n == 1) {
            ret.push_back(nums);
            return ret;
        }
        vector<int> b(nums.begin()+1, nums.end());
        auto pb = permute(b);
        auto a = nums[0];
        for(const auto x: pb) {
            int i = 0;
            while(true) {
                vector<int> c(x.begin(), x.end());
                c.insert(c.begin()+i, a);
                ret.push_back(c);
                if(i == x.size()) {
                    break;
                }
                i++;
            }
        }
        return ret;
    }
};