class Solution {
public:
    string reverseWords(string s) {
        stringstream ss(s);
        stringstream ret;
        string word;
        while(ss>>word) {
            reverse(word.begin(), word.end());
            ret<<word;
            if(!ss.eof()) {
                ret<<" ";
            }
        }
        return ret.str();
    }
};