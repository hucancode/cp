class Solution {
public:
    int longestConsecutive(vector<int>& nums) {
        if(nums.empty()) {
            return 0;
        }
        map<int, int> streaks;
        for(const auto& x: nums) {
            streaks[x] = 1;
        }
        int ret = 1;
        for(const auto& item: streaks) {
            auto next = item.first + 1;
            ret = max(ret, item.second);
            if(streaks.find(next) != streaks.end()) {
                streaks[next] = item.second + 1;
            }
        }
        return ret;
    }
};