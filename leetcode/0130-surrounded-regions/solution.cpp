class Solution {
public:
    void solve(vector<vector<char>>& board) {
        int n = board.size();
        int m = board[0].size();
        vector<vector<bool>> vis(n, vector<bool>(m, false));
        for(int i=0;i<n;i++) {  
            for(int j=0;j<m;j++) {
                if(vis[i][j] || board[i][j] == 'X') {
                    continue;
                }
                bool surrounded = true;
                vector<int> buffer;
                vector<int> q = {i,j};
                while(!q.empty()) {
                    int y = q.back();q.pop_back();
                    int x = q.back();q.pop_back();
                    if(x < 0 || x >= n || y < 0 || y >= m) {
                        surrounded = false;
                        continue;
                    }
                    if(vis[x][y] || board[x][y] == 'X') {
                        continue;
                    }
                    vis[x][y] = true;
                    buffer.push_back(x);
                    buffer.push_back(y);
                    q.push_back(x+1);q.push_back(y);
                    q.push_back(x-1);q.push_back(y);
                    q.push_back(x);q.push_back(y+1);
                    q.push_back(x);q.push_back(y-1);
                }
                while(surrounded && !buffer.empty()) {
                    int y = buffer.back();buffer.pop_back();
                    int x = buffer.back();buffer.pop_back();
                    board[x][y] = 'X';
                }
            }
        }
    }
};
