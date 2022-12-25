class Solution {
public:
    vector<int> answerQueries(vector<int>& nums, vector<int>& q) {
        const int INF = 1e9;
        int n = nums.size();
        vector<int> f(n+1, INF);
        f[0] = 0;
        int r = 1;
        for(int i = 1;i<=n;i++) {
            int x = nums[i-1];
            for(int j=i;j>=1;j--) {
                f[j] = min(f[j-1]+x, f[j]);
            }
        }
        int m = q.size();
        vector<int> ret(m, 0);
        for(int i = 0;i<m;i++) {
            for(int j = 1;j<=n;j++) {
                if(f[j] <= q[i]) {
                    ret[i] = max(ret[i], j);
                }
            }
        }
        return ret;
    }
};