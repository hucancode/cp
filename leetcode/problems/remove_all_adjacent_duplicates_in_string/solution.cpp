class Solution {
public:
    string removeDuplicates(string s) {
        string ret;
        for(char c: s) {
            if(!ret.empty() && *ret.rbegin() == c) {
                ret.pop_back();
                continue;
            }
            ret.push_back(c);
        }
        return ret;
    }
};