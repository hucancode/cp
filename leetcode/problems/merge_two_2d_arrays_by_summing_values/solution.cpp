class Solution {
public:
    vector<vector<int>> mergeArrays(vector<vector<int>>& nums1, vector<vector<int>>& nums2) {
        vector<int> mp(1001, 0);
        for(auto& p: nums1) {
            mp[p[0]]+= p[1];
        }
        for(auto& p: nums2) {
            mp[p[0]]+= p[1];
        }
        vector<vector<int>> ret;
        for(int i = 1;i<= 1000;i++) {
            if(mp[i] != 0) {
                ret.push_back({i, mp[i]});
            }
        }
        return ret;
    }
};