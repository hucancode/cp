class Solution {
public:
    bool isAnagram(string s, string t) {
        vector<int> a(26, 0);
        vector<int> b(26, 0);
        for(char c: s) a[c-'a']++;
        for(char c: t) b[c-'a']++;
        return equal(a.begin(), a.end(), b.begin());
    }
};