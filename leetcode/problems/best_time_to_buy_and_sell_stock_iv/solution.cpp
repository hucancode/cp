class Solution {
public:
    int maxProfit(int maxk, vector<int>& prices) {
        int n = prices.size();
        if(n < 2) {
            return 0;
        }
        vector<vector<int>> f(maxk+1, vector<int>(n, 0));
        vector<vector<int>> delta(n, vector<int>(n, 0));
        int ret = 0;
        for(int i = 1;i<n;i++) {
            for(int j = 0;j<i;j++) {
                delta[i][j] = prices[i] - prices[j];
            }
        }
        for(int i = 1;i<n;i++) {
            for(int k = 1;k<=maxk;k++) {
                f[k][i] = max(f[k][i], f[k-1][i]);
                f[k][i] = max(f[k][i], f[k][i-1]);
                //cout<<"max profit till now = "<<f[k][i]<<endl;
                for(int j = 0;j<i;j++) {
                    if(delta[i][j] < 0) {
                        continue;
                    }
                    //cout<<"check range a["<<j<<"]="<<prices[j]<<", a["<<i<<"]="<<prices[i]<<", delta = "<<delta<<endl;
                    if(j > 0) {
                        f[k][i] = max(f[k][i], f[k-1][j-1] + delta[i][j]);
                    } else {
                        f[k][i] = max(f[k][i], delta[i][j]);
                    }
                }
            }
        }
        return f[maxk][n-1];
    }
};