class Solution {
public:
    int integerBreak(int n) {
        vector<int> f(n+1, 0);
        f[1] = 1;
        for(int i = 2;i<=n;i++) {
            for(int j = 1;j<=i/2;j++) {
                int j2 = i-j;
                f[i] = max(f[i], max(j, f[j])*max(j2, f[j2]));
            }
        }
        return f[n];
    }
};