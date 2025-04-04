class Solution {
public:
    bool isMatch(string s, string p) {
        int n = p.size();
        int m = s.size();
        vector<vector<bool>> f(n+1, vector<bool>(m+1, false));
        f[0][0] = true;
        for(int i = 2;i<=n;i++) {
            f[i][0] = p[i-1] == '*' && f[i-2][0];
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                if (p[i-1] != '*') {
                    f[i][j] = (p[i-1] == s[j-1] || p[i-1] == '.') && f[i-1][j-1];
                    continue;
                }
                if (p[i-2] == s[j-1] || p[i-2] == '.') {
                    f[i][j] = f[i-2][j-1] || f[i][j-1] || f[i-2][j];
                } else {
                    f[i][j] = f[i-2][j];
                }
            }
        }
        return f[n][m];
    }
};
