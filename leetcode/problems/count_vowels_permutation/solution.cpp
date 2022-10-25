class Solution {
public:
    int countVowelPermutation(int n) {
        const int INF = 1e9+7;
        const int A = 0, E = 1, I = 2, O = 3, U = 4;
        vector<vector<long long>> f(5, vector<long long>(n, 0));
        for(int i=A;i<=U;i++) {
            f[i][0] = 1;
        }
        for(int i=1; i<n; i++) {
            f[A][i] = (f[E][i-1] + f[U][i-1] + f[I][i-1])%INF;
            f[E][i] = (f[A][i-1] + f[I][i-1])%INF;
            f[I][i] = (f[E][i-1] + f[O][i-1])%INF;
            f[O][i] = f[I][i-1];
            f[U][i] = (f[O][i-1] + f[I][i-1])%INF;
        }
        return (f[A][n-1] + f[E][n-1] + f[I][n-1] + f[O][n-1] + f[U][n-1])%INF;
    }
};