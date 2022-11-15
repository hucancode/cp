class Solution {
public:
    bool canReach(string s, int minJump, int maxJump) {
        int n = s.size();
        vector<bool> vis(n, false);
        queue<int> q;
        q.push(0);
        vis[0] = true;
        int l = 0;
        while(!q.empty()) {
            auto u = q.front();
            q.pop();
            if(u == n-1) {
                break;
            }
            int v0 = max(l, u+minJump);
            int vn = min(n-1,u+maxJump);
            l = max(l, vn);
            for(int v = v0;v<=vn;v++) {
                if(vis[v]) {
                    continue;
                }
                if(s[v] == '1') {
                    continue;
                }
                vis[v] = true;
                q.push(v);
            }
        }
        return vis[n-1];
    }
};