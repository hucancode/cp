class Solution {
public:
    int maximalNetworkRank(int n, vector<vector<int>>& roads) {
        vector<vector<bool>> connected(n, vector<bool>(n, false));
        vector<int> degree(n, 0);
        int m = roads.size();
        for(int i = 0;i<m;i++) {
            int a = roads[i][0];
            int b = roads[i][1];
            connected[a][b] = connected[b][a] = true;
            degree[a]++;
            degree[b]++;
        }
        int ret = 0;
        for(int i = 0;i<n;i++) {
            for(int j = i+1;j<n;j++) {
                ret = max(ret, degree[i] + degree[j] - (connected[i][j]?1:0));
            }
        }
        return ret;
    }
};