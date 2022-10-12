class Solution {
public:
    int longestPalindromeSubseq(string s) {
        string a(s.begin(), s.end());
        string b(s.rbegin(), s.rend());
        int m = a.size();
        int n = b.size();
        vector<vector<int>> f(m+1, vector<int>(n+1, 0));
        int ret = 1;
        for(int i = 1; i <= m; i++) {
            for(int j = 1; j <= n; j++) {
                // calculate lcs of s (a) and reverve of s (b)
                // only accept the pair where len(a) + len(reverse(b)) <= len(s)
                if(i + j > n) {
                    continue;
                }
                bool matched = a[i-1] == b[j-1];
                if(matched) {
                    int prev = f[i-1][j-1];
                    f[i][j] = prev + 1;
                } else {
                    int prev_a = f[i-1][j];
                    int prev_b = f[i][j-1];
                    f[i][j] = max(prev_a, prev_b);
                }
                // update answer
                if(i + j < n) {
                    ret = max(ret, f[i][j]*2 + 1);
                } else if(i + j == n) {
                    ret = max(ret, f[i][j]*2);
                }
            }
        }
        return ret;
    }
};