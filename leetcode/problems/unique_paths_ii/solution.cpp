class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        vector<vector<int>> f(m, vector<int>(n));
        f[0][0] = grid[0][0] == 0?1:0;
        for(int i = 1;i<m;i++) {
            f[i][0] = grid[i][0] == 0?f[i-1][0]:0;
        }
        for(int j = 1;j<n;j++) {
            f[0][j] = grid[0][j] == 0?f[0][j-1]:0;
        }
        for(int i = 1;i<m;i++) {
            for(int j = 1;j<n;j++) {
                if(grid[i][j] == 1) {
                    f[i][j] = 0;
                } else {
                    f[i][j] = f[i-1][j] + f[i][j-1];
                }
            }
        }
        // cout<<"f: "<<endl;
        // for(int i = 0;i<m;i++) {
        //     for(int j = 0;j<n;j++) {
        //         cout<<f[i][j]<<" ";
        //     }
        //     cout<<endl;
        // }
        return f[m-1][n-1];
    }
};