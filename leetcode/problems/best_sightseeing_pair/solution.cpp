class Solution {
public:
    int maxScoreSightseeingPair(vector<int>& values) {
        int n = values.size();
        vector<int> f(n, 0);
        int ret = 0;
        f[0] = values[0];
        for(int i = 1;i<n;i++) {
            f[i] = max(f[i-1] - 1, values[i]);
        }
        for(int i = 1;i<n;i++) {
            ret = max(ret, f[i-1] + values[i] - 1);
        }
        return ret;
    }
};