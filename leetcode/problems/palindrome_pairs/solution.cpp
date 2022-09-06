class Solution {
public:
    bool palindrome(string& s) {
        int i = 0;
        int j = s.size() - 1;
        while(i < j) {
            if(s[i] != s[j]) {
                return false;
            }
            i++;
            j--;
        }
        return true;
    }
    vector<vector<int>> palindromePairs(vector<string>& words) {
        vector<vector<int>> ret;
        ret.reserve(1000000);
        int n = words.size();
        unordered_map<string, int> dict;
        for(int i = 0;i<n;i++) {
            string v = words[i];
            reverse(v.begin(), v.end());
            dict[v] = i;
        }
        for(int i = 0;i<n;i++) {
            int m = words[i].size();
            if(m == 0) {
                continue;
            }
            for(int k = 0;k<=m;k++) {
                auto left = words[i].substr(0, k);
                auto right = words[i].substr(k);
                auto it = dict.find(left);
                if(it != dict.end() && it->second != i && palindrome(right)) {
                    ret.push_back({i, it->second});
                }
                if(k == 0) {
                    continue;
                }
                it = dict.find(right);
                if(it != dict.end() && it->second != i && palindrome(left)) {
                    ret.push_back({it->second, i});
                }
            }
        }
        return ret;
    }
};