class Solution {
public:
    int minimumJumps(vector<int>& forbidden, int a, int b, int x) {
        int n = 7e3;//x+(a+b)*2;
        vector<vector<bool>> vis(n+1, vector<bool>(2, false));
        for(auto x: forbidden) {
            vis[x][0] = true;
            vis[x][1] = true;
        }
        vector<int> dist(n+1, -1);
        dist[0] = 0;
        queue<pair<int,bool>> q;
        q.emplace(0, false);
        while(!q.empty()) {
            int u;
            bool jumpedBack;
            tie(u, jumpedBack) = q.front();
            q.pop();
            if(vis[u][jumpedBack]) {
                continue;
            }
            vis[u][jumpedBack] = true;
            int v1 = u+a;
            int v2 = u-b;
            //cout<<"check "<<u<<endl;
            if(v1 <= n) {
                if(dist[v1] == -1 || dist[v1] >= dist[u]+1) {
                    dist[v1] = dist[u]+1;
                    q.emplace(v1, false);
                }
            }
            if(v2 > 0 && !jumpedBack) {
                if(dist[v2] == -1 || dist[v2] > dist[u]+1) {
                    dist[v2] = dist[u]+1;
                    q.emplace(v2, true);
                }
            }
            if(dist[x] != -1) {
                break;
            }
        }
        return dist[x];
    }
};