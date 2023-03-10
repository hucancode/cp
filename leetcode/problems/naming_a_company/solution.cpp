class Solution {
public:
    long long distinctNames(vector<string>& ideas) {
        long long ret = 0;
        vector<set<string>> group(26);
        for(auto& x: ideas) {
            char i = x[0]-'a';
            auto suffix = x.substr(1);
            group[i].insert(suffix);
        }
        sort(group.begin(), group.end(), [](const set<string>& a, const set<string>& b) {
            return a.size() < b.size();
        });
        for(int i = 0;i<26;i++) {
            if(group[i].empty()) continue;
            for(int j = i+1;j<26;j++) {
                int k = 0;
                for(auto& str: group[i]) {
                    k += group[j].find(str) != group[j].end();
                }
                ret += 2*(group[i].size() - k)*(group[j].size() - k);
            }
        }
        return ret;
    }
};