class Solution {
public:
    int findJudge(int n, vector<vector<int>>& trust) {
        vector<int> f(n+1, 0); // f[i] = person i reputation
        vector<bool> g(n+1, true); // g[i] = person i trust nobody
        for(auto item: trust) {
            g[item[0]] = false;
            f[item[1]]++;
        }
        int idx = -1;
        for(int i = 1;i<=n;i++) {
            if(f[i] == n-1 && g[i])  {
                if(idx == -1) {
                    idx = i;
                } else {
                    return -1;
                }
            }
        }
        return idx;
    }
};