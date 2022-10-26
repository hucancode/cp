class Solution {
public:
    int countOrders(int n) {
        const int INF = 1e9+7;
        vector<int> f(n+1, 0);
        f[1] = 1;
        for(int i = 2;i<=n;i++) {
            int prevSize = (i-1)*2;
            int j = prevSize+1;
            // there are (i-1)*2+1 positions to put Di
            // for each decision of Di at j, there are j way to put Pi
            // as a result, there are j+(j-1)+(j-2)+...+1 ways to pick (Pi,Di) combination
            int k = (j+1)*j/2;
            f[i] = (f[i-1]*(long long)k)%INF;
        }
        return f[n];
    }
};