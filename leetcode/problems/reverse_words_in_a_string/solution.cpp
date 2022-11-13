class Solution {
public:
    string reverseWords(string s) {
        vector<string> a;
        istringstream ss(s);
        string str;
        while(ss>>str) {
            a.push_back(move(str));
        }
        string ret;
        for(auto it = a.rbegin(); it != a.rend();it++) {
            ret += *it;
            ret += " ";
        }
        ret.resize(ret.size()-1);
        return ret;
    }
};