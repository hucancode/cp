class Solution {
public:
    vector<vector<int>> updateMatrix(vector<vector<int>>& mat) {
        int n = mat.size();
        int m = mat[0].size();
        vector<vector<int>> f(n);
        for(int i = 0;i<n;i++) {
            f[i].resize(m);
        }
        const int INF = 20001;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                f[i][j] = INF;
                if(mat[i][j] == 0) {
                    f[i][j] = 0;
                }
            }
        }
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                if(i > 0) {
                    f[i][j] = min(f[i-1][j] + 1, f[i][j]);
                }
                if(j > 0) {
                    f[i][j] = min(f[i][j-1] + 1, f[i][j]);
                }
            }
        }
        for(int i = n-1;i>=0;i--) {
            for(int j = m-1;j>=0;j--) {
                if(i < n-1) {
                    f[i][j] = min(f[i+1][j] + 1, f[i][j]);
                }
                if(j < m - 1) {
                    f[i][j] = min(f[i][j+1] + 1, f[i][j]);
                }
            }
        }
        return f;
    }
};