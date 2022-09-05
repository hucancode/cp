class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        int ret = 0;
        queue<pair<int, int>> q;
        for(int i = 0;i<m; i++) {
            for(int j = 0;j<n; j++) {
                if(grid[i][j] == '1') {
                    ret++;
                    q.push(make_pair(i,j));
                    while(!q.empty()) {
                        auto loc = q.front();
                        q.pop();
                        auto x = loc.first;
                        auto y = loc.second;
                        if(x >= m || x < 0) {
                            continue;
                        }
                        if(y >= n || y < 0) {
                            continue;
                        }
                        if(grid[x][y] != '1') {
                            continue;
                        }
                        grid[x][y] = '0';
                        q.push(make_pair(x+1, y));
                        q.push(make_pair(x-1, y));
                        q.push(make_pair(x, y+1));
                        q.push(make_pair(x, y-1));
                    }
                }
            }
        }
        return ret;
    }
};