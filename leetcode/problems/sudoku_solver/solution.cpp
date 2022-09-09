class Solution {
public:
    bool check(vector<vector<char>>& a, int x, int y, char v) {
        int ox = x/3*3;
        int oy = y/3*3;
        for(int i = 0;i<9;i++) {
            if(a[x][i] == v || a[i][y] == v) {
                return false;
            }
        }
        for(int i = 0;i<3;i++) {
            for(int j = 0;j<3;j++) {
                if(a[i+ox][j+oy] == v) {
                    return false;
                }
            }
        }
        return true;
    }
    void solveSudoku(vector<vector<char>>& board) {
        stack<pair<pair<int, int>, char>> st;
        int i = 0;
        int j = 0;
        while(i < 9 && j < 9 && board[i][j] != '.') {
            j++;
            if(j == 9) {
                i++;
                j = 0;
            }
        }
        for(char c = '1'; c<='9';c++) {
            if(!check(board, i, j, c)) {
                continue;
            }
            auto pos = make_pair(i, j);
            auto candidate = make_pair(pos, c);
            st.push(candidate);
            //cout<<"push candidate "<<i<<"-"<<j<<", value "<<c<<endl;
        }
        while(!st.empty()) {
            auto candidate = st.top();
            st.pop();
            auto x = candidate.first.first;
            auto y = candidate.first.second;
            auto value = candidate.second;
            board[x][y] = value;
            if(value == '.') {
                //cout<<"restore "<<x<<"-"<<y<<endl;
                continue;
            }
            //cout<<"try "<<x<<"-"<<y<<", value "<<value<<endl;
            i = 0;
            j = 0;
            while(i < 9 && j < 9 && board[i][j] != '.') {
                j++;
                if(j >= 9) {
                    i++;
                    j = 0;
                }
            }
            if(i >= 9 || j >= 9) {
                break;
            }
            st.push(make_pair(make_pair(x, y), '.'));
            for(char c = '1'; c <= '9'; c++) {
                if(!check(board, i, j, c)) {
                    continue;
                }
                auto pos = make_pair(i, j);
                auto candidate = make_pair(pos, c);
                st.push(candidate);
                //cout<<"push candidate "<<i<<"-"<<j<<", value "<<c<<endl;
            }
        }
    }
};