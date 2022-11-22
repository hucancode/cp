class Solution {
public:
    int largestVariance(string& s, char a, char b) {
        // consider potential pair (a,b)
        // find optimal substring where count[a] - count[b] maximum
        int n = s.size();
        int ret = 0;
        vector<int> f(n, 0); // optimal score for substring with or without b in it
        vector<int> g(n, -1e9); // optimal score for substring with b in it
        if(s[0] == a) {
            f[0] = 1;
        } else if(s[0] == b) {
            g[0] = -1;
        }
        for(int i = 1;i<n;i++) {
            if(s[i] == a) {
                f[i] = max(f[i-1], g[i-1])+1;
                g[i] = g[i-1]+1;
            } else if(s[i] == b) {
                f[i] = max(0, f[i-1]-1);
                g[i] = max(f[i-1],g[i-1])-1;
            } else {
                f[i] = f[i-1];
                g[i] = g[i-1];
            }
            ret = max(ret, g[i]);
        }
        return ret;
        cout<<"g: ";
        for(auto x: g) {
            cout<<x<<" ";
        }
        cout<<endl;
        cout<<"largest variance, check "<<a<<"-"<<b<<" return "<<ret<<endl;
        return ret;
    }
    int largestVariance(string s) {
        set<char> pool;
        for(auto c: s) {
            pool.insert(c);
        }
        int n = s.size();
        int ret = 0;
        for(char a: pool) {
            for(char b: pool) {
                if(a == b) {
                    continue;
                }
                ret = max(ret, largestVariance(s, a, b));
            }
        }
        return ret;
    }
};