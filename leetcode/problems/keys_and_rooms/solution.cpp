class Solution {
public:
    bool canVisitAllRooms(vector<vector<int>>& rooms) {
        int n = rooms.size();
        int visited = 0;
        vector<bool> vis(n, false);
        queue<int> q;
        q.push(0);
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            if(vis[u]) {
                continue;
            }
            vis[u] = true;
            visited++;
            for(auto v: rooms[u]) {
                q.push(v);
            }
        }
        return visited == n;
    }
};