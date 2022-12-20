class Solution {
public:
    int swimInWater(vector<vector<int>>& grid) {
        int n = grid.size();
        int m = grid[0].size();
        vector<vector<bool>> vis(n, vector<bool>(m, false));
        typedef tuple<int,int,int> state;
        priority_queue<state, vector<state>, greater<state>> q;
        q.emplace(grid[0][0], 0, 0);
        while(!q.empty()) {
            int h, x, y;
            tie(h,x,y) = q.top();
            q.pop();
            if(vis[x][y]) {
                continue;
            }
            vis[x][y] = true;
            if(x == n-1 && y == m-1) {
                return h;
            }
            if(x > 0) {
                int next = max(h, grid[x-1][y]);
                q.emplace(next, x-1,y);
            }
            if(x < n-1) {
                int next = max(h, grid[x+1][y]);
                q.emplace(next, x+1,y);
            }
            if(y > 0) {
                int next = max(h, grid[x][y-1]);
                q.emplace(next, x,y-1);
            }
            if(y < m-1) {
                int next = max(h, grid[x][y+1]);
                q.emplace(next, x,y+1);
            }
        }
        cout<<"something went wrong";
        return 0;
    }
};