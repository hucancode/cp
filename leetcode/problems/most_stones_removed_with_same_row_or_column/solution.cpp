class Solution {
public:
    int removeStones(vector<vector<int>>& stones) {
        int n = stones.size();
        vector<vector<bool>> edges(n, vector<bool>(n, false));
        for(int i = 0;i<n-1;i++) {
            for(int j = i+1;j<n;j++) {
                if(stones[i][0] == stones[j][0] || 
                  stones[i][1] == stones[j][1]) {
                    edges[i][j] = true;
                    edges[j][i] = true;
                }
            }
        }
        int island = 0;
        vector<int> vis(n);
        for(int i = 0;i<n;i++) {
            vis[i] = i;
        }
        while(!vis.empty()) {
            queue<int> q;
            q.push(vis[0]);
            while(!q.empty()) {
                int u = q.front();
                auto it = lower_bound(vis.begin(), vis.end(), u);
                if(it != vis.end() && *it == u) {
                    vis.erase(it);
                }
                q.pop();
                for(int v = 0;v<n;v++) {
                    if(edges[u][v] && binary_search(vis.begin(), vis.end(), v)) {
                        q.push(v);
                    }
                }
            }
            island++;
        }
        return n - island;
    }
};