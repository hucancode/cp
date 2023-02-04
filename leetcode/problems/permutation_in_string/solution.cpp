class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        int n = s2.size();
        int m = s1.size();
        vector<int> c1(26, 0);
        for(auto c: s1) {
            c1[c-'a']++;
        }
        vector<vector<int>> c2(n+1, vector<int>(26, 0));
        for(int i = 1;i<=n;i++) {
            c2[i].assign(c2[i-1].begin(), c2[i-1].end());
            auto c = s2[i-1];
            c2[i][c-'a']++;
        }
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
                return true;
            }
            k--;
        }
        return false;
    }
};