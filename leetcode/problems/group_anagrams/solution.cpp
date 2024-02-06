class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        map<vector<int>, vector<string>> f;
        for(auto s: strs) {
            vector<int> k(26);
            for(auto c: s) k[c-'a']++;
            f[k].push_back(s);
        }
        vector<vector<string>> ret;
        for(auto kv: f) ret.push_back(kv.second);
        return ret;
    }
};