class Solution {
public:
    int shortestSequence(vector<int>& rolls, int k) {
        vector<bool> vis(k+1, false);
        int count = 0, ret = 1;
        for(auto x: rolls) {
            if(vis[x]) continue;
            vis[x] = true;
            if(++count != k) continue;
            fill(vis.begin(), vis.end(), false);
            count = 0;
            ret++;
        }
        return ret;
    }
};