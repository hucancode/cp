class Solution {
public:
    vector<int> parent;
    int find(int x) {
        if(parent[x] == x) {
            return x;
        }
        return find(parent[x]);
    }

    void unionXY(int x, int y) {
        int px = find(x);
        int py = find(y);
        parent[py] = px;
    }

    vector<int> solve(vector<vector<int>>& edges) {
        int n = edges.size();
        parent.resize(n+1);
        iota(parent.begin(), parent.end(), 0);
        bool found = false;
        vector<int> ret = {0,0};
        for(auto e: edges) {
            int u = e[0];
            int v = e[1];
            bool valid = find(v) == v && find(u) != v;
            if(valid) {
                unionXY(u,v);
                continue;
            }
            if(found) {
                // the edge we found before is not the answer 
                found = false;
                ret = {0, 0};
                break;
            }
            ret = e;
            found = true;
        }
        return ret;
    }

    vector<int> findRedundantDirectedConnection(vector<vector<int>>& edges) {
        auto ret = solve(edges);
        if(ret[0] == 0) {
            reverse(edges.begin(), edges.end());
            ret = solve(edges);
        }
        return ret;
    }
};