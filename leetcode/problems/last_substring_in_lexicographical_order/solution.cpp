class Solution {
public:
    string lastSubstring(string s) {
        int k = 0;
        int j = 0;
        bool streak = true;
        for(int i = 1;i+j<s.size();) {
            if(s[i+j] > s[k+j]) {
                k = i;
                i++;
                j = 0;
                streak = true;
            } else if(s[i+j] == s[k+j] && !streak) {
                j++;
            } else {
                j = 0;
                i++;
            }
            if(s[i] != s[k]) {
                streak = false;
            }
        }
        return s.substr(k);
    }
};