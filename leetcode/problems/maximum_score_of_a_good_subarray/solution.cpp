class Solution {
public:
    int maximumScore(vector<int>& nums, int k) {
        int n = nums.size();
        int u = k;
        int v = k;
        int x = nums[k];
        int ret = 0;
        while(u >= 0 && v < n) {
            x = min(x, min(nums[u], nums[v]));
            auto score = x*(v-u+1);
            ret = max(ret, score);
            if(u == 0) {
                v++;
            } else if(v == n-1) {
                u--;
            } else if(nums[v+1] > nums[u-1]){
                v++;
            } else {
                u--;
            }
        }
        return ret;
    }
};