class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> ret;
        int n = 1;
        while(n <= numRows) {
            vector<int> f(n, 0);
            for(int i = 0;i<n;i++) {
                if(i == 0 || i == n-1) {
                    f[i] = 1;
                    continue;
                }
                f[i] = ret[n-2][i-1] + ret[n-2][i];
            }
            ret.push_back(f);
            n++;
        }
        return ret;
    }
};