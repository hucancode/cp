class Solution {
public:
    string convert(string s, int numRows) {
        if(numRows == 1) {
            return s;
        }
        int n = s.size();
        int numCols = (n/(numRows*2-1)+1)*numRows;
        vector<vector<char>> mat(numRows, vector<char>(numCols, '_'));
        int x = 0;
        int y = 0;
        bool goDown = true;
        for(auto c: s) {
            mat[y][x] = c;
            if(goDown) {
                if(y == numRows - 1) {
                    x++;
                    y--;
                    goDown = false;
                } else {
                    y++;
                }
            } else {
                if(y == 0) {
                    y++;
                    goDown = true;
                } else {
                    x++;
                    y--;
                }
            }
        }
        string ret = "";
        for(auto row: mat) {
            for(auto c: row) {
                if(c == '_') continue;
                ret += c;
            }
        }
        return ret;
    }
};