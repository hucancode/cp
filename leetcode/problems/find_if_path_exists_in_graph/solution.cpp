class Solution {
public:
    bool validPath(int n, vector<vector<int>>& edges, int source, int destination) {
        map<int, vector<int>> adj;
        for(auto e: edges) {
            adj[e[0]].push_back(e[1]);
            adj[e[1]].push_back(e[0]);
        }
        vector<bool> vis(n, false);
        stack<int> st;
        st.push(source);
        while(!st.empty()) {
            auto u = st.top();
            st.pop();
            if(vis[u]) {
                continue;
            }
            vis[u] = true;
            if(u == destination) {
                return true;
            }
            for(auto v: adj[u]) {
                st.push(v);
            }
        }
        return false;
    }
};