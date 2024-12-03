class Solution {
public:
    string addSpaces(string s, vector<int>& spaces) {
        string ret;
        int n = s.size(), m = spaces.size();
        ret.reserve(n+m);
        int i = 0;
        for(int j: spaces) {
            ret.append(s, i, j-i);
            ret.push_back(' ');
            i = j;
        }
        ret.append(s, i, n);
        return ret;
    }
};
