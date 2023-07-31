class Solution {
public:
    int minDistance(string word1, string word2) {
        int n = word1.size();
        int m = word2.size();
        vector<vector<int>> f(n+1, vector<int>(m+1, 0));
        for(int i = 1;i<=n;i++) {
            f[i][0] = i;
        }
        for(int j = 1;j<=m;j++) {
            f[0][j] = j;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                if(word1[i-1] == word2[j-1]) {
                    f[i][j] = f[i-1][j-1];
                } else {
                    int delete1 = f[i-1][j] + 1;
                    int delete2 = f[i][j-1] + 1;
                    int delete12 = f[i-1][j-1] + 2;
                    f[i][j] = min(delete12, min(delete1, delete2));
                }
            }
        }
        return f[n][m];
    }
};