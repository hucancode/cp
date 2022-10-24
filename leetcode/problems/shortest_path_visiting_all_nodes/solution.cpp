class Solution {
public:
    int bfs(vector<vector<int>>& graph) {
        const int n = graph.size();
        const int PERFECT = (1<<n) - 1;
        if(n<2) {
            return 0;
        }
        int ret = 1e9;
        queue<tuple<int, int, int>> q;
        set<pair<int, int>> visited;
        for(int i = 0;i<n;i++) {
            q.emplace(1<<i, 0, i);
            visited.emplace(i, 1<<i);
        }        
        while(!q.empty()) {
            int u, len, mask;
            tie(mask, len, u) = q.front();
            q.pop();
            if(mask == PERFECT) {
                ret = min(ret, len);
                if(ret == n-1) {
                    break;
                }
                continue;
            }
            for(auto v: graph[u]) {
                int nextMask = mask | 1<<v;
                auto config = make_pair(v, nextMask);
                if(visited.find(config) != visited.end()) {
                    continue;
                }
                visited.insert(config);
                int nextLen = len+1;
                q.emplace(nextMask, nextLen, v);
            }
        }
        return ret;
    }
    int shortestPathLength(vector<vector<int>>& graph) {
        return bfs(graph);
    }
};