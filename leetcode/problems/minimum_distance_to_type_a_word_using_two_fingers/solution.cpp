class Solution {
public:
    int distance(int a, int b) {
        int xa = a%6;
        int ya = a/6;
        int xb = b%6;
        int yb = b/6;
        return abs(xb-xa) + abs(yb-ya);
    }
    int minimumDistance(string word) {
        const int INF = 2e9;
        typedef vector<int> vi;
        typedef vector<vi> vvi;
        typedef vector<vvi> v3i;
        typedef vector<v3i> v4i;
        int n = word.size();
        // f[i][j][k] = left finger at char i, right finger at char j, about to write char k
        v3i f(26, vvi(26, vi(n+1, INF)));
        f[0][0][0] = 0;
        for(int i = 0;i<26;i++) {
            for(int j = 0;j<26;j++) {
                f[i][j][0] = 0;
            }
        }
        for(int k = 1;k<=n;k++) {
            for(int i = 0;i<26;i++) {
                for(int j = 0;j<26;j++) {
                    int c = word[k-1] - 'A';
                    int currentCost = f[i][j][k-1];
                    int costi = currentCost + distance(i, c);
                    int costj = currentCost + distance(j, c);
                    // use right finger
                    f[i][c][k] = min(f[i][c][k], costj);
                    // use left finger
                    f[c][j][k] = min(f[c][j][k], costi);
                }
            }
        }
        int ret = 2e9;
        for(int i = 0;i<26;i++) {
            for(int j = 0;j<26;j++) {
                ret = min(ret, f[i][j][n]);
            }
        }
        return ret;
    }
};