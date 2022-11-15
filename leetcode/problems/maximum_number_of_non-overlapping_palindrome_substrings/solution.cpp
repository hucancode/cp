class Solution {
public:
    bool palindrome(int i, int j, string& s) {
        while(i > j) {
            if(s[i] != s[j]) {
                return false;
            }
            i--;
            j++;
        }
        return true;
    }
    int maxPalindromes(string s, int k) {
        int n = s.size();
        if(k == 1) {
            return n;
        }
        int ret = 0;
        vector<int> f(n+1, 0);
        for(int i = k;i<=n;i++) {
            f[i] = max(f[i-1],f[i]);
            for(int j = i-k;j>=0;j--) {
                if(f[j] < f[i]) {
                    break;
                }
                if(palindrome(i-1,j,s)) {
                    f[i] = f[j]+1;
                    break;
                }
            }
            ret = max(ret,f[i]);
        }
        return ret;
    }
};