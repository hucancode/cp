class Solution {
public:
    int minimumDeleteSum(string s1, string s2) {
        int ret = 0;
        int n = s1.size();
        int m = s2.size();
        vector<vector<int>> f(n+1, vector<int>(m+1, 0));
        for(int i = 1;i<=n;i++) {
            f[i][0] = f[i-1][0] + s1[i-1];
        }
        for(int j = 1;j<=m;j++) {
            f[0][j] = f[0][j-1] + s2[j-1];
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                if(s1[i-1] == s2[j-1]) {
                    f[i][j] = f[i-1][j-1];
                } else {
                    int delete1 = f[i-1][j] + s1[i-1];
                    int delete2 = f[i][j-1] + s2[j-1];
                    int delete12 = f[i-1][j-1] + s1[i-1] + s2[j-1];
                    f[i][j] = min(delete12, min(delete1, delete2));
                }
                // cout<<"f["<<i<<"]["<<j<<"] = "<<
                //     "f["<<s1[i-1]<<"]["<<s2[j-1]<<"] = "<<f[i][j]<<endl;
            }
        }
        return f[n][m];
    }
};