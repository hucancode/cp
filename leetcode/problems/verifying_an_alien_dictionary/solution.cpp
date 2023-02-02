class Solution {
public:
    bool isAlienSorted(vector<string>& words, string order) {
        map<char,int> mp;
        for(int i = 0;i<order.size();i++) {
            mp[order[i]] = i;
        }
        for(auto& word: words) {
            for(auto& c: word) {
                c = mp[c];
            }
        }
        for(int i = 1;i<words.size();i++) {
            if(words[i] < words[i-1]) {
                return false;
            }
        }
        return true;
    }
};