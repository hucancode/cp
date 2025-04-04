class Solution {
public:
    bool isMatch(string s, string p) {
        int n = s.size();
        int m = p.size();
        vector<vector<bool>> f(n+1, vector<bool>(m+1, false));
        f[0][0] = true;
        for(int j = 1;j<=m;j++) {
            f[0][j] = p[j-1] == '*' && f[0][j-1];
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                if(p[j-1] == '?') {
                    f[i][j] = f[i-1][j-1];
                    continue;
                }
                if(p[j-1] == '*') {
                    f[i][j] = f[i-1][j] || f[i-1][j-1] || f[i][j-1];
                    continue;
                }
                f[i][j] = s[i-1] == p[j-1] && f[i-1][j-1];
            }
        }
        return f[n][m];
    }
};
