class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int ret, score = 0;
        for(auto x: nums) {
            if(score == 0) {
                ret = x;
            }
            if(ret == x) {
                score++;
            } else {
                score--;
            }
        }
        return ret;
    }
};