class Solution {
public:
    bool samePerson(vector<string>& a, vector<string>& b) {
        if(a[0] != b[0]) {
            return false;
        }
        auto i = a.begin() + 1;
        auto j = b.begin() + 1;
        auto n = upper_bound(i, a.end(), *b.rbegin());
        auto m = upper_bound(j, b.end(), *a.rbegin());
        while(i < n && j < m) {
            if(*i == *j) {
                return true;
            }
            auto nextj = lower_bound(j, m, *i);
            auto nexti = lower_bound(i, n, *j);
            i = nexti;
            j = nextj;
        }
        return false;
    }
    vector<vector<string>> accountsMerge(vector<vector<string>>& accounts) {
        int n = accounts.size();
        vector<vector<bool>> edge(n, vector<bool>(n, false));
        for(int i = 0;i<n;i++) {
            sort(accounts[i].begin()+1, accounts[i].end());
        }
        for(int i = 0;i<n-1;i++) {
            for(int j = i+1;j<n;j++) {
                if(samePerson(accounts[i], accounts[j])) {
                    edge[i][j] = true;
                    edge[j][i] = true;
                }
            }
        }
        vector<vector<string>> ret;
        ret.reserve(n);
        vector<int> vis(n);
        for(int i = 0;i<n;i++) {
            vis[i] = i;
        }
        while(!vis.empty()) {
            int u = vis[0];
            vector<string> account;
            set<string> emails;
            account.push_back(accounts[u][0]);
            stack<int> st;
            st.push(u);
            vis.erase(vis.begin());
            while(!st.empty()) {
                u = st.top();
                st.pop();
                emails.insert(accounts[u].begin()+1, accounts[u].end());
                for(auto it = vis.begin();it!=vis.end();) {
                    auto v = *it;
                    if(edge[u][v]) {
                        st.push(v);
                        it = vis.erase(it);
                    } else {
                        it++;
                    }
                }
            }
            account.insert(account.end(), emails.begin(), emails.end());
            ret.push_back(account);
        }
        return ret;
    }
};