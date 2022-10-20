typedef vector<double> vd;
typedef vector<vd> vvd;
typedef pair<double, int> State;
class Solution {
public:
    double maxProbability(int n, vector<vector<int>>& edges, vector<double>& succProb, int start, int end) {
        map<int, vector<State>> adj;
        for(int i = 0;i<edges.size();i++) {
            int a = edges[i][0];
            int b = edges[i][1];
            double w = succProb[i];
            adj[a].emplace_back(w, b);
            adj[b].emplace_back(w, a);
        }
        priority_queue<State> vis;
        vd dist(n, 0.0);
        dist[start] = 1.0;
        vis.emplace(1.0, start);
        while(!vis.empty()) {
            double uw;
            int u;
            tie(uw, u) = vis.top();
            vis.pop();
            for(auto x: adj[u]) {
                double w;
                int v;
                tie(w, v) = x;
                double next = dist[u]*w;
                if(next > dist[v]) {
                    vis.emplace(next, v);
                    dist[v] = next;
                }
            }
        }
        return dist[end];
    }
};