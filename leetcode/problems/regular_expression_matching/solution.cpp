class Solution {
public:
    bool good(char a, char b) {
        return a == '.' || a == b;
    }
    bool isMatch(string s, string p) {
        int n = s.size();
        int m = p.size();
        vector<vector<bool>> f(n+1, vector<bool>(m+1, false));
        f[0][0] = true;
        for(int j = 2;j<=m;j+=2) {
            if(p[j-1] !='*') {
                break;
            }
            f[0][j] = true;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                auto token = p[j-1];
                auto c = s[i-1];
                //cout<<"check "<<token<<s[i-1]<<endl;
                if(token != '*') {
                    f[i][j] = f[i-1][j-1] && good(token, c);
                    continue;
                }
                token = p[j-2];
                f[i][j] = f[i][j-2];
                int k = i;
                while(k > 0 && good(token, s[k-1]) && !f[i][j]) {
                    k--;
                    f[i][j] = f[k][j-2];
                }
                //cout<<"f["<<i<<"-"<<j<<"] = "<<f[i][j]<<endl;
            }
        }
        return f[n][m];
    }
};