class Solution {
public:
    int nearestExit(vector<vector<char>>& maze, vector<int>& entrance) {
        int n = maze.size();
        int m = maze[0].size();
        vector<vector<int>> dist(n, vector<int>(m, 2e9));
        vector<vector<bool>> vis(n, vector<bool>(m, false));
        typedef pair<int,int> pi;
        queue<pi> q;
        q.emplace(entrance[0], entrance[1]);
        dist[entrance[0]][entrance[1]] = 0;
        vis[entrance[0]][entrance[1]] = true;
        while(!q.empty()) {
            int ux, uy;
            tie(ux,uy) = q.front();
            q.pop();
            if(!(ux == entrance[0] && uy == entrance[1]) && (ux == 0 || ux == n-1 || uy == 0 || uy == m-1)) {
                return dist[ux][uy];
            }
            for(int vx = ux-1;vx<=ux+1;vx++) {
                if(vx < 0 || vx >= n) {
                    continue;
                }
                if(vis[vx][uy]) {
                    continue;
                }
                vis[vx][uy] = true;
                if(maze[vx][uy] == '+') {
                    continue;
                }
                dist[vx][uy] = dist[ux][uy]+1;
                q.emplace(vx,uy);
            }
            for(int vy = uy-1;vy<=uy+1;vy++) {
                if(vy < 0 || vy >= m) {
                    continue;
                }
                if(vis[ux][vy]) {
                    continue;
                }
                vis[ux][vy] = true;
                if(maze[ux][vy] == '+') {
                    continue;
                }
                dist[ux][vy] = dist[ux][uy]+1;
                q.emplace(ux,vy);
            }
        }
        return -1;
    }
};