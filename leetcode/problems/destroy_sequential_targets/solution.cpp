class Solution {
public:
    int destroyTargets(vector<int>& nums, int space) {
        int n = nums.size();
        map<int, int> remaining;
        map<int, int> start;
        for(auto k: nums) {
            int x = k%space;
            remaining[x]++;
            if(start.find(x) == start.end()) {
                start[x] = k;
            } else {
                start[x] = min(start[x], k);
            }
        }
        int ret = nums[0];
        int maxScore = 1;
        for(auto& item: remaining) {
            if(item.second > maxScore || (item.second == maxScore && start[item.first] < ret)) {
                maxScore = item.second;
                ret = start[item.first];
            }
        }
        return ret;
    }
};