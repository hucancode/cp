class Solution {
public:
    int minScore(int n, vector<vector<int>>& roads) {
        map<int, vector<pair<int,int>>> adj;
        for(auto r: roads) {
            adj[r[0]].emplace_back(r[1], r[2]);
            adj[r[1]].emplace_back(r[0], r[2]);
        }
        int ret = 1e4;
        queue<int> q;
        q.push(1);
        vector<bool> vis(n+1, false);
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            if(vis[u]) continue;
            vis[u] = true;
            for(auto v: adj[u]) {
                q.push(v.first);
                ret = min(ret, v.second);
            }
        }
        return ret;
    }
};