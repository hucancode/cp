class Solution {
public:
    vector<int> smallestRange(vector<vector<int>>& nums) {
        int n = nums.size();
        auto left = make_pair(0,0);
        auto right = make_pair(0,1);
        auto minLen = 1000000;
        auto cmp = [&](const pair<int, int>& a, const pair<int, int>& b) {
            return nums[a.first][a.second] > nums[b.first][b.second];
        };
        vector<pair<int, int>> range;
        for(int i = 0;i<n;i++) {
            auto value = make_pair(i, 0);
            range.insert(
                lower_bound(range.begin(), range.end(), value, cmp), 
                value);
        }
        while(true) {
            // check current candidate
            auto l = range[n-1];
            auto r = range[0];
            auto len = nums[r.first][r.second] - nums[l.first][l.second];
            if(len < minLen) {
                left = l;
                right = r;
                minLen = len;
            }
            // advance to next candidate
            auto rowToAdvance = l.first;
            auto next = l.second + 1;
            range.pop_back();
            if(next >= nums[rowToAdvance].size()) {
                break;
            }
            auto item = make_pair(rowToAdvance, next);
            range.insert(
                lower_bound(range.begin(), range.end(), item, cmp), 
                item);
        }
        vector<int> ret = {nums[left.first][left.second], nums[right.first][right.second]};
        return ret;
    }
};