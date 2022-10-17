class Solution {
public:
    bool checkIfPangram(string sentence) {
        vector<bool> f(26, false);
        for(auto x: sentence) {
            f[x-'a'] = true;
        }
        for(auto x: f) {
            if(!x) {
                return false;
            }
        }
        return true;
    }
};