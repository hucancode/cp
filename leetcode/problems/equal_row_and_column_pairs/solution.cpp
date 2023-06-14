class Node {
public:
    int value;
    int count;
    map<int, Node*> children;
    Node(int v=0) {
        value = v;
        count = 0;
    }
    Node* get(int v) {
        return children[v];
    }
    Node* insert(int v) {
        if(!children[v]) children[v] = new Node(v);
        children[v]->count++;
        return children[v];
    }
};
class Solution {
public:
    int equalPairs(vector<vector<int>>& grid) {
        auto root = new Node();
        for(auto row: grid) {
            auto node = root;
            for(auto x: row) node = node->insert(x);
        }
        int n = grid.size(), m = grid[0].size();
        int ret = 0;
        for(int j = 0;j<m;j++) {
            auto node = root;
            for(int i = 0;i<n && node;i++) node = node->get(grid[i][j]);
            if(node) ret += node->count;
        }
        return ret;
    }
};