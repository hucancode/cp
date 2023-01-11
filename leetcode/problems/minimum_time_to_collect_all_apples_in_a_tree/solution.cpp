class Solution {
public:
    void dfs(map<int, vector<int>>& adj, vector<bool>& hasApple) {
        int n = hasApple.size();
        vector<bool> vis(n);
        stack<pair<int,int>> st;
        st.emplace(-1,0);
        while(!st.empty()) {
            int p,u;
            tie(p,u) = st.top();
            vis[u] = true;
            bool done = true;
            for(auto v: adj[u]) {
                if(vis[v]) {
                    continue;
                }
                st.emplace(u,v);
                done = false;
            }
            if(done) {
                st.pop();
                if(p >= 0) {
                    hasApple[p] = hasApple[p] || hasApple[u];
                }
            }
        }
    }
    int minTime(int n, vector<vector<int>>& edges, vector<bool>& hasApple) {
        map<int, vector<int>> adj;
        for(auto e: edges) {
            adj[e[0]].push_back(e[1]);
            adj[e[1]].push_back(e[0]);
        }
        dfs(adj, hasApple);
        int apples = accumulate(hasApple.begin(), hasApple.end(), 0);
        if(apples <= 1) {
            return 0;
        }
        if(apples == 2) {
            return 2;
        }
        return (apples - 1)*2;
    }
};