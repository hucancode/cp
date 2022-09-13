class Solution {
public:
    int minDistanceRecursive(string a, string b) {
        if(a.empty()) {
            return b.size();
        }
        if(b.empty()) {
            return a.size();
        }
        string ra = a.substr(1);
        string rb = b.substr(1);
        if(a[0] == b[0]) {
            return minDistanceRecursive(ra, rb);
        }
        return 1 + min(minDistanceRecursive(ra,rb), min(minDistanceRecursive(a, rb), minDistanceRecursive(ra, b)));
    }
    int minDistance(string a, string b) {
        if(a.empty()) {
            return b.size();
        }
        if(b.empty()) {
            return a.size();
        }
        int n = a.size();
        int m = b.size();
        vector<vector<int>> f(n+1, vector<int>(m+1));
        bool matched = false;
        f[0][0] = 0;
        for(int i = 1;i<=n;i++) {
            f[i][0] = i;
        }
        for(int j = 1;j<=m;j++) {
            f[0][j] = j;
        }
        for(int i = 1;i<=n;i++) {
            for(int j = 1;j<=m;j++) {
                if(a[i-1] == b[j-1]) {
                    f[i][j] = f[i-1][j-1];
                    continue;
                }
                f[i][j] = min(f[i-1][j-1], min(f[i-1][j], f[i][j-1])) + 1;
            }
        }
        return f[n][m];
    }
};