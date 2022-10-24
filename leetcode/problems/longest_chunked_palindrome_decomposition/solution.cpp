class Solution {
public:
    inline bool same(string::iterator it1, string::iterator it2, int len) {
        while(len--) {
            if(*it1 != *it2) {
                return false;
                break;
            }
            it1++;
            it2++;
        }
        return true;
    }
    int longestDecomposition(string text) {
        int n = text.size();
        vector<bool> f(n, false);
        f[0] = true;
        int count = 0;
        for(int i = 1;i<=n/2;i++) {
            for(int j = i-1;j>=0;j--) {
                int ri = n - i;
                auto len = i - j;
                if(same(text.begin()+j, text.begin()+ri, len)) {
                    f[i] = f[j];
                    if(f[i]) {
                        //cout<<"hit "<<text.substr(j, len)<<endl;
                        count++;
                        break;
                    }
                }
            }
        }
        int ret = count*2;
        if(n%2 != 0 || !f[n/2]) {
            ret++;
        }
        return ret;
    }
};