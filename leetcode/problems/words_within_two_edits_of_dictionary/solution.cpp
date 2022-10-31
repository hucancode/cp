class Solution {
public:
    int countEdit(string& a, string& b) {
        int ret = 0;
        for(int i = 0;i<a.size();i++) {
            if(a[i] != b[i]) {
                ret++;
            }
            if(ret >2) {
                break;
            }
        }
        return ret;
    }
    vector<string> twoEditWords(vector<string>& queries, vector<string>& dictionary) {
        vector<string> ret;
        for(auto q: queries) {
            for(auto word: dictionary) {
                if(countEdit(q, word) <= 2) {
                    ret.push_back(q);
                    break;
                }
            }
        }
        return ret;
    }
};