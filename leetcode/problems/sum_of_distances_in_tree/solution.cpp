class Solution {
public:
    vector<int> parent;
    vector<int> childCount;
    vector<vector<int>> children;
    vector<vector<int>> adj;
    void buildTree(vector<vector<int>>& adj) {
        vector<bool> vis(parent.size(), false);
        queue<int> q;
        q.push(0);
        vis[0] = true;
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            for(auto v: adj[u]) {
                if(vis[v]) {
                    continue;
                }
                vis[v] = true;
                q.push(v);
                parent[v] = u;
                children[u].push_back(v);
            }
        }
    }
    void buildDistBottomUp(int root, vector<int>& score) {
        int p = parent[root];
        if(p != -1) {
            int n = parent.size();
            int otherSideScore = score[p] - score[root] - childCount[root] - 1;
            int otherCount = n - childCount[root] - 1;
            //cout<<"build "<<root<<" current score = "<<dist[root]<<", other side score = "<<otherSideScore<<" other count = "<<otherCount<<endl;
            score[root] += otherSideScore + otherCount;
        }
        for(auto child: children[root]) {
            buildDistBottomUp(child, score);
        }
    }
    void buildDistTopDown(int root, vector<int>& score) {
        for(auto child: children[root]) {
            buildDistTopDown(child, score);
            score[root] += score[child]+childCount[child]+1;
        }
    }
    void countChildren(int root) {
        for(auto child: children[root]) {
            countChildren(child);
            childCount[root] += childCount[child]+1;
        }
    }
    vector<int> sumOfDistancesInTree(int n, vector<vector<int>>& edges) {
        parent.resize(n,-1);
        childCount.resize(n, 0);
        children.resize(n);
        adj.resize(n);
        for(auto e: edges) {
            adj[e[0]].push_back(e[1]);
            adj[e[1]].push_back(e[0]);
        }
        vector<int> ret(n, 0);
        buildTree(adj);
        countChildren(0);
        buildDistTopDown(0, ret);
        buildDistBottomUp(0, ret);
        return ret;
    }
};