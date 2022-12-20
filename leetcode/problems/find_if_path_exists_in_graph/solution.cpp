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

    bool validPath(int n, vector<vector<int>>& edges, int source, int destination) {
        parent.resize(n);
        iota(parent.begin(), parent.end(), 0);
        for(auto e: edges) {
            unionXY(e[0], e[1]);
        }
        return find(source) == find(destination);
    }
};