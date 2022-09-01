class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        map<int, int> m;
        for(int i = 0;i<nums.size();i++) {
            auto it = m.find(target - nums[i]);
            if(it != m.end()) {
                return {i, it->second};
            }
            m[nums[i]] = i;
        }
        return {0, 0};
    }
};