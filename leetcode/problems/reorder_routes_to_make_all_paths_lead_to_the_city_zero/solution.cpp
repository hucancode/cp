class Solution {
public:
    int minReorder(int n, vector<vector<int>>& connections) {
        vector<vector<int>> out(n);
        vector<vector<int>> in(n);
        for(auto edge: connections) {
            auto a = edge[0];
            auto b = edge[1];
            out[a].push_back(b);
            in[b].push_back(a);
        }
        vector<bool> vis(n, false);
        int ret = 0;
        queue<int> q;
        q.push(0);
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            if(vis[u]) continue;
            vis[u] = true;
            for(auto v: out[u]) {
                if(vis[v]) continue;
                q.push(v);
                ret++;
            }
            for(auto v: in[u]) {
                q.push(v);
            }
        }
        return ret;
    }
};