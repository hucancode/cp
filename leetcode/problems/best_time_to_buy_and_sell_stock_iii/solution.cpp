class Solution {
public:
    int maxProfit(vector<int>& prices) {
        const int maxk = 2;
        int ret = 0;
        int n = prices.size();
        if(n < 2) {
            return 0;
        }
        vector<vector<int>> f(maxk+1, vector<int>(n, 0));
        vector<vector<int>> g(maxk+1, vector<int>(n, 0));
        // f[i][j] = max profit if buy at j, have bought i times
        // g[i][j] = max profit if sell at j, have sold i times
        for(int i = 1;i<=maxk;i++) {
            for(int j = 0;j<i;j++) {
                g[i][j] = -2e9;
                f[i][j] = (j>0?f[i][j-1]:0) - prices[j];
            }
        }
        for(int i = 1;i<=maxk;i++) {
            for(int j = 1;j<n;j++) {
                int buy = g[i-1][j-1]-prices[j];
                int skip = f[i][j-1];
                f[i][j] = max(buy, skip);
            }
            for(int j = 1;j<n;j++) {
                int sell = f[i][j-1]+prices[j];
                int skip = g[i][j-1];
                g[i][j] = max(sell, skip);
                ret = max(ret, g[i][j]);
            }
            continue;
            cout<<"round "<<i<<endl;
            cout<<", f = ";
            for(auto x: f[i]) {
                cout<<x<<" ";
            }
            cout<<endl;
            cout<<", g = ";
            for(auto x: g[i]) {
                cout<<x<<" ";
            }
            cout<<endl;
        }
        return ret;
    }
};