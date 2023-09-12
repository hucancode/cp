class Solution {
public:
    int minDeletions(string s) {
        int n = 26;
        vector<int> freq(n, 0);
        for(auto c: s) {
            freq[c-'a']++;
        }
        sort(freq.rbegin(), freq.rend());
        int top = INT_MAX;
        int ret = 0;
        for(auto x: freq) {
            int target = max(0, top -1);
            ret += max(0, x - target);
            top = min(x, target);
        }
        return ret;
    }
};