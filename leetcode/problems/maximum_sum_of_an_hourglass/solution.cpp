class Solution {
public:
    int maxSum(vector<vector<int>>& grid) {
        int n = grid.size();
        int m = grid[0].size();
        vector<vector<int>> dx3(n, vector<int>(m));
        vector<vector<int>> dx1(n, vector<int>(m));
        vector<vector<int>> f(n, vector<int>(m, 0));
        for(int i = 0;i<n;i++) {
            for(int j = 3;j<m;j++) {
                dx3[i][j] = grid[i][j] - grid[i][j-3];
            }
        }
        for(int i = 0;i<n;i++) {
            for(int j = 1;j<m-1;j++) {
                dx1[i][j] = grid[i][j] - grid[i][j-1];
            }
        }
        int ret = 0;
        for(int i = 2;i<n;i++) {
            f[i][2] += grid[i-2][0] + grid[i-2][1] + grid[i-2][2];
            f[i][2] += grid[i-1][1];
            f[i][2] += grid[i][0] + grid[i][1] + grid[i][2];
            ret = max(ret, f[i][2]);
        }
        for(int i = 2;i<n;i++) {
            for(int j = 3;j<m;j++) {
                f[i][j] = f[i][j-1];
                f[i][j] += dx3[i-2][j];
                f[i][j] += dx1[i-1][j-1];
                f[i][j] += dx3[i][j];
                ret = max(ret, f[i][j]);
            }
        }
        // cout<<"f: "<<endl;
        // for(int i = 0;i<n;i++) {
        //     for(int j = 0;j<m;j++) {
        //         cout<<f[i][j]<<" ";
        //     }
        //     cout<<endl;
        // }
        return ret;
    }
};