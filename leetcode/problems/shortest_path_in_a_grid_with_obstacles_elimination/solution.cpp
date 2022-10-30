class Solution {
public:
    int shortestPath(vector<vector<int>>& grid, int kmax) {
        typedef tuple<int, int, int, int> state;
        int n = grid.size();
        int m = grid[0].size();
        int brick = 0;
        for(auto row: grid) {
            for(auto x: row) {
                brick += x;
            }
        }
        kmax = min(kmax, brick);
        vector<vector<vector<bool>>> vis(n, vector<vector<bool>>(m, vector<bool>(kmax+1, false)));
        queue<state> q;
        // at position 0,0,used 0 breaker so far, cost is 0
        int ret = -1;
        q.emplace(0,0,0,0);
        while(!q.empty()) {
            int i,j,k,cost;
            tie(i,j,k,cost) = q.front();
            q.pop();
            bool valid = i>=0&&i<n&&j>=0&&j<m;
            if(!valid) {
                continue;
            }
            if(grid[i][j]) {
                k++;
            }
            if(k>kmax) {
                continue;
            }
            if(vis[i][j][k]) {
                continue;
            }
            vis[i][j][k] = true;
            if(i==n-1 && j==m-1) {
                if(ret == -1) {
                    ret = cost;
                } else {
                    ret = min(ret, cost);
                }
            }
            // visit 4 neighbors
            q.emplace(i+1,j,k,cost+1);
            q.emplace(i-1,j,k,cost+1);
            q.emplace(i,j+1,k,cost+1);
            q.emplace(i,j-1,k,cost+1);
        }
        return ret;
    }
};