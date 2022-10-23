class Solution {
public:
    int subarrayGCD(vector<int>& nums, int k) {
        int n = nums.size();
        vector<vector<int>> f(n+1, vector<int>(n+1, 1));
        int ret = 0;
        for(int i = 0;i<n;i++) {
            f[i][i] = nums[i];
            for(int j = i+1;j<n;j++) {
                f[i][j] = __gcd(f[i][j-1], nums[j]);
            }
        }
        for(int i = 0;i<n;i++) {
            for(int j = i;j<n;j++) {
                if(f[i][j] == k) {
                    ret++;
                    //cout<<i<<'-'<<j<<' ';
                }
            }
        }
        return ret;
    }
};