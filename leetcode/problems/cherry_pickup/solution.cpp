typedef vector<int> vi;
typedef vector<vi> vvi;
typedef vector<vvi> v3i;
typedef vector<v3i> v4i;
class Solution {
public:
    int calculateGain(vvi& grid, int x1, int y1, int x2, int y2) {
        int gain = 0;
        if(x1 == x2 && y1 == y2) {
            if(grid[x1][y1] == 1) {
                gain = 1;
            }
        } else {
            if(grid[x1][y1] == 1) {
                gain++;
            }
            if(grid[x2][y2] == 1) {
                gain++;
            }
        }
        return gain;
    }
    void updateMaxGain(vvi& grid, v3i& f, int prev, int k, int x1, int x2) {
        int n = grid.size();
        int y1 = k - x1;
        int y2 = k - x2;
        if(x1 >= n || x2 >= n || y1 >= n || y2 >= n) {
            return;
        }
        f[k][x1][x2] = max(f[k][x1][x2], prev + 
            calculateGain(grid, x1, y1, x2, y2));
    }
    int cherryPickup(vector<vector<int>>& grid) {
        int n = grid.size();
        int step = n*2-1;
        v3i f(step,vvi(n,vi(n,-1)));
        // f[k][i][j] = moved k steps so far, currently at x1 = i, (y1 = k-i), x2 = j (y2 = k-j)
        // what is the maximum cherry picked
        f[0][0][0] = grid[0][0]==1?1:0;
        for(int k = 0;k<step;k++) {
            for(int x1 = 0;x1<=k;x1++) {
                for(int x2 = 0;x2<=k;x2++) {
                    int y1 = k-x1;
                    int y2 = k-x2;
                    bool invalid = x1 >=n || x2 >=n || y1 >= n || y2 >=n || 
                        grid[x1][y1] == -1 || grid[x2][y2] == -1 || f[k][x1][x2] == -1;
                    if(invalid) {
                        continue;
                    }
                    int prev = f[k][x1][x2];
                    updateMaxGain(grid, f, prev, k+1, x1+1, x2+1);
                    updateMaxGain(grid, f, prev, k+1, x1+1, x2);
                    updateMaxGain(grid, f, prev, k+1, x1, x2+1);
                    updateMaxGain(grid, f, prev, k+1, x1, x2);
                }
            }
            // cout<<"at step "<<k<<endl;
            // for(auto x: f[k]) {
            //     for(auto y: x) {
            //         cout<<y<<' ';
            //     }
            //     cout<<endl;
            // }
        }
        return max(0, f[step-1][n-1][n-1]);
    }
};