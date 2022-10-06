class Solution {
public:
    vector<vector<int>> matrixBlockSum(vector<vector<int>>& mat, int k) {
        int n = mat.size();
        int m = mat[0].size();
        vector<vector<int>> sum(n+1, vector<int>(m+1, 0));
        vector<vector<int>> ret(n, vector<int>(m, 0));
        for(int i = 1;i<=n;i++) {
            int rowSum = 0;
            for(int j = 1;j<=m;j++) {
                rowSum += mat[i-1][j-1];
                sum[i][j] = rowSum + sum[i-1][j];
            }
        }
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                int i0 = max(0, i - k);
                int j0 = max(0, j - k);
                int i1 = min(n, i + k + 1);
                int j1 = min(m, j + k + 1);
                auto big = sum[i1][j1];
                auto small = sum[i0][j0];
                auto left = sum[i1][j0];
                auto up = sum[i0][j1];
                ret[i][j] = big - left - up + small;
            }
        }
        return ret;
    }
};