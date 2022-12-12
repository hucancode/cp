typedef tuple<int,int,int> state;
class Solution {
    vector<vector<bool>> vis;
    priority_queue<state, vector<state>, greater<state>> q;
    int score;
public:
    void enqueue(vector<vector<int>>& grid, int i, int j) {
        int n = grid.size();
        int m = grid[0].size();
        if(i < 0 || i>=n || j<0 || j>=m) {
            return;
        }
        q.emplace(grid[i][j],i,j);
    }
    int bfs(vector<vector<int>>& grid, int x) {
        //cout<<"bfs "<<x<<endl;
        int n = grid.size();
        int m = grid[0].size();
        while(!q.empty()) {
            int v,i,j;
            tie(v,i,j) = q.top();
            if(v >= x) {
                break;
            }
            q.pop();
            if(vis[i][j]) {
                continue;
            }
            //cout<<"check "<<i<<"-"<<j<<"("<<v<<")"<<endl;
            vis[i][j] = true;
            score++;
            enqueue(grid,i-1,j);
            enqueue(grid,i+1,j);
            enqueue(grid,i,j-1);
            enqueue(grid,i,j+1);
        }
        return score;
    }
    vector<int> solve(vector<vector<int>>& grid, vector<pair<int,int>> queries) {
        sort(queries.begin(), queries.end());
        int n = grid.size();
        int m = grid[0].size();
        vector<int> ret(queries.size());
        score = 0;
        enqueue(grid,0,0);
        vis.resize(n, vector<bool>(m, false));
        for(auto item: queries) {
            ret[item.second] = bfs(grid, item.first);
        }
        return ret;
    }
    vector<int> maxPoints(vector<vector<int>>& grid, vector<int>& queries) {
        int n = queries.size();
        vector<pair<int,int>> sortedQueries(n);
        for(int i = 0;i<n;i++) {
            sortedQueries[i] = make_pair(queries[i], i);
        }
        return move(solve(grid, sortedQueries));
    }
};