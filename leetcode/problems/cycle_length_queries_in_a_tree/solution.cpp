class Solution {
public:
    int dfs(long long root, int p, int q) {
        int step = 1;
        int k = p;
        while(k>root) {
            k = k>>1;
            step++;
        }
        if(k != root) {
            return 9999;
        }
        k = q;
        while(k>root) {
            k = k>>1;
            step++;
        }
        if(k != root) {
            return 9999;
        }
        auto left = dfs(root*2, p, q);
        auto right = dfs(root*2+1, p, q);
        return min(step, min(left, right));
    }
    vector<int> cycleLengthQueries(int n, vector<vector<int>>& queries) {
        vector<int> ret;
        for(auto q: queries) {
            ret.push_back(dfs(1, q[0],q[1]));
        }
        return ret;
    }
};