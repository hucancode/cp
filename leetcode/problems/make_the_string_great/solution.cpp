class Solution {
public:
    string makeGood(string s) {
        bool changed = false;
        for(int i = s.size()-1;i>0;i--) {
            if(tolower(s[i]) == tolower(s[i-1]) && s[i] != s[i-1]) {
                s.erase(s.begin()+i-1, s.begin()+i+1);
                i--;
                changed = true;
            }
        }
        if(!changed) {
            return s;
        }
        return makeGood(s);
    }
};