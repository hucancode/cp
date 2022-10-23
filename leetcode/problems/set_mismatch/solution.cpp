class Solution {
public:
    vector<int> findErrorNums(vector<int>& nums) {
        vector<int> ret;
        vector<int> count(1e4+1, 0);
        for(auto& x: nums) {
            count[x]++;
        }
        for(int i = 1;i<=nums.size();i++) {
            if(count[i] > 1) {
                ret.push_back(i);
            }
        }
        for(int i = 1;i<=nums.size();i++) {
            if(count[i] == 0) {
                ret.push_back(i);
            }
        }
        return ret;
    }
};