class Solution {
public:
    bool check(vector<vector<char>>& a, int x, int y, char v) {
        int ox = x/3*3;
        int oy = y/3*3;
        for(int i = 0;i<9;i++) {
            if(a[x][i] == v && y != i) {
                return false;
            }
            if(a[i][y] == v && x != i) {
                return false;
            }
        }
        for(int i = 0;i<3;i++) {
            for(int j = 0;j<3;j++) {
                int x1 = ox+i;
                int y1 = oy+j;
                if(x1 == x && y1 == y) {
                    continue;
                }
                if(a[x1][y1] == v) {
                    return false;
                }
            }
        }
        return true;
    }
    bool isValidSudoku(vector<vector<char>>& board) {
        for(int i = 0;i<9;i++) {
            for(int j = 0;j<9;j++) {
                if(board[i][j] == '.') {
                    continue;
                }
                if(!check(board, i, j, board[i][j])) {
                    //cout<<"invalid at "<<i<<"-"<<j<<endl;
                    return false;
                }
            }
        }
        return true;
    }
};