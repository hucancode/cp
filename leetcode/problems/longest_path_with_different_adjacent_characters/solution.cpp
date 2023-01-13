class Solution {
public:
    int longestPath(vector<int>& parent, string s) {
        map<int, vector<int>> children;
        int n = parent.size();
        for(int u=0;u<n;u++) {
            int p = parent[u];
            children[p].push_back(u);
        }
        stack<pair<int,int>> st;
        vector<bool> vis(n, false);
        vector<vector<int>> score(n);
        st.emplace(-1, 0);
        int ret = 1;
        while(!st.empty()) {
            int p,u;
            tie(p,u) = st.top();
            bool done = true;
            for(auto v: children[u]) {
                if(vis[v]) {
                    continue;
                }
                st.emplace(u,v);
                done = false;
            }
            vis[u] = true;
            if(done) {
                sort(score[u].rbegin(), score[u].rend());
                if(score[u].size()>2) {
                    score[u].resize(2);
                }
                int k2 = accumulate(score[u].begin(), score[u].end(), 1);
                if(score[u].size()>1) {
                    score[u].resize(1);
                }
                int k1 = accumulate(score[u].begin(), score[u].end(), 1);
                if(p >= 0 && s[u] != s[p]) {
                    score[p].push_back(k1);
                }
                ret = max(ret, k2);
                st.pop();
            }
        }
        return ret;
    }
};