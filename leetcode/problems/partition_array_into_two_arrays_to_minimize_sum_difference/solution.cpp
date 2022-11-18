class Solution {
public:
    int minimumDifference(vector<int>& nums) {
        // Brute force can AC until n = 12
        // n = 13, 14 it started to TLE, n=15 it even MLE
        int n = nums.size();
        int m = n/2;
        int all = accumulate(nums.begin(), nums.end(), 0);
        int target = all/2;
        int k = (1<<m) - 1;
        vector<vector<int>> left(m+1), right(m+1);
        vector<int> f(k+1, 0);
        vector<int> g(k+1, 0);
        for(int mask = 0;mask<k;mask++) {
            for(int i = 0;i<m;i++) {
                int nextMask = mask | 1<<i;
                if(nextMask == mask) {
                    continue;
                }
                f[nextMask] = f[mask] + nums[i];
                g[nextMask] = g[mask] + nums[i+m];
            }
        }
        for(int mask = 0;mask<=k;mask++) {
            int j = __builtin_popcount(mask);
            left[j].push_back(f[mask]);
            right[j].push_back(g[mask]);
        }
        int ret = 2e9;
        for(auto& x: right) {
            sort(x.begin(), x.end());
        }
        for(int i = 0;i<=m;i++) {
            int j = m - i;
           // cout<<"take "<<i<<" from left, take "<<j<<" from right"<<endl;
            for(auto x: left[i]) {
                int y = target - x;
                //cout<<"sum left = "<<x<<", ideal sum right = "<<y<<endl;
                auto it = lower_bound(right[j].begin(), right[j].end(), y);
                if(it != right[j].end()) {
                    int delta = abs(2*(x + *it) - all);
                    //cout<<"found "<<*it<<" delta = "<<delta<<endl;
                    ret = min(ret, delta);
                }
                if(it == right[j].begin()) {
                    continue;
                }
                it--;
                int delta = abs(2*(x + *it) - all);
                //cout<<"found "<<(*it)<<" delta = "<<delta<<endl;
                ret = min(ret, delta);
            }
        }
        return ret;
    }
};