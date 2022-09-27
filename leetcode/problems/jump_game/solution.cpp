class Solution {
public:
    bool canJump(vector<int>& nums) {
        int n = nums.size();
        vector<bool> f(n, false);
        f[0] = true;
        for(int i = 0;i<n;i++) {
            if(!f[i]) {
                continue;
            }
            int left = i+1;
            int right = min(n, left + nums[i]);
            if(right == n) {
                return true;
            }
            fill(f.begin()+left, f.begin()+right, true);
        }
        return f[n-1];
    }
};