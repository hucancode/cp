class Solution {
public:
    int flip(int input, int x, int y, int n, int m) {
        int output = input;
        vector<pair<int, int>> offsets = {
            {-1, 0},
            {1, 0},
            {0, -1},
            {0, 1},
            {0, 0}
        };
        for(auto offset: offsets) {
            int xi = x+offset.first;
            int yj = y+offset.second;
            if(xi < 0 || xi >= n || yj < 0 || yj >= m) {
                continue;
            }
            int k = xi*m+yj;
            output ^= 1<<k;
        }
        return output;
    }
    vector<int> buildNeighbor(int config, int n, int m) {
        vector<int> ret;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                ret.push_back(flip(config, i, j, n, m));
            }
        }
        return ret;
    }
    void bfs(vector<int>& dist, int n, int m, int start) {
        queue<int> vis;
        vis.push(start);
        dist[start] = 0;
        while(!vis.empty()) {
            int u = vis.front();
            vis.pop();
            auto neighbor = buildNeighbor(u, n, m);
            for(const auto& v:neighbor) {
                if(dist[v] == -1) {
                    dist[v] = dist[u]+1;
                    vis.push(v);
                }
            }
        }
    }
    void dijkstra(vector<int>& dist, int n, int m, int start) {
        typedef pair<int, int> state;
        priority_queue<state, vector<state>, greater<state>> vis;
        dist[start] = 0;
        vis.emplace(0, start);
        while(!vis.empty()) {
            int cost;
            int u;
            tie(cost, u) = vis.top();
            vis.pop();
            auto neighbor = buildNeighbor(u, n, m);
            for(const auto& v:neighbor) {
                int next = dist[u] + 1;
                if(dist[v] == -1 || dist[v] > next) {
                    vis.emplace(next, v);
                    dist[v] = next;
                }
            }
        }
    }
    int minFlips(vector<vector<int>>& mat) {
        int n = mat.size();
        int m = mat[0].size();
        int maxConfig = 1<<(n*m);
        vector<int> dist(maxConfig, -1);
        int config = 0;
        for(int i = 0;i<n;i++) {
            for(int j = 0;j<m;j++) {
                int k = i*m+j;
                if(mat[i][j]) {
                    config |= 1<<k;
                }
            }
        }
        bfs(dist, n, m, config);
        //dijkstra(dist, n, m, config);
        return dist[0];
    }
};