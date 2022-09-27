class Solution {
public:
    bool isMatch(string s, string p) {
        int n = s.size();
        int m = p.size();
        vector<vector<bool>> f(n+1, vector<bool>(m+1, false));
        for(int i = 0;i<=n;i++) {
            for(int j = 0;j<=m;j++) {
                if(j == 0) {
                    f[i][j] = i == 0;
                    continue;
                }
                if(i == 0) {
                    f[i][j] = p[j-1] == '*' && f[i][j-1];
                    continue;
                }
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
        // cout<<"_ ";
        // for(int i = 0;i<m;i++) {
        //     cout<<p[i]<<" ";
        // }
        // cout<<endl;
        // for(int i = 1;i<=n;i++) {
        //     cout<<s[i-1]<<" ";
        //     for(int j = 1;j<=m;j++) {
        //         cout<<f[i][j]<<" ";
        //     }
        //     cout<<endl;
        // }
        // cout<<endl;
        return f[n][m];
    }
};