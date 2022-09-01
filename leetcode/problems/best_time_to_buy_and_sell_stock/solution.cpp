class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        vector<int> f(n);
        vector<int> g(n);
        f[0] = prices[0];
        for(int i = 1;i<n;i++) {
            f[i] = min(f[i-1], prices[i]);
        }
        g[n-1] = prices[n-1];
        int ret = g[n-1] - f[n-1];
        for(int i = n-2;i>=0;i--) {
            g[i] = max(g[i+1], prices[i]);
            ret = max(ret, g[i] - f[i]);
        }
        return ret;
    }
};