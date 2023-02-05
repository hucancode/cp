class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size();
        int m = p.size();
        vector<int> c1(26, 0);
        for(auto c: p) {
            c1[c-'a']++;
        }
        vector<vector<int>> c2(n+1, vector<int>(26, 0));
        for(int i = 1;i<=n;i++) {
            c2[i].assign(c2[i-1].begin(), c2[i-1].end());
            auto c = s[i-1];
            c2[i][c-'a']++;
        }
        vector<int> ret;
        int k = n - m;
        while(k>=0) {
            bool matched = true;
            for(int i = 0;i<26;i++) {
                int x = c2[k+m][i] - c2[k][i];
                if(x != c1[i]) {
                    matched = false;
                    break;
                }
            }
            if(matched) {
                ret.push_back(k);
            }
            k--;
        }
        return ret;
    }
};