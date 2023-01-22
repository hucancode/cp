class Solution {
public:
    bool makeStringsEqual(string s, string target) {
        int n = s.size();
        // 00 => 01
        // 01 => 11
        // 11 => 10
        // 10 => 11
        int a = 0, b = 0;
        for(int i = 0;i<n;i++) {
            a += s[i] == '1';
            b += target[i] == '1';
        }
        if(b == 0 && a != 0) {
            return false;
        }
        if(a == 0 && b != 0) {
            return false;
        }
        return true;
    }
};