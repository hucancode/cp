class Solution {
public:
    int maxProfit(vector<int>& prices, int fee) {
        int n = prices.size();
        vector<int> buy(n, -50001);
        vector<int> sell(n, 0);
        int ret = 0;
        buy[0] = -prices[0];
        for(int i = 1;i<n;i++) {
            sell[i] = max(sell[i-1], buy[i-1] + prices[i] - fee);
            buy[i] = max(buy[i-1], sell[i-1] - prices[i]);
            ret = max(ret, sell[i]);
        }
        // cout<<"sell profit: ";
        // for(int i = 0;i<n;i++) {
        //     cout<<sell[i]<<" ";
        // }
        // cout<<endl;
        // cout<<"buy profit: ";
        // for(int i = 0;i<n;i++) {
        //     cout<<buy[i]<<" ";
        // }
        // cout<<endl;
        return ret;
    }
};