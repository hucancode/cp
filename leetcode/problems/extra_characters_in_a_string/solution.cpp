class Solution {
public:
    int minExtraChar(string s, vector<string>& dictionary) {
        set<string> pool(dictionary.begin(), dictionary.end());
        int n = s.size();
        vector<int> f(n+1,n);
        f[0] = 0;
        for(int i=1;i<=n;i++) {
            f[i] = f[i-1]+1;
            for(int j=i;j>0;j--) {
                if(pool.find(s.substr(j-1, i-j+1)) == pool.end()) {
                    continue;
                }
                for(int k=j-1;k>=0;k--) {
                    f[i] = min(f[i], f[k] + (j-k-1));
                }
            }
        }
        int ret = n;
        for(int i=1;i<=n;i++) {
            ret = min(ret, n-i+f[i]);
        }
        return ret;
    }
};