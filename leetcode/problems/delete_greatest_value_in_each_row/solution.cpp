class Solution {
public:
    int deleteGreatestValue(vector<vector<int>>& grid) {
        int ret = 0;
        for(auto& row: grid) {
            sort(row.begin(), row.end());
        }
        int n = grid.size();
        int m = grid[0].size();
        for(auto j = m-1;j>=0;j--) {
            int k = 0;
            for(auto i=0;i<n;i++) {
                k = max(k,grid[i][j]);
            }
            //cout<<"k = "<<k<<endl;
            ret += k;
        }
        return ret;
    }
};