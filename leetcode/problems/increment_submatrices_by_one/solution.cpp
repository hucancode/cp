class Solution {
public:
    vector<vector<int>> rangeAddQueries(int n, vector<vector<int>>& queries) {
        vector<vector<int>> ret(n, vector<int>(n, 0));
        vector<vector<int>> delta(n, vector<int>(n, 0));
        for(auto& q: queries) {
            int x0 = q[0], y0 = q[1], xn = q[2], yn = q[3];
            for(int y = y0;y<=yn;y++) {
                delta[x0][y]++;
                if(xn+1 >= n) continue;
                delta[xn+1][y]--;
            }
        }
        for(int x = 0;x<n;x++) {
            for(int y = 0;y<n;y++) {
                auto prev = (x == 0?0:ret[x-1][y]);
                ret[x][y] = prev + delta[x][y];
            }
        }
        return ret;
    }
};