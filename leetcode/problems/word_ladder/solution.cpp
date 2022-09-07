class Solution {
public:
    bool adj(string a, string b) {
        bool diff = false;
        for(int i = 0;i<a.size();i++) {
            if(a[i] == b[i]) {
                continue;
            }
            if(diff) {
                return false;
            }
            diff = true;
        }
        return true;
    }
    int ladderLength(string beginWord, string endWord, vector<string>& a) {
        const int INF = 9999999;
        // edge cases
        auto begin = distance(a.begin(), find(a.begin(), a.end(), beginWord));
        auto end = distance(a.begin(), find(a.begin(), a.end(), endWord));
        if(end >= a.size()) {
            return 0;
        }
        if(begin >= a.size()) {
            a.push_back(beginWord);
            begin = a.size() - 1;
        }
        // setup
        int n = a.size();
        vector<int> vis(n);
        for(int i = 0;i<n;i++) {
            vis[i] = i;
        }
        vector<int> dist(n, INF);
        dist[begin] = 0;
        // dijkstra
        while(!vis.empty()) {
            int umin = 0;
            int u = vis[umin];
            int du = dist[u];
            for(int i = 1;i<vis.size();i++) {
                int v = vis[i];
                int dv = dist[v];
                if(dv < du) {
                    umin = i;
                    u = v;
                    du = dv;
                }
            }
            if(u == end) {
                break;
            }
            vis.erase(vis.begin() + umin);
            for(const auto& v: vis) {
                bool hasEdge = adj(a[u], a[v]);
                if(hasEdge) {
                    dist[v] = min(dist[v], dist[u] + 1);
                }
            }
        }
        int ret = dist[end] + 1;
        if(ret >= INF) {
            return 0;
        }
        return ret;
    }
};