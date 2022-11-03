class Solution {
public:
    vector<int> findAllPeople(int n, vector<vector<int>>& meetings, int firstPerson) {
        typedef pair<int, int> pi;
        map<int, vector<pi>> edges;
        edges[0].emplace_back(firstPerson, 0);
        for(auto x: meetings) {
            edges[x[0]].emplace_back(x[1], x[2]);
            edges[x[1]].emplace_back(x[0], x[2]);
        }
        queue<pi> q;
        vector<int> hasSecret(n,-1);
        q.emplace(0, 0);
        while(!q.empty()) {
            int u, time;
            tie(u, time) = q.front();
            q.pop();
            if(hasSecret[u] != -1 && hasSecret[u] <= time) {
                continue;
            }
            hasSecret[u] = time;
            for(auto edge: edges[u]) {
                int v, nextTime;
                tie(v, nextTime) = edge;
                if(nextTime < time) {
                    continue;
                }
                q.emplace(v, nextTime);
            }
        }
        vector<int> ret;
        for(int i = 0;i<n;i++) {
            if(hasSecret[i] != -1) {
                ret.push_back(i);
            }
        }
        return ret;
    }
};