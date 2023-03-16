class Solution {
public:
    int numDistinct(string s, string t) {
        int n = s.size();
        int m = t.size();
        vector<vector<unsigned int>> f(n+1, vector<unsigned int>(m+1, 0));
        for(int i = 0;i<=n;i++) {
            f[i][0] = 1;
        }
        for(int j = 1;j<=m;j++) {
            for(int i = j;i<=n;i++) {
                f[i][j] += f[i-1][j];
                if(s[i-1] == t[j-1]) {
                    f[i][j] += f[i-1][j-1];
                }
                f[i][j] %= INT_MAX;
            }
        }
        return f[n][m];
        cout<<"f: "<<endl;
        for(auto row: f) {
            for(auto x: row) {
                cout<<x<<" ";
            }
            cout<<endl;
        }
        return f[n][m];
    }
};