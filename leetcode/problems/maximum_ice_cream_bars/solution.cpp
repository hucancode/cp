class Solution {
public:
    int maxIceCream(vector<int>& costs, int coins) {
        int n = costs.size();
        sort(costs.begin(), costs.end());
        int ret = 0;
        int i = 0;
        while(i<n && coins > 0) {
            if(coins >= costs[i]) {
                coins -= costs[i];
                ret++;
            }
            i++;
        }
        return ret;
    }
};