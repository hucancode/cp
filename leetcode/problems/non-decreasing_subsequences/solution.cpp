class Solution {
public:
    vector<vector<int>> findSubsequences(vector<int>& nums) {
        map<int, set<vector<int>>> mp;
        int n = nums.size();
        for(int i = 0;i<n;i++) {
            auto prev = mp[nums[i]];
            for(int j = -100;j<nums[i];j++) {
                for(auto arr: mp[j]) {
                    arr.push_back(nums[i]);
                    mp[nums[i]].insert(arr);
                }
            }
            vector<int> arr = {nums[i]};
            mp[nums[i]].emplace(arr);
            for(auto arr: prev) {
                arr.push_back(nums[i]);
                mp[nums[i]].emplace(arr);
            }
            
        }
        vector<vector<int>> ret;
        for(auto& item: mp) {
            for(auto& arr: item.second) {
                if(arr.size() < 2) continue;
                ret.push_back(arr);
            }
        }
        return ret;
    }
};