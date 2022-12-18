class Solution {
public:
    bool similar(string& a, string& b) {
        vector<bool> ma(30, false);
        vector<bool> mb(30, false);
        for(auto c: a) {
            ma[c-'a'] = true;
        }
        for(auto c: b) {
            mb[c-'a'] = true;
        }
        for(int i = 0;i<30;i++) {
            if(ma[i] != mb[i]) {
                return false;
            }
        }
        return true;
    }
    int similarPairs(vector<string>& words) {
        int ret = 0;
        int n = words.size();
        for(int i = 0;i<n-1;i++) {
            for(int j = i+1;j<n;j++) {
                if(similar(words[i], words[j])) {
                    ret++;
                }
            }
        }
        return ret;
    }
};