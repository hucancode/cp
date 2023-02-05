class Solution {
public:
    vector<int> vowelStrings(vector<string>& words, vector<vector<int>>& queries) {
        int n = words.size();
        const string vowels = "aeiou";
        vector<int> f(n+1, 0);
        for(int i=1;i<=n;i++) {
            auto& word = words[i-1];
            bool matched0 = false;
            bool matchedn = false;
            for(auto c: vowels) matched0 |= word[0] == c;
            for(auto c: vowels) matchedn |= word[word.size()-1] == c;
            f[i] = f[i-1] + (matched0 && matchedn);
        }
        int m = queries.size();
        vector<int> ans(m);
        for(int i = 0;i<m;i++) {
            int l = queries[i][0];
            int r = queries[i][1];
            ans[i] = f[r+1] - f[l];
        }
        return ans;
    }
};