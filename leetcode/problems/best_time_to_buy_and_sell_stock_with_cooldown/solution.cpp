class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        vector<int> buy(n+1, 0);
        vector<int> sell(n+1, 0);
        int ret = 0;
        for(int i = 2;i<=n;i++) {
            sell[i] = sell[i-1];
            for(int j = 1;j<i;j++) {
                auto profit = prices[i-1] - prices[j-1];
                sell[i] = max(sell[i], buy[j] + profit);
            }
            buy[i] = max(buy[i-1], sell[i-2]);
            ret = max(ret, sell[i]);
        }
        // cout<<"sell profit: ";
        // for(int i = 1;i<=n;i++) {
        //     cout<<sell[i]<<" ";
        // }
        // cout<<endl;
        // cout<<"buy profit: ";
        // for(int i = 1;i<=n;i++) {
        //     cout<<buy[i]<<" ";
        // }
        // cout<<endl;
        return ret;
    }
};