class Solution {
public:
    int minFallingPathSum(vector<vector<int>>& grid) {
        int n = grid.size();
        if(n==1) {
            return grid[0][0];
        }
        // f[j] = min value of grid[i] with grid[i][j] as the last choice
        vector<int> f(grid[0].begin(), grid[0].end());
        // m[i] = min value of f excluding f[i]
        vector<int> m(n);
        vector<int> left(n);
        vector<int> right(n);
        for(int i = 1;i<n;i++) {
            left[0] = f[0];
            for(int j = 1;j<n-1;j++) {
                left[j] = min(left[j-1], f[j]);
            }
            right[n-1] = f[n-1];
            for(int j = n-2;j>=1;j--) {
                right[j] = min(right[j+1], f[j]);
            }
            m[0] = right[1];
            m[n-1] = left[n-2];
            for(int j = 1;j<n-1;j++) {
                m[j] = min(left[j-1], right[j+1]);
            }
            for(int j = 0;j<n;j++) {
                f[j] = grid[i][j] + m[j];
            }
        }
        return *min_element(f.begin(), f.end());
    }
};