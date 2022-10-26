class Solution {
public:
    int cost(string::iterator a, string::iterator b) {
        int ret = 0;
        while(a < b) {
            if(*a != *b) {
                ret++;
            }
            a++;
            b--;
        }
        return ret;
    }
    int palindromePartition(string s, int k) {
        int n = s.size();
        vector<vector<int>> f(n+1, vector<int>(k+1, n));
        // f[i][j] = optimal cost for j cuts with last piece end at position i
        // the answer is f[n][k]
        f[0][0] = 0;
        for(int i = 1;i<=n;i++) {
            for(int x = 0;x<i;x++) {
                int c = cost(s.begin()+x, s.begin()+i-1);
                for(int j = 1;j<=k;j++) {
                    f[i][j] = min(f[i][j], f[x][j-1] + c);
                }
            }
        }
        return f[n][k];
    }
};