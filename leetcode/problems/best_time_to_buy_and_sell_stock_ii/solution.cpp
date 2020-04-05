class Solution {
public:
    int maxProfit(vector<int>& prices)
    {
        int n = prices.size();
        if(n<2)
        {
            return 0;
        }
        vector<int> keep(n, 0);// best way to keep the stock at day i
        vector<int> sell(n, 0);// best way to sell the stock at day i
        keep[0] = prices[0];
        sell[0] = 0;
        for(int i=1;i<n;i++)
        {
            keep[i] = min(keep[i-1], prices[i] - sell[i-1]);
            sell[i] = max(prices[i] - keep[i-1], sell[i-1]);
        }
        return sell[n-1];
    }
};