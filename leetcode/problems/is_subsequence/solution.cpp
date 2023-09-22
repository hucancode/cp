class Solution {
public:
    bool isSubsequence(string s, string t) {
        int n = t.size(), m = s.size();
        int i = 0, j = 0;
        while(j < m && i < n) {
            if(t[i] == s[j]) j++;
            i++;
        }
        return j >= m;
    }
};