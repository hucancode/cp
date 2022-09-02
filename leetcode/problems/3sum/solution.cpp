class Solution {
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        vector<vector<int>> ret;
        sort(nums.begin(), nums.end());
        for(int i = 0;i<nums.size()-2;i++) {
            if(i > 0 && nums[i] == nums[i-1]) {
                continue;
            }
            for(int j = i+1;j<nums.size()-1;j++) {
                if(j > i+1 && nums[j] == nums[j-1]) {
                    continue;
                }
                int key = -(nums[i] + nums[j]);
                if(binary_search(nums.begin()+j+1, nums.end(), key)) {
                    ret.push_back({nums[i], nums[j], key});
                }
            }
        }
        return ret;
    }
};