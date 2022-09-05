class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        queue<pair<int, int>> q;
        for(int i = 0;i<m; i++) {
            for(int j = 0;j<n; j++) {
                if(grid[i][j] == 2) {
                    q.push(make_pair(i+1, j));
                    q.push(make_pair(i-1, j));
                    q.push(make_pair(i, j+1));
                    q.push(make_pair(i, j-1));
                }
            }
        }
        int ret = 0;
        while(!q.empty()) {
            int k = q.size();
            bool rotten = false;
            for(int i = 0;i<k;i++) {
                auto loc = q.front();
                q.pop();
                int x = loc.first;
                int y = loc.second;
                if(x >= m || x < 0) {
                    continue;
                }
                if(y >= n || y < 0) {
                    continue;
                }
                if(grid[x][y] != 1) {
                    continue;
                }
                rotten = true;
                grid[x][y] = 2;
                q.push(make_pair(x+1, y));
                q.push(make_pair(x-1, y));
                q.push(make_pair(x, y+1));
                q.push(make_pair(x, y-1));
            }
            if(rotten) {
                ret++;
            }
        }
        for(int i = 0;i<m; i++) {
            for(int j = 0;j<n; j++) {
                if(grid[i][j] == 1) {
                    return -1;
                }
            }
        }
        return ret;
    }
};