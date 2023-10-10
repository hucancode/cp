class Solution {
public:
    int minOperations(vector<int>& nums) {
        int n = nums.size();
        sort(nums.begin(), nums.end());
        nums.erase(unique(nums.begin(), nums.end()), nums.end());
        int ret = n-1;
        for(auto l = nums.begin(); l != nums.end();l++) {
            int target = *l+n-1;
            auto r = lower_bound(l, nums.end(), target);
            if(r != nums.end() && *r == target) r++;
            int cost = n-distance(l,r);
            ret = min(ret, cost);
        }
        return ret;
    }
};