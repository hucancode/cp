class Solution {
public:
    int buyChoco(vector<int>& prices, int money) {
        int x = 5000;
        int y = 5000;
        for(auto p: prices) {
            y = min(y, max(x,p));
            x = min(x, p);
        }
        if(money >= x + y) {
            return money - x - y;
        }
        return money;
    }
};