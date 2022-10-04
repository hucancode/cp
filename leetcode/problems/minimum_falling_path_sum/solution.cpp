class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& matrix) {
        const int INF = 100000;
        int n = matrix.size();
        vector<vector<int>> f(n, vector<int>(n, INF));
        for(int j = 0;j<n;j++) {
            f[0][j] = matrix[0][j];
        }
        for(int i = 1;i<n;i++) {
            for(int j = 0;j<n;j++) {
                f[i][j] = min(f[i][j], f[i-1][j]);
                if(j > 0) {
                    f[i][j] = min(f[i][j], f[i-1][j-1]);
                }
                if(j < n-1) {
                    f[i][j] = min(f[i][j], f[i-1][j+1]);
                }
                f[i][j] += matrix[i][j];
            }
        }
        int ret = INF;
        for(int j = 0;j<n;j++) {
            ret = min(ret, f[n-1][j]);
        }
        return ret;
    }
};