class Solution {
public:
    int minInsertions(string s) {
        int n = s.size();
        vector<vector<int>> f(n+1, vector<int>(n+1,0));
        f[0][0] = 0;
        for(int i = 1;i<n;i++) {
            f[0][i] = i;
            f[i][0] = i;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=n;j++) {
                if(s[i-1] == s[n-j]) {
                    f[i][j] = f[i-1][j-1];
                    continue;
                }
                f[i][j] = min(min(f[i-1][j]+1, f[i][j-1]+1), f[i-1][j-1]+2);
            }
        }
        return f[n][n]/2;
    }
};