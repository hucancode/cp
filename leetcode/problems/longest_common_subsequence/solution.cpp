class Solution {
public:
    int longestCommonSubsequence(string a, string b) {
        int m = a.size();
        int n = b.size();
        vector<vector<int>> f(m+1, vector<int>(n+1, 0));
        for(int i = 1; i <= m; i++) {
            for(int j = 1; j <= n; j++) {
              bool matched = a[i-1] == b[j-1];
              if(matched) {
                int prev = f[i-1][j-1];
                f[i][j] = prev + 1;
              } else {
                int prev_a = f[i-1][j];
                int prev_b = f[i][j-1];
                f[i][j] = max(prev_a, prev_b);
              }
            }
        }
        return f[m][n];
    }
};