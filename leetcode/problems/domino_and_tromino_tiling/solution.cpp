class Solution {
public:
    int numTilings(int n) {
        if(n == 1) {
            return 1;
        }
        if(n == 2) {
            return 2;
        }
        typedef long long ll;
        const int INF = 1e9+7;
        vector<ll> f(n,0); // f[i] = number of way to assemble full tile
        vector<ll> fu(n,0); // fu[i] = number of way to assemble semi full tile at i, with hole on top
        vector<ll> fd(n,0); // fd[i] = number of way to assemble semi full tile at i, with hole at bottom
        f[0] = 1;
        f[1] = 2;
        fu[1] = 1;
        fd[1] = 1;
        for(int i = 2;i<n;i++) {
            fu[i] = f[i-2] + fd[i-1];
            fd[i] = f[i-2] + fu[i-1];
            f[i] = f[i-1] + f[i-2] + fu[i-1] + fd[i-1];
            f[i] %= INF;
        }
        return f[n-1];
    }
};