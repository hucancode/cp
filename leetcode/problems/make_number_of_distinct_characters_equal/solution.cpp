class Solution {
public:
    bool isItPossible(string word1, string word2) {
        const int n = 'z';
        vector<int> w1(n+1,0);
        vector<int> w2(n+1,0);
        for(auto c: word1) {
            w1[c]++;
        }
        for(auto c: word2) {
            w2[c]++;
        }
        int c1 = 0, c2 = 0;
        for(int i = 0;i<=n;i++) {
            if(w1[i]>0) c1++;
            if(w2[i]>0) c2++;
        }
        for(int i = 0;i<=n;i++) {
            for(int j = 0;j<=n;j++) {
                if(w1[i] == 0 || w2[j]==0) {
                    continue;
                }
                if(i == j) {
                    if(c1 == c2) {
                        return true;
                    }
                    continue;
                }
                int k1 = c1;
                int k2 = c2;
                if(w1[i] - 1 == 0) {
                    k1--;
                }
                if(w1[j] + 1 == 1) {
                    k1++;
                }
                if(w2[j] - 1 == 0) {
                    k2--;
                }
                if(w2[i] + 1 == 1) {
                    k2++;
                }
                if(k1 == k2) {
                    return true;
                }
            }
        }
        return false;
    }
};