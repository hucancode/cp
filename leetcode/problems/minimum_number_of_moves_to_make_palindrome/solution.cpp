class Solution {
public:
    int minMovesToMakePalindrome(string s) {
        int ret = 0;
        while(s.size() > 1) {
            char x = *s.rbegin();
            s.pop_back();
            int i = 0;
            while(i<s.size() && s[i] != x) {
                i++;
            }
            if(i == s.size()) {
                ret += s.size()/2;
                continue;
            }
            ret += i;
            s.erase(s.begin()+i);
        }
        return ret;
    }
};