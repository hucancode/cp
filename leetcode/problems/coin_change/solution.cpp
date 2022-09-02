class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        const int INF = 9999999;
        int f[10001][12];
        for(int j = 0;j<coins.size();j++) {
            f[0][j] = 0;
        }
        for(int i = 1;i<=amount;i++) {
            for(int j = 0;j<coins.size();j++) {
                int k = i - coins[j];
                if(k >= 0) {
                    f[i][j] = *min_element(f[k], f[k]+coins.size()) + 1;
                } else {
                    f[i][j] = INF;
                }
                //cout<<"f["<<i<<"]["<<j<<"] = "<<f[i][j]<<endl;
            }
        }
        int ret = *min_element(f[amount], f[amount]+coins.size());
        if(ret >= INF) {
            return -1;
        }
        return ret;
    }
};