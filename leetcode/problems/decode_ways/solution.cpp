class Solution {
public:
    bool valid(string s) {
        if(s[0] < '1' || s[0] > '9') {
            return false;
        }
        if(s.size() == 1) {
            return true;
        }
        if(s[0] == '1') {
            return s[1] >= '0' && s[1] <= '9';
        }
        if(s[0] == '2') {
            return s[1] >= '0' && s[1] <= '6';
        }
        return false;
    }
    int numDecodings(string s) {
        int n = s.size();
        if(n == 1) {
            return valid(s.substr(0, 1));
        }
        vector<int> f(n, 0);
        f[0] = valid(s.substr(0, 1));
        f[1] = valid(s.substr(1, 1))*f[0] + valid(s.substr(0, 2));
        for(int i = 2;i<n;i++) {
            f[i] += valid(s.substr(i, 1))*f[i-1];
            f[i] += valid(s.substr(i-1, 2))*f[i-2];
        }
        return f[n-1];
    }
};