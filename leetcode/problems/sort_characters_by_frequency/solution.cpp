class Solution {
public:
    string frequencySort(string s) {
        map<char,int> mp;
        for(auto c: s) {
            mp[c]++;
        }
        map<int,vector<char>> freq;
        for(auto item: mp) {
            freq[item.second].push_back(item.first);
        }
        string ret;
        for(auto it = freq.rbegin(); it != freq.rend();it++) {
            int n = it->first;
            for(auto c: it->second) {
                for(int i = 0;i<n;i++) {
                    ret.push_back(c);
                }
            }
        }
        return ret;
    }
};