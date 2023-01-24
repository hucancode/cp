class Solution {
public:
    pair<int,int> idToXY(int u, int n) {
        int ux, uy;
        uy = u/n;
        ux = u%n;
        if(uy%2==1) {
            ux = n-1-ux;
        }
        uy = n - 1 - uy;
        return make_pair(ux, uy);
    }
    int snakesAndLadders(vector<vector<int>>& board) {
        const int INF = 1e9;
        int n = board.size();
        int n2 = n*n;
        queue<pair<int,int>> q;
        vector<int> d(n2+1, INF);
        q.emplace(0, 0);
        while(!q.empty()) {
            int u, du;
            tie(u,du) = q.front();
            q.pop();
            if(du >= d[u]) {
                continue;
            }
            d[u] = du;
            for(auto v = u+1;v < min(u+7, n2);v++) {
                int vx, vy;
                tie(vx, vy) = idToXY(v, n);
                if(board[vy][vx] != -1) {
                    q.emplace(board[vy][vx]-1, du+1);
                } else {
                    q.emplace(v, du+1);
                }
                
            }
        }
        return (d[n2-1] < INF)?d[n2-1]:-1;
    }
};