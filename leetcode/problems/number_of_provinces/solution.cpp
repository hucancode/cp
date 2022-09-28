class Solution {
public:
    int findCircleNum(vector<vector<int>>& isConnected) {
        int n = isConnected.size();
        int ret = 0;
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
                    if(isConnected[u][v] && binary_search(vis.begin(), vis.end(), v)) {
                        q.push(v);
                    }
                }
            }
            ret++;
        }
        return ret;
    }
};