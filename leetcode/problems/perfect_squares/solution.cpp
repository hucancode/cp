class Solution {
public:
    int numSquares(int n) {
        vector<int> f(n+1, 1e5);
        f[0] = 0;
        for(int i = 1;i<=n;i++) {
            int k = 1;
            int j = i-k*k;
            while(j >= 0) {
                f[i] = min(f[i], f[j] + 1);
                k++;
                j = i - k*k;
            }
        }
        return f[n];
    }
};