class Solution {
public:
    string removeDuplicates(string s) {
        bool removed = false;
        for(int i = s.size()-1;i>0;i--) {
            if(s[i] == s[i-1]) {
                s.erase(s.begin()+i-1, s.begin()+i+1);
            }
        }
        return s;
    }
};