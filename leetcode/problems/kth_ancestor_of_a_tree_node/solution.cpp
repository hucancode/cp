class TreeAncestor {
    vector<vector<int>> powerParent; //powerParent[i][j] = parent 2^j of i (if exist)
    vector<vector<int>> children;
public:
    TreeAncestor(int n, vector<int>& parent) {
        int m = parent.size();
        children.resize(n);
        for(int i = 0;i<m;i++) {
            auto x = parent[i];
            if(x != -1) {
                children[x].push_back(i);
            }
        }
        powerParent.resize(n);
        dfs();
    }

    void dfs() {
        vector<int> path;
        stack<int> st;
        st.push(0);
        while(!st.empty()) {
            auto u = st.top();
            st.pop();
            if(u == -1) {
                path.pop_back();
                continue;
            }
            path.push_back(u);
            calculatePowerParent(path);
            st.push(-1);
            for(auto v: children[u]) {
                st.push(v);
            }
        }
    }

    void calculatePowerParent(vector<int>& path) {
        int x = *path.rbegin();
        int n = path.size();
        int i = 1;
        while(i < n) {
            auto y = *(path.rbegin() + i);
            powerParent[x].push_back(y);
            i = i*2;
        }
    }

    int getKthAncestor(int node, int k) {
        if(k == 0) {
            return node;
        }
        int n = 1;
        int x = 0;
        while(n*2 <= k) {
            n = n*2;
            x++;
        }
        if(x >= powerParent[node].size()) {
            return -1;
        }
        node = powerParent[node][x];
        return getKthAncestor(node, k-n);
    }
};

/**
 * Your TreeAncestor object will be instantiated and called as such:
 * TreeAncestor* obj = new TreeAncestor(n, parent);
 * int param_1 = obj->getKthAncestor(node,k);
 */