class Solution {
public:
    string smallestString(string s) {
        int i = 0;
        bool used = false;
        while(i < s.size()) {
            if(s[i] == 'a') {
                if(used) {
                    break;
                }
                i++;
                continue;
            }
            s[i]--;
            i++;
            used = true;
        }
        if(!used) {
            s[s.size()-1] = 'z';
        }
        return s;
    }
};