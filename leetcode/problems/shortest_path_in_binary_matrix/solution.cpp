class Solution {
public:
    int shortestPathBinaryMatrix(vector<vector<int>>& grid) {
        int n = grid.size();
        queue<tuple<int,int,int>> q;
        q.emplace(0,0,1);
        while(!q.empty()) {
            int x, y, d;
            tie(x, y, d) = q.front();
            q.pop();
            if(x<0||x>=n||y<0||y>=n || grid[x][y]) {
                continue;
            }
            if(x == n-1 && y == n-1) {
                return d;
            }
            grid[x][y] = 1;
            d++;
            for(int dx=-1;dx<=1;dx++) {
                for(int dy=-1;dy<=1;dy++) {
                    q.emplace(x+dx, y+dy, d);
                }
            }
        }
        return -1;
    }
};