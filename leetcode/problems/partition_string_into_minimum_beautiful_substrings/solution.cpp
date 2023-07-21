class Solution {
public:
    int minimumBeautifulSubstrings(string s) {
        if(s[0] == '0') {
            return -1;
        }
        int n = 0;
        for(auto c: s) {
            n = n<<1 | (c=='0'?0:1);
        }
        //cout<<"n = "<<n<<endl;
        vector<int> f(n+1,1e9);
        set<int> power5 = {1,5,25,125,625,3125,15625};
        for(auto x: power5) {
            if(x<=n) f[x] = 1;
        }
        for(int mask = 2;mask<=n;mask++) {
            for(int i = 0;i<15;i++) {
                if((mask & (1<<i)) == 0) continue;
                int a = mask & ((1<<(i+1)) - 1);
                int b = mask>>(i+1);
                //cout<<"split mask "<<bitset<4>(mask)<<" at "<<i<<" into "<<bitset<4>(a)<<" and "<<bitset<4>(b)<<endl;
                f[mask] = min(f[mask], f[a] + f[b]);
            }
        }
        // for(int i = 0;i<=n;i++) {
        //     cout<<"f["<<i<<"]="<<f[i]<<endl;
        // }
        return (f[n] == 1e9)?-1:f[n];
    }
};