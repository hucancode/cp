class Solution {
public:
    void dfs(map<int, vector<int>>& adj, string& labels, vector<int>& ans) {
        int n = labels.size();
        stack<pair<int,int>> st;
        vector<bool> vis(n, false);
        vector<vector<int>> count(n, vector<int>(26,0));
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
                auto key = labels[u] - 'a';
                count[u][key]++;
                if(p >= 0) {
                    for(key = 0;key<26;key++) {
                        count[p][key] += count[u][key];
                    }
                }
            }
        }
        for(int i = 0;i<n;i++) {
            auto key = labels[i]-'a';
            ans[i] = count[i][key];
        }
    }
    vector<int> countSubTrees(int n, vector<vector<int>>& edges, string labels) {
        vector<int> ans(n, 0);
        map<int, vector<int>> adj;
        for(auto e: edges) {
            adj[e[0]].push_back(e[1]);
            adj[e[1]].push_back(e[0]);
        }
        dfs(adj, labels, ans);
        return ans;
    }
};