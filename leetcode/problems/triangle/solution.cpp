class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        const int INF = 10000000;
        int h = triangle.size();
        vector<vector<int>> f(h);
        f[0].resize(3, INF);
        f[0][1] = triangle[0][0];
        for(int i = 1;i<h;i++) {
            int n = triangle[i].size();
            f[i].resize(n+2, INF);
            for(int j = 1;j<=n;j++) {
                f[i][j] = min(f[i-1][j], f[i-1][j-1]) + triangle[i][j-1];
            }
        }
        return *min_element(f[h-1].begin(), f[h-1].end());
    }
};