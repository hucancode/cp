class Solution {
public:
    int calculateScore(string& word, vector<int>& state, vector<int>& score) {
        int ret = 0;
        for(char c: word) {
            auto key = c-'a';
            state[key]--;
            ret += score[key];
            if(state[key] < 0) {
                return -1;
            }
        }
        return ret;
    }
    int maxScoreWords(vector<string>& words, vector<char>& letters, vector<int>& score) {
        typedef vector<int> vi;
        typedef vector<vi> vvi;
        typedef vector<vvi> v3i;
        int ret = 0;
        int n = words.size();
        int k = (1<<n);
        vvi f(k, vi(26,0)); // state
        vi g(k, -1); // score
        int m = letters.size();
        for(int i = 0;i<m;i++) {
            int j = letters[i]-'a';
            f[0][j]++;
        }
        g[0] = 0;
        for(int mask = 0;mask<k;mask++) {
            if(g[mask] == -1) {
                continue;
            }
            ret = max(ret, g[mask]);
            for(int j = 0;j<n;j++) {
                if(mask & 1<<j) {
                    continue;
                }
                int nextMask = mask | 1<<j;
                f[nextMask] = f[mask];
                int gain = calculateScore(words[j], f[nextMask], score);
                if(gain != -1) {
                    g[nextMask] = g[mask] + gain;
                }
            }
        }
        return ret;
    }
};