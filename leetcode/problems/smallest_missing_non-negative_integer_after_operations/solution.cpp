class Solution {
public:
    int findSmallestInteger(vector<int>& nums, int value) {
        map<int, vector<int>> group;
        for(auto x: nums) {
            if(x < 0) {
                x = value - (-x % value);
            }
            auto g = x%value;
            group[g].push_back(x);
        }
        int ret = 0;
        while(true) {
            int g = ret % value;
            if(group[g].empty()) {
                break;
            }
            group[g].pop_back();
            ret++;
        }
        return ret;
    }
};