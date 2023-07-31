class Solution {
public:
    int strangePrinter(string s) {
        int n = s.size();
        vector<vector<int>> f(n, vector<int>(n, n));
        for(int i = 0;i<n;i++) {
            f[i][i] = 1;
        }
        for(int len = 1;len<=n;len++) {
            for(int head = 0;head<n;head++) {
                int tail = head+len;
                if(tail >= n) {
                    continue;
                }
                char c = s[tail];
                int first_diff = head;
                while(first_diff < tail && s[first_diff] == s[tail]) first_diff++;
                f[head][tail] = f[first_diff][tail];
                for(int k = first_diff;k<tail;k++) {
                    int score = f[first_diff][k] + f[k+1][tail];
                    f[head][tail] = min(f[head][tail], score);
                }
            }
        }
        return f[0][n-1];
    }
};