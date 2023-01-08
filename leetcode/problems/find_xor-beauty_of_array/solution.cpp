class Solution {
public:
    int xorBeauty(vector<int>& nums) {
        int ret = 0;
        for(auto x: nums) {
            ret = ret^x;
        }
        return ret;
    }
};