class Solution {
public:
    int longestSubsequence(vector<int>& arr, int difference) {
        int ret = 0;
        map<int, int> f;
        for(int x: arr) {
            f[x] = max(f[x], f[x-difference]+1);
            ret = max(ret, f[x]);
        }
        return ret;
    }
};