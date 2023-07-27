class Solution {
public:
    string sortVowels(string s) {
        vector<int> idx;
        vector<char> v;
        string vowels = "aeiouAEIOU";
        for(int i = 0;i<s.size();i++) {
            if(vowels.find(s[i]) != string::npos) {
                idx.push_back(i);
                v.push_back(s[i]);
            }
        }
        sort(v.begin(), v.end());
        for(int i = 0;i<idx.size();i++) {
            int j = idx[i];
            s[j] = v[i];
        }
        return s;
    }
};