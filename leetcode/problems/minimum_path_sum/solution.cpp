class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        const int INF = 100000;
        int n = grid.size();
        int m = grid[0].size();
        vector<vector<int>> f(n, vector<int>(m, INF));
        f[0][0] = 0;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                if(i > 0) {
                    f[i][j] = min(f[i][j], f[i-1][j]);
                }
                if(j > 0) {
                    f[i][j] = min(f[i][j], f[i][j-1]);
                }
                f[i][j] += grid[i][j];
            }
        }
        
        return f[n-1][m-1];
    }
};