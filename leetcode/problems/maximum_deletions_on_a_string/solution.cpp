class Solution {
public:
    bool repeated(string& s, int i, int j) {
        int k = j;
        while(i < k) {
            if(s[i++] != s[j++]) {
                return false;
            }
        }
        return true;
    }
    int deleteString(string s) {
        int n = s.size();
        vector<int> f(n+1, 1);
        f[n] = 0;
        for(int i = n-1;i>=0;i--) {
            int k = 1;
            while(i+k*2 <= n) {
                auto j = i+k;
                auto next = f[j] + 1;
                if(next > f[i] && repeated(s,i,j)) {
                    f[i] = next;
                }
                k++;
            }
        }
        return f[0];
    }
};