class Solution {
public:
    int findMaxK(vector<int>& nums) {
        sort(nums.begin(), nums.end());
        auto pivot = upper_bound(nums.begin(), nums.end(), 0);
        auto it = pivot;
        int ret = -1;
        while(it != nums.end()) {
            auto k = *it;
            if(binary_search(nums.begin(), it, -k)) {
                ret = max(ret, k);
            }
            it++;
        }
        return ret;
    }
};