class Solution {
public:
    bool checkValidGrid(vector<vector<int>>& grid) {
        int n = grid.size();
        int m = n*n;
        if(grid[0][0] != 0) {
            return false;
        }
        int v = 1;
        int x = 0;
        int y = 0;
        while(v < m) {
            for(int i = 0;i<n;i++) {
                bool found = false;
                for(int j = 0;j<n;j++) {
                    if(grid[i][j] == v) {
                        if(abs(j-y) + abs(i-x) != 3) {
                            return false;
                        }
                        x = i;
                        y = j;
                        found = true;
                        break;
                    }
                }
                if(found) break;
            }
            v++;
        }
        return true;
    }
};