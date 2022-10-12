class Solution {
public:
    int change(int amount, vector<int>& coins) {
        int n = coins.size();
        vector<vector<int>> f(amount+1, vector<int>(n+1, 0));
        for(int j = 0;j<=n;j++) {
            f[0][j] = 1;
        }
        for(int i = 1;i<=amount;i++) {
            for(int j = 1;j<=n;j++) {
                f[i][j] = f[i][j-1];
                int k = 1;
                int v = coins[j-1];
                while(k*v <= i) {
                    int remaining = i - k*v;
                    f[i][j] += f[remaining][j-1];
                    k++;
                }
            }
        }
        // cout<<"f = "<<endl;
        // for(int i = 0;i<=amount;i++) {
        //     for(int j = 0;j<=n;j++) {
        //         cout<<" "<<f[i][j];
        //     }
        //     cout<<endl;
        // }
        return f[amount][n];
    }
};