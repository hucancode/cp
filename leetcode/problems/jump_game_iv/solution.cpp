class Solution {
public:
    int minJumps(vector<int>& arr) {
        int n = arr.size();
        map<int, vector<int>> mp;
        for(int i = 0;i<n;i++) {
            mp[arr[i]].push_back(i);
        }
        if(mp.size() == n) {
            return n-1;
        }
        int ret = 0;
        vector<bool> vis(n, false);
        queue<int> q;
        q.push(0);
        vis[0] = true;
        while(!q.empty()) {
            int m = q.size();
            while(m--) {
                auto u = q.front();
                q.pop();
                if(u == n-1) {
                    return ret;
                }
                auto& adj = mp[arr[u]];
                for(auto& v: adj) {
                    if(vis[v]) continue;
                    q.push(v);
                    vis[v] = true;
                }
                adj.clear();
                if(u < n-1 && !vis[u+1]) {
                    q.push(u+1);
                }
                if(u > 0 && !vis[u-1]) {
                    q.push(u-1);
                }
            }
            ret++;
        }
        return ret;
    }
};