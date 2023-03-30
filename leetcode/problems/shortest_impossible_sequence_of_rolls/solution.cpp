class Solution {
public:
    int shortestSequence(vector<int>& rolls, int k) {
        vector<bool> vis(k+1, false);
        int count = 0;
        int ret = 1;
        for(auto x: rolls) {
            if(!vis[x]) {
                count++;
                vis[x] = true;
                if(count == k) {
                    fill(vis.begin(), vis.end(), false);
                    count = 0;
                    ret++;
                }
            }
        }
        return ret;
    }
};